# GPU Priority Module Rewrite

## Overview
This module replaces the previous PowerShell-based implementation of `set_gpu_priority_high` with a pure Win32 API solution using the `windows-rs` crate.

## Change-Points

### 1. Removal of `powershell.exe` Dependency
- **Before**: The function invoked `powershell.exe` to write registry keys via `Set-ItemProperty`.
- **After**: Direct registry manipulation using `RegCreateKeyExW` and `RegSetValueExW` from `windows-rs::Win32::System::Registry`.

### 2. Improved Error Handling
- **Before**: Errors were inferred from PowerShell output strings; missing admin rights produced vague messages.
- **After**: Explicit `Result` return with `anyhow::Context` indicating likely cause (e.g., insufficient privileges). The function returns `Err` if the registry key cannot be opened/created or if any value fails to set.

### 3. Type‑Safety
- **Before**: All values were written as strings; the registry type was inferred by PowerShell.
- **After**: Numeric values (`GPU Priority`, `Priority`) are written as `REG_DWORD`. String values (`Scheduling Category`, `SFIO Priority`) are written as `REG_SZ`. This eliminates ambiguity and matches the expected schema.

### 4. Resource Safety
- **Before**: No explicit handle management; PowerShell handles its own resources.
- **After**: Registry handles are managed via RAII (`ScopeGuard`) guaranteeing `RegCloseKey` is called even on early return.

### 5. Performance
- **Before**: Spawning a `powershell.exe` process for each call incurred process‑creation overhead.
- **After**: Pure in‑process DLL calls; substantially lower latency and no transient console windows.

## Build Requirements
- `windows = { version = "0.58", features = [ "Win32_System_Registry", "Win32_Foundation" ] }` (already present in Cargo.toml)
- No additional dependencies.

## Usage
The function signature remains unchanged:
```rust
pub fn set_gpu_priority_high() -> Result<()>
```
Call it from `sys::set_gpu_priority_high()` as before; the implementation now lives in `engine/src/gpu_priority.rs`.

## Testing on Windows 11
1. Ensure you run the binary with administrator privileges (writing to HKLM requires elevated rights).
2. After a successful call, verify the registry keys:
   ```
   reg query "HKLM\\SOFTWARE\\Microsoft\\Windows NT\\CurrentVersion\\Multimedia\\SystemProfile\\Tasks\\Games"
   ```
   Expected values:
   - `GPU Priority`    0x8
   - `Priority`        0x6
   - `Scheduling Category`    High
   - `SFIO Priority`          High

## Notes
- The module deliberately avoids any use of `powershell.exe`, `cmd.exe`, or other external interpreters.
- All logic is contained within the Rust module, making it easier to audit and sandbox.
- If the key already exists, `RegCreateKeyExW` opens it without altering existing unrelated values.