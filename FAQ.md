# Vortex — FAQ

*Dernière mise à jour : 2026-06-13*

---

## Questions Rapides

**Q : Mon PC ne提速 plus après apply.**
**R :** Lance `vortex.exe restore` pour revenir à l'état d'avant. Le backup est dans `C:\ProgramData\Vortex\`.

**Q : J'ai un Dell avec 16 Go RAM — c'est compatible ?**
**R :** Oui. Vortex a été conçu pour le Dell de 16 Go. Le paging file est configuré à RAM×1.5 (24 Go).

**Q : Est-ce que ça void ma garantie ?**
**R :** Non. Vortex ne touche pas au firmware. Restore réinitialise tout via le registre et powercfg.

**Q : Mon antivirus le bloque.**
**R :** Ajoute une exception pour `C:\ProgramData\Vortex\` et `%LOCALAPPDATA%\vortex\`. Rust compilé = pas d'installateur tierce, méfiance normale.

**Q : Est-ce que ça marche sur Windows 10 ?**
**R :** Testé sur Windows 11. Windows 10 : certaines optimisations (HAGS) ne sont pas disponibles. Apply warn mais restore fonctionne.

**Q : J'ai appliqué, rien ne se passe.**
**R :** Redémarre après apply. Certaines optimisations (power plan, services) nécessitent un redémarrage.

---

## Comment ça marche — en résumé

Vortex applique 7 optimisations à Windows 11 :

| # | Optimisation | Ce que ça fait |
|---|-------------|----------------|
| 1 | Ultimate Performance power plan | Schéma haute performance du panel Nvidia/AMD |
| 2 | HAGS activé | Hardware-Accelerated GPU Scheduling — GPU scheduler natif |
| 3 | GPU priority HIGH | Catégorie "Games" en priorité haute pour le GPU |
| 4 | Paging file RAM×1.5 fixe | 24 Go sur Dell 16 Go — plus de paging erratic |
| 5 | Working-set trimming |进程 en arrière-plan rendus à la RAM système |
| 6 | Nagle off + network throttling | Latence réseau réduite pour gaming |
| 7 | Xbox Game Bar + Superfetch off | Services superflus désactivés, RAM libérée |

Tout est backuppé AVANT modification dans `C:\ProgramData\Vortex\`.

---

## Comment lire le dashboard (`dash`)

```cmd
vortex.exe dash
```

Affiche en temps réel :
- RAM libre / totale
- Load CPU
- Power plan actif
- Processes qui bouffent le plus
- Status HAGS

CTRL+C pour arrêter.

---

## Comment protéger les gains (`protect`)

```cmd
vortex.exe protect
```

Daemon léger qui re-trim les working sets toutes les 30s. Pour garder les gains sur des sessions longues.

---

## Désinstaller / Reset complet

```cmd
vortex.exe restore
```
Puis supprime le dossier `C:\ProgramData\Vortex\` si tu veux nettoyer.

Il n'y a pas de désinstallateur NSIS à lancé — tout est revert.

---

## Besoin d'aide ?

- Issues : https://github.com/lvs0/vortex/issues
- Code : https://github.com/lvs0/vortex
- Écosystème NYX : https://lvs0.github.io/nyx/