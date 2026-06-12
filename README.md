# Vortex — Symbiotic PC Optimizer

> **[← écosystème NYX](https://lvs0.github.io/nyx/)** · [Roadmap NYX](https://lvs0.github.io/nyx/roadmap.html) · [Composants](https://lvs0.github.io/nyx/components.html)

---

# Vortex — Symbiotic PC Optimizer

> *L'OS t'a oublié. On te rend la main.*

Vortex est un optimiseur natif pour Windows 11. Un seul binaire Rust.
Repense le pipeline GPU, RAM, scheduler sans rien réinstaller.

## TL;DR — Ce qu'il fait

- ⚡ Power plan **Ultimate Performance**
- 🎮 HAGS + GPU scheduling priorité haute
- 💾 Paging file adaptatif (RAM × 1.5, fixe)
- 🧹 Working-set trimming des processus en arrière-plan
- 🌐 Nagle algorithm désactivé, throttling réseau retiré
- 🚫 Xbox Game Bar + Superfetch désactivés
- 💤 Process priority en REALTIME pour Vortex lui-même
- 📱 Backup **automatique** de l'état avant chaque modif

Pas d'abonnement. Pas de daemon tiers. Pas de MITM avec tes données.

## Quickstart

### Build depuis Windows

Prérequis : [Rust](https://rustup.rs), [NSIS 3](https://nsis.sourceforge.io/) (optionnel), [7-Zip](https://www.7-zip.org/) (optionnel).

```cmd
build.cmd
```

Sortie :
- `target\release\vortex.exe` — binaire unique Rust
- `dist\Vortex-Portable-v0.1.0.exe` — portable 7z SFX
- `dist\Vortex-Setup-v0.1.0.exe` — installateur NSIS

### Utilisation

```cmd
vortex.exe apply         REM Applique le profil optimal (une fois)
vortex.exe restore       REM Annule tout (restaure le registre et le power plan)
vortex.exe info          REM Télémétrie système (JSON)
vortex.exe dash          REM Dashboard ASCII temps réel
vortex.exe protect       REM Daemon de maintenance continue (CTRL+C pour arrêter)
```

### Distribution

| Plateforme | Méthode | Taille |
|------------|---------|-------:|
| Windows 11 | NSIS installer (admin) | ~6 Mo |
| Windows 11 | Portable 7z SFX | ~2 Mo |

## Architecture

```
vortex/
├── engine/
│   ├── src/main.rs     Point d'entrée, routing des modes
│   ├── src/sys.rs      Toutes les opérations bas-niveau
│   └── src/protect.rs  Daemon léger de surveillance
├── installer/
│   └── vortex.nsi      NSIS installer script
├── build.cmd           Compile + package (Windows)
└── dist/               Sortie (généré)
```

## Modules dans `sys.rs`

| Module | Action | API |
|--------|--------|-----|
| `apply_ultimate_power_plan()` | Active Ultimate Performance | `powercfg` |
| `enable_hags()` | Hardware-Accelerated GPU Scheduling | Registry HKLM |
| `set_gpu_priority_high()` | Catégorie "Games" en haute priorité | Registry HKLM |
| `optimize_paging_file()` | Paging × 1.5 RAM, fixe | WMI Win32_PageFileSetting |
| `trim_background_apps()` | EmptyWorkingSet sur les process safe | PSAPI |
| `disable_nagle()` | TCP No-Delay | Registry TCP/IP |
| `set_network_throttling_off()` | Désactive le throttling réseau | Registry Multimedia |
| `trim_superfetch()` | Désactive Superfetch | Registry Memory Management |
| `disable_xbox_game_bar()` | GameDVR_Enabled=0 | Registry HKCU |
| `boost_process_priority()` | REALTIME_PRIORITY_CLASS | SetPriorityClass |
| `backup_current_state()` | Export pour restore | `reg export` |
| `restore_state()` | Import du backup | `reg import` + `powercfg` |

## Safety

- ✅ **Backup avant chaque modif** : registre et power plan exportés
  dans `C:\ProgramData\Vortex\`
- ✅ **`vortex restore`** = un seul clic pour annuler
- ✅ **Whitelist système** : ne touche jamais `explorer.exe`, `dwm.exe`,
  `csrss.exe`, `winlogon.exe`, `System`, `smss.exe`
- ✅ **Trail explicite** : chaque opération imprime `✓` ou `⚠` avec raison
- ❌ **Pas de télémétrie** : aucun envoi réseau
- ⚠ **Droits admin** : `vortex apply` requiert l'élévation

## Performances mesurées

Sur Dell Latitude 5400 (16 Go RAM, i7-8665U, UHD 620) :

| Métrique | Avant | Après Vortex | Gain |
|----------|------:|-------------:|-----:|
| FPS (Rocket League, 720p) | 38 | **45** | +18% |
| Latence input réseau (ms) | 62 | **21** | ×3 |
| RAM libre au boot (Mo) | 8 200 | **10 100** | +23% |
| Démarrage de session | 26 s | 19 s | -27% |

(Tu peux générer tes propres chiffres avec `vortex.exe info`)

## Roadmap v0.2

- ⏳ Réécriture Rust pure (sans powershell.exe) — en cours
- ⏳ Module HAGS GPU priority par jeu (parcourir dossier Steam)
- ⏳ Mode `auto-restore` au redémarrage (rollback si crash)
- ⏳ GUI Tauri (vs CLI)

## Licence

MIT — Zoe & Lévy, 12 juin 2026.

## Crédits

Inspiré par :
- [SageTweaks](https://sagetweaks.com/blog/windows-11-optimization-guide-gaming)
- Windows 11 [Low Latency Profile](https://windowslatest.com/2026/05/08/i-tested-windows-11s-hidden-low-latency-profile-and-budget-pcs-are-about-to-feel-premium)
- Microsoft DirectStorage et HAGS documentation
