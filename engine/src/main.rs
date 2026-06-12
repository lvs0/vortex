//! Vortex — Moteur principal
//! Auteur: Zoe pour Lévy
//! Concept: ère symbiotique — exploiter 100% du potentiel matériel caché
//!         via l'architecture logicielle.

#![cfg_attr(windows, windows_subsystem = "windows")]

use std::time::Duration;
use std::thread;
use std::process;

#[cfg(windows)]
mod sys;
#[cfg(windows)]
mod protect;

fn main() {
    #[cfg(not(windows))]
    {
        eprintln!("Vortex doit être compilé et exécuté sous Windows 11.");
        eprintln!("Voir build.cmd dans le dépôt.");
        process::exit(1);
    }

    #[cfg(windows)]
    {
        let args: Vec<String> = std::env::args().collect();
        let mode = args.get(1).map(|s| s.as_str()).unwrap_or("apply");

        match mode {
            "apply"      => run_apply(),
            "restore"    => sys::restore_state().unwrap_or_else(|e| eprintln!("✗ {e}")),
            "info"       => run_info(),
            "protect"    => protect::run_protect(),
            "dash"       => run_dash(),
            "help"|_     => print_help(),
        }
    }
}

fn print_help() {
    println!("Vortex v0.1 — optimise Windows 11 selon ton matériel");
    println!();
    println!("Usage:");
    println!("  vortex              Applique le profil adaptatif (recommandé)");
    println!("  vortex info         Affiche la télémétrie du système");
    println!("  vortex protect      Active le daemon léger de maintenance continue");
    println!("  vortex dash         Lance le tableau de bord console temps réel");
    println!("  vortex restore      Restaure les paramètres d'origine (sauvegardés)");
}

#[cfg(windows)]
fn run_apply() {
    println!("⚡ Vortex — ère symbiotique v0.1");
    println!("Profil détecté: {}", sys::detect_profile());

    if let Err(e) = sys::backup_current_state() { eprintln!("⚠ backup: {e}"); }
    if let Err(e) = sys::enable_hags()          { eprintln!("⚠ HAGS: {e}"); } else { println!("✓ HAGS activé"); }
    if let Err(e) = sys::set_gpu_priority_high() { eprintln!("⚠ GPU prio: {e}"); } else { println!("✓ Priorité GPU élevée"); }
    if let Err(e) = sys::apply_ultimate_power_plan() { eprintln!("⚠ Power: {e}"); } else { println!("✓ Ultimate Performance"); }
    if let Err(e) = sys::optimize_paging_file() { eprintln!("⚠ Paging: {e}"); } else { println!("✓ Paging adapté à ta RAM"); }
    if let Err(e) = sys::trim_background_apps() { eprintln!("⚠ Trim RAM: {e}"); } else { println!("✓ Working sets compressés"); }
    if let Err(e) = sys::disable_nagle() { eprintln!("⚠ Nagle: {e}"); } else { println!("✓ Nagle désactivé"); }
    if let Err(e) = sys::set_network_throttling_off() { eprintln!("⚠ Net throttling: {e}"); } else { println!("✓ Throttling réseau off"); }
    let _ = sys::trim_superfetch();
    let _ = sys::disable_xbox_game_bar();
    if let Err(e) = sys::boost_process_priority(process::id()) { eprintln!("⚠ Vortex prio: {e}"); } else { println!("✓ Vortex en haute priorité"); }

    println!();
    println!("✓ Vortex appliqué. Relance ton jeu pour sentir la différence.");
}

#[cfg(windows)]
fn run_info() {
    let t = sys::telemetry::collect();
    println!("{}", serde_json::to_string_pretty(&t).unwrap());
}

#[cfg(windows)]
fn run_dash() {
    println!("Vortex Dash — 'q' pour quitter.");
    loop {
        let t = sys::telemetry::collect();
        print!("\x1B[2J\x1B[H");
        println!("VORTEX DASH  ·  ZOE/LÉVY  ·  {}", chrono::Local::now().format("%H:%M:%S"));
        println!("═══════════════════════════════════════════════════════");
        println!("CPU    : {:>5.1}%   {} cœurs  {}",   t.cpu_percent, t.cpu_cores, t.cpu_brand);
        println!("RAM    : {:>5.1}%   {:.0} / {:.0} Mo", t.ram_percent, t.ram_used_mb, t.ram_total_mb);
        println!("Disque : {:>5.1}%",                  t.disk_percent);
        println!("───────────────────────────────────────────────────────");
        println!("Top RAM :");
        for p in t.top_ram.iter().take(5) {
            println!("   {:<24} {:>6.0} Mo  pid={}", p.name, p.rss_mb, p.pid);
        }
        thread::sleep(Duration::from_millis(500));
    }
}
