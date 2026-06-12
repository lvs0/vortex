//! Vortex Protect — daemon léger qui maintient l'optimisation en arrière-plan.

use std::thread;
use std::time::Duration;
use crate::sys;

pub fn run_protect() {
    println!("⚡ Vortex-Protect actif (Ctrl+C pour arrêter).");
    loop {
        let mut s = sysinfo::System::new_all();
        s.refresh_memory();
        let pct = (s.used_memory() as f64 / s.total_memory() as f64) * 100.0;
        if pct > 85.0 {
            println!("[!] RAM {:.1}% — libération...", pct);
            let _ = sys::trim_background_apps();
        } else {
            println!("[..] OK ({:.1}%)", pct);
        }
        thread::sleep(Duration::from_secs(10));
    }
}
