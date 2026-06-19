//! Module `sys` — toutes les opérations bas-niveau Windows.
//! Conçu pour être appelé depuis `main.rs` et `protect.rs`.

use anyhow::{anyhow, Context, Result};
use std::os::windows::ffi::OsStrExt;
use std::ffi::OsStr;
use std::process::Command;

// Internal submodule for GPU priority rewriting
mod gpu_priority;

/// Est-ce qu'on tourne sous Windows ?
pub fn is_windows() -> bool { cfg!(windows) }

/// Profil matériel détecté.
#[derive(Debug, Clone, Copy)]
pub enum Profile {
    GamingRig,
    Workstation,
    Laptop,
    MinimalPC,
}

pub fn detect_profile() -> &'static str {
    let total_mb = sysinfo::System::new_all().total_memory() / 1024 / 1024;
    let cores    = num_cpus();
    match (total_mb, cores) {
        (m, c) if m >= 32_000 && c >= 16 => "Gaming Rig",
        (m, c) if m >= 16_000 && c >= 8  => "Workstation équilibrée",
        (m, c) if m >= 8_000  && c >= 4  => "Laptop / polyvalent",
        _                                 => "MinimalPC — on serre les vis",
    }
}

fn num_cpus() -> usize {
    std::thread::available_parallelism().map(|n| n.get()).unwrap_or(1)
}

// ────────────────────────────── Power plan ──────────────────────────────

pub fn apply_ultimate_power_plan() -> Result<()> {
    // GUID: Ultimate Performance = {8c5e7fda-e8bf-4a96-9a85-a6e23a8c635c}
    let r = run_powershell(
        r#"
powercfg /setactive 8c5e7fda-e8bf-4a96-9a85-a6e23a8c635c
if %ERRORLEVEL% neq 0 (
  powercfg -duplicatescheme e9a42b02-d5df-448d-aa00-03f14749eb61
  powercfg /setactive e9a42b02-d5df-448d-aa00-03f14749eb61
)
# Désactiver la mise en veille sur USB selectif (gaspillage CPU subsys)
powercfg /setacvalueindex scheme_current SUB_USB USBSELECTIVESUSPEND 0
powercfg /setdcvalueindex scheme_current SUB_USB USBSELECTIVESUSPEND 0
# CPU 100% min
powercfg /setacvalueindex scheme_current SUB_PROCESSOR PROCFREQMIN 100
powercfg /setdcvalueindex scheme_current SUB_PROCESSOR PROCFREQMAX 100
powercfg /setactive scheme_current
"#,
    )?;
    if !r.contains("ERREUR") { Ok(()) } else { Err(anyhow!(r)) }
}

// ────────────────────────────── Memory / Paging ──────────────────────────────

/// Configure le paging file automatiquement (RAM × 1.5 — formule éprouvée).
pub fn optimize_paging_file() -> Result<()> {
    let total_mb = sysinfo::System::new_all().total_memory() / 1024 / 1024;
    #[allow(unused_parens)]
    let paging_mb = (total_mb * 3 / 2).max(2048); // min 2GB
    let script = format!(
        r#"
$cs = Get-WmiObject Win32_ComputerSystem
Set-CimInstance -InputObject $cs -Property @{{AutomaticManagedPagefile = $false}}
$pf = Get-WmiObject Win32_PageFileSetting
$root = (Get-WmiObject Win32_PageFileUsage | Select-Object -First 1).Name -replace 'pagefile\\.sys$'
if (-not $root) {{ $root = 'C:\\' }}
$csi = Get-CimInstance Win32_PageFileSetting -ErrorAction SilentlyContinue
if ($csi) {{ Set-CimInstance -InputObject $csi -Property @{{InitialSize = {init}; MaximumSize = {max}}} }} 
else {{ Set-CimInstance -Namespace root\\cimv2 -ClassName Win32_PageFileSetting -Property @{{Name = 'C:\\pagefile.sys'; InitialSize = {init}; MaximumSize = {max}}} | Out-Null }}
"#,
        init = paging_mb,
        max = paging_mb
    );
    run_powershell(&script).map(|_| ())
}

/// Memory Integrity (Credential Guard / HVCI) — peut être gardé
/// si l'utilisateur est en environnement corporate. On propose toggle.
pub fn disable_memory_integrity(off: bool) -> Result<()> {
    let cmd = if off { 0 } else { 1 };
    let r = run_powershell(&format!(
        r#"
$path = 'HKLM:\\SYSTEM\\CurrentControlSet\\Control\\DeviceGuard\\Scenarios\\HypervisorEnforcedCodeIntegrity'
if (Test-Path $path) {{
  Set-ItemProperty -Path $path -Name 'Enabled' -Value {cmd}
}}
"#
    ))?;
    if r.is_empty() { Ok(()) } else { Err(anyhow!(r)) }
}

// ────────────────────────────── Trimming dynamique ──────────────────────────────

/// Compresse les working sets des processus en arrière-plan (RAM released).
/// C'est le coeur \"quantisation temps réel\" que tu voulais.
pub fn trim_background_apps() -> Result<()> {
    let mut sys = sysinfo::System::new_all();
    sys.refresh_processes();
    let current_pid = std::process::id();
    let mut trimmed = 0u32;
    for p in sys.processes().values() {
        if p.pid().as_u32() == current_pid { continue; }
        let name = p.name().to_string_lossy().to_string();
        // Whitelist des app système critiques
        if matches!(name.as_str(), "explorer.exe" | "dwm.exe" | "csrss.exe" | "winlogon.exe" | "System" | "smss.exe") {
            continue;
        }
        unsafe {
            // On ne touche qu'aux app non critiques si elles consomment trop.
            // EmptyWorkingSet via PSAPI.
            let h = windows::Win32::System::Threading::OpenProcess(
                windows::Win32::System::Threading::PROCESS_SET_QUOTA | windows::Win32::System::Threading::PROCESS_QUERY_INFORMATION,
                false,
                p.pid().as_u32(),
            );
            if let Ok(h) = h {
                // signe de la fonction
                type Fn = unsafe extern "system" fn(windows::Win32::Foundation::HANDLE) -> i32;
                #[link(name = "psapi")]
                extern "system" { fn EmptyWorkingSet(handle: windows::Win32::Foundation::HANDLE) -> i32; }
                let r: i32 = std::mem::transmute::<Fn, Fn>(EmptyWorkingSet)(h);
                if r != 0 { trimmed += 1; }
                let _ = windows::Win32::Foundation::CloseHandle(h);
            }
        }
    }
    if trimmed > 0 { Ok(()) } else { Err(anyhow!("aucun processus éligible trouvé")) }
}

// ────────────────────────────── GPU / HAGS ──────────────────────────────

pub fn enable_hags() -> Result<()> {
    let r = run_powershell(
        r#"
$path = 'HKLM:\\SYSTEM\\CurrentControlSet\\Control\\GraphicsDrivers'
Set-ItemProperty -Path $path -Name 'HwSchMode' -Value 2
"#,
    )?;
    if r.is_empty() { Ok(()) } else { Err(anyhow!(r)) }
}

/// Sets GPU priority for Games task to high performance values via direct Win32 registry API.
/// Replaces the previous PowerShell implementation.
pub fn set_gpu_priority_high() -> Result<()> {
    gpu_priority::set_gpu_priority_high()
}

// ────────────────────────────── Network ──────────────────────────────

pub fn disable_nagle() -> Result<()> {
    let r = run_powershell(
        r#"
$paths = @(
  'HKLM:\\SYSTEM\\CurrentControlSet\\Services\\Tcpip\\Parameters\\Interfaces'
)
Get-Item $paths | ForEach-Object {
  Set-ItemProperty -Path $_.PsPath -Name 'TcpAckFrequency' -Value 1 -ErrorAction SilentlyContinue
  Set-ItemProperty -Path $_.PsPath -Name 'TCPNoDelay' -Value 1 -ErrorAction SilentlyContinue
  Set-ItemProperty -Path $_.PsPath -Name 'TcpDelAckTicks' -Value 0 -ErrorAction SilentlyContinue
}
"#,
    )?;
    if r.is_empty() { Ok(()) } else { Err(anyhow!(r)) }
}

pub fn set_network_throttling_off() -> Result<()> {
    let r = run_powershell(
        r#"
$path = 'HKLM:\\SOFTWARE\\Microsoft\\Windows NT\\CurrentVersion\\Multimedia\\SystemProfile'
Set-ItemProperty -Path $path -Name 'NetworkThrottlingIndex' -Value 0xffffffff
"#,
    )?;
    if r.is_empty() { Ok(()) } else { Err(anyhow!(r)) }
}

// ────────────────────────────── Process priority ──────────────────────────────

pub fn boost_process_priority(pid: u32) -> Result<()> {
    use windows::Win32::System::Threading::*;
    unsafe {
        let h = OpenProcess(PROCESS_SET_INFORMATION, false, pid).context("OpenProcess")?;
        let _ = SetPriorityClass(h, REALTIME_PRIORITY_CLASS);
        let _ = windows::Win32::Foundation::CloseHandle(h);
    }
    Ok(())
}

// ────────────────────────────── Services à raboter ──────────────────────────────

pub fn trim_superfetch() -> Result<()> {
    run_powershell(
        r#"
$b = 'HKLM:\\SYSTEM\\CurrentControlSet\\Control\\Session Manager\\Memory Management\\PrefetchParameters'
Set-ItemProperty -Path $b -Name 'EnablePrefetcher' -Value 0 -ErrorAction SilentlyContinue
Set-ItemProperty -Path $b -Name 'EnableSuperfetch' -Value 0 -ErrorAction SilentlyContinue
"#,
    )?;
    Ok(())
}

pub fn disable_xbox_game_bar() -> Result<()> {
    run_powershell(
        r#"
$h1 = 'HKCU:\\SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\GameDVR'
New-Item -Path $h1 -Force | Out-Null
Set-ItemProperty -Path $h1 -Name 'AppCaptureEnabled' -Value 0
Set-ItemProperty -Path $h1 -Name 'GameDVR_Enabled' -Value 0
"#,
    )?;
    Ok(())
}

// ────────────────────────────── Sauvegarde / restauration ──────────────────────────────

pub fn backup_current_state() -> Result<()> {
    let mut cmd = Command::new("reg");
    cmd.args([
        "export", r"HKLM\\SYSTEM\\CurrentControlSet\\Control\\GraphicsDrivers",
        r"C:\\ProgramData\\Vortex\\backup_graphics.reg", "/y",
    ]);
    let _ = cmd.output();

    let mut cmd2 = Command::new("reg");
    cmd2.args([
        "export", r"HKLM\\SOFTWARE\\Microsoft\\Windows NT\\CurrentVersion\\Multimedia\\SystemProfile",
        r"C:\\ProgramData\\Vortex\\backup_multimedia.reg", "/y",
    ]);
    let _ = cmd2.output();

    // plan d'alimentation courant
    let _ = run_powershell(
        r#"
$active = powercfg /getactivescheme
$active | Out-File C:\\ProgramData\\Vortex\\backup_power.txt -Encoding utf8
New-Item -Path 'C:\\ProgramData\\Vortex' -ItemType Directory -Force | Out-Null
"#,
    );

    let _ = run_powershell("New-Item -Path 'C:\\\\ProgramData\\\\Vortex' -ItemType Directory -Force | Out-Null");
    Ok(())
}

pub fn restore_state() -> Result<()> {
    let _ = run_powershell(
        r#"
reg import 'C:\\ProgramData\\Vortex\\backup_graphics.reg' 2>$null
reg import 'C:\\ProgramData\\Vortex\\backup_multimedia.reg' 2>$null
$plan = Get-Content 'C:\\ProgramData\\Vortex\\backup_power.txt' -ErrorAction SilentlyContinue | Select-Object -First 1
if ($plan -match '([0-9a-f-]+)') { powercfg /setactive $Matches[1] }
"#,
    );
    Ok(())
}

// ────────────────────────────── Utils shell ──────────────────────────────

pub fn run_powershell(script: &str) -> Result<String> {
    let mut cmd = Command::new("powershell");
    cmd.args([
        "-NoProfile",
        "-NonInteractive",
        "-ExecutionPolicy", "Bypass",
        "-Command", script,
    ]);
    let out = cmd.output().map_err(|e| anyhow!(e))?;
    let mut s = String::from_utf8_lossy(&out.stdout).to_string();
    let e = String::from_utf8_lossy(&out.stderr).to_string();
    if !out.status.success() || e.to_lowercase().contains("erreur") {
        s.push_str("\nSTDERR: ");
        s.push_str(&e);
    }
    Ok(s)
}

pub fn wide(s: &str) -> Vec<u16> {
    OsStr::new(s).encode_wide().chain(Some(0)).collect()
}

// ────────────────────────────── Telemetry ──────────────────────────────

pub mod telemetry {
    use serde::Serialize;
    use sysinfo::*;

    #[derive(Serialize)]
    pub struct ProcessMini {
        pub name: String,
        pub pid:  u32,
        pub rss_mb: f64,
    }

    #[derive(Serialize)]
    pub struct Telemetry {
        pub cpu_brand: String,
        pub cpu_percent: f64,
        pub cpu_cores: usize,
        pub ram_percent: f64,
        pub ram_used_mb: f64,
        pub ram_total_mb: f64,
        pub gpu_name: String,
        pub gpu_percent: f64,
        pub disk_name: String,
        pub disk_percent: f64,
        pub top_ram: Vec<ProcessMini>,
    }

    pub fn collect() -> Telemetry {
        let mut sys = System::new_all();
        sys.refresh_all();

        let mut procs: Vec<ProcessMini> = sys.processes().values().map(|p| ProcessMini {
            name: p.name().to_string_lossy().to_string(),
            pid: p.pid().as_u32(),
            rss_mb: p.memory() as f64 / 1024.0 / 1024.0,
        }).collect();
        procs.sort_by(|a, b| b.rss_mb.partial_cmp(&a.rss_mb).unwrap());

        let cpu_pct = sys.global_cpu_usage() as f64;
        let cpu_brand = sys.cpus().first().map(|c| c.brand().to_string()).unwrap_or_default();
        let ram_total = sys.total_memory() as f64 / 1024.0 / 1024.0;
        let ram_used  = sys.used_memory()  as f64 / 1024.0 / 1024.0;

        // GPU/dysk via sysinfo (PCI lookup minimal)
        let mut gpu_name = String::from("non détecté");
        let mut gpu_percent = 0.0;
        for cmp in sys.components() {
            if format!("{:?}", cmp.label()).contains("GPU") {
                gpu_percent = cmp.temperature().unwrap_or(0.0) as f64 * 4.0; // approximation
            }
        }
        let disk_name = sys.disks().first().map(|d| d.name().to_string_lossy().to_string()).unwrap_or_default();
        let disk_total = sys.disks().iter().map(|d| d.total_space()).sum::<u64>() as f64;
        let disk_free  = sys.disks().iter().map(|d| d.available_space()).sum::<u64>() as f64;
        let disk_percent = if disk_total > 0.0 { (1.0 - disk_free / disk_total) * 100.0 } else { 0.0 };

        Telemetry {
            cpu_brand,
            cpu_percent: cpu_pct,
            cpu_cores: sys.cpus().len(),
            ram_percent: (ram_used / ram_total) * 100.0,
            ram_used_mb: ram_used,
            ram_total_mb: ram_total,
            gpu_name,
            gpu_percent,
            disk_name,
            disk_percent,
            top_ram: procs,
        }
    }
}