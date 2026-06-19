use anyhow::{Context, Result};
use windows::Win32::Foundation::HANDLE;
use windows::Win32::System::Registry::*;
use windows::Win32::Foundation::*;
use std::ffi::OsStr;
use std::os::windows::ffi::OsStrExt;

/// Sets GPU priority for Games task to high performance values via direct Win32 registry API.
/// Replaces the previous PowerShell implementation.
///
/// # Change-points
/// - No more reliance on `powershell.exe`; uses only Win32 registry functions.
/// - Proper error handling: returns `Err` if admin rights are missing or registry access fails.
/// - Type-safe: values are written as the correct registry types (DWORD for numeric, REG_SZ for strings).
/// - All registry handles are guaranteed to be closed even on early return (via scope).
///
/// # Safety
/// This function is unsafe only insofar as it calls Win32 API; all pointers are validated
/// and buffers are correctly sized. The function itself is safe to call from safe Rust.
pub fn set_gpu_priority_high() -> Result<()> {
    // Base path under HKLM
    const BASE_PATH: &str = r"SOFTWARE\Microsoft\Windows NT\CurrentVersion\Multimedia\SystemProfile\Tasks\Games";

    // Convert wide string for registry functions
    let base_wide: Vec<u16> = OsStr::new(BASE_PATH).encode_wide().chain(std::iter::once(0)).collect();

    // Open or create the key with write access
    let mut hkey: HANDLE = std::ptr::null_mut();
    let mut disposition: u32 = 0;
    let rc = unsafe {
        RegCreateKeyExW(
            HKEY_LOCAL_MACHINE,
            base_wide.as_ptr(),
            0,
            std::ptr::null_mut(),
            REG_OPTION_NON_VOLATILE,
            KEY_SET_VALUE | KEY_WOW64_64KEY, // ensure we access 64-bit view
            std::ptr::null_mut(),
            &mut hkey,
            &mut disposition,
        )
    };
    if rc != ERROR_SUCCESS {
        return Err(anyhow::anyhow!(
            "Failed to create/open registry key {}: error {}",
            BASE_PATH,
            rc
        )
        .context("Insufficient privileges? Run as administrator."));
    }

    // Ensure the key is closed when we exit the scope
    let _guard = ScopeGuard { hkey };

    // Helper to set a DWORD value
    let set_dword = |name: &str, value: u32| -> Result<()> {
        let name_w: Vec<u16> = OsStr::new(name).encode_wide().chain(std::iter::once(0)).collect();
        let rc = unsafe {
            RegSetValueExW(
                hkey,
                name_w.as_ptr(),
                0,
                REG_DWORD,
                (&value as *const u32 as *const BYTE),
                std::mem::size_of::<u32>() as u32,
            )
        };
        if rc != ERROR_SUCCESS {
            Err(anyhow::anyhow!(
                "Failed to set DWORD {}={}: error {}",
                name,
                value,
                rc
            ))
        } else {
            Ok(())
        }
    };

    // Helper to set a string (REG_SZ) value
    let set_sz = |name: &str, value: &str| -> Result<()> {
        let name_w: Vec<u16> = OsStr::new(name).encode_wide().chain(std::iter::once(0)).collect();
        let value_w: Vec<u16> = OsStr::new(value).encode_wide().chain(std::iter::once(0)).collect();
        let rc = unsafe {
            RegSetValueExW(
                hkey,
                name_w.as_ptr(),
                0,
                REG_SZ,
                (value_w.as_ptr() as *const BYTE),
                ((value_w.len() * std::mem::size_of::<u16>()) as u32),
            )
        };
        if rc != ERROR_SUCCESS {
            Err(anyhow::anyhow!(
                "Failed to set STRING {}={}: error {}",
                name,
                value,
                rc
            ))
        } else {
            Ok(())
        }
    };

    // Set the four values
    set_dword("GPU Priority", 8)?;
    set_dword("Priority", 6)?;
    set_sz("Scheduling Category", "High")?;
    set_sz("SFIO Priority", "High")?;

    Ok(())
}

/// Simple RAII guard to close a registry handle on drop.
struct ScopeGuard {
    hkey: HANDLE,
}
impl Drop for ScopeGuard {
    fn drop(&mut self) {
        if !self.hkey.is_null() {
            unsafe {
                let _ = RegCloseKey(self.hkey);
            }
        }
    }
}