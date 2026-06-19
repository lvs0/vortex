# Vortex — Brand Style Guide

> *Dernière mise à jour : 2026-06-13*

---

## Identity

**Nom** : Vortex
**Tagline** : *L'OS t'a oublié. On te rend la main.*
**Domaine** : Optimisation PC native, open-source, Rust
**Licence** : MIT
**GitHub** : github.com/lvs0/vortex
**Site** : lvs0.github.io/vortex

---

## Écosystème

```
Vortex → écosystème NYX
├── Vortex      (optimiseur PC)
├── Symbiote    (simulateur 3D)
├── Aegis       (cloud chiffré PQ — Polygone-based)
├── Nyx-IA      (assistant local)
└── Nyx-Browser (navigateur concept)
```

Tous les 5 produits partagent le système de tokens NYX.

---

## Palette de couleurs

| Rôle | Couleur | Hex | Usage |
|------|---------|-----|-------|
| Primary | Vortex indigo | `#5B4AE6` | Logo, boutons CTA, accents |
| Secondary | Deep navy | `#0D1117` | Background, contrast |
| Accent | Cyan plasma | `#00D9FF` | Effets lumineux, highlights |
| Surface | Dark surface | `#161B22` | Cards, surfaces élevées |
| Text primary | Near white | `#E6EDF3` | Titres, corps |
| Text secondary | Muted | `#8B949E` | Descriptions, captions |
| Success | Neon vert | `#3FB950` | Status OK, restore OK |
| Warning | Amber | `#D29922` | Warnings, prudence |
| Error | Red | `#F85149` | Erreurs, restore critique |

---

## Typographie

- **Headings** : `Inter` (700, 800) ou `JetBrains Mono` pour les éléments techniques
- **Body** : `Inter` (400, 500)
- **Code/CLI** : `JetBrains Mono` (400, 500)

Pas de typographie décorative — Vortex est sobre, technique, efficace.

---

## Tone of Voice

| Contexte | Ton |
|----------|-----|
| README | Technique mais accessible — on explique pas baby |
| FAQ | Pratique, direct, réponses courtes |
| Telegram / réseau | Brut, minimal, emoji rare |
| Branding | Sobre, puissant, pas de blabla marketing |

**Règles** :
- Jamais "faites ci", toujours "fais ça"
- Pas de promesses exagérées ("le meilleur optimizeur !")
- Expliquer le "pourquoi" pas juste le "quoi"

---

## Logo — Spec

**Forme** : Spirale/tourbillon stylisée (représente le "vortex" du nom)
**Couleurs** : Indigo primary `#5B4AE6` + cyan accent `#00D9FF`
**Style** : Minimal, géométrique, pas de gradient complexe
**Formats à produire** : SVG (source) + PNG 512px + ICO (Windows)

**À produire** : Levy doit designer ou commander le logo final. Cette spec donne la direction.

---

## Assets

- [x] Logo Vortex SVG source — `assets/logo.svg` (spirale indigo/cyan, 256×256 viewBox)
- [x] Logo Vortex PNG 512px — `assets/logo-512.png` (fond transparent)
- [x] Favicon 32×32 — `assets/favicon-32.png` (fond dark navy pour contraste Windows)
- [ ] Build .exe Windows — dépend de la machine cible
- [ ] Banner GitHub README header (optionnel)

---

## README Structure

Le README.md actuel suit cette structure :
1. TL;DR — ce qu'il fait (bullet points)
2. Quickstart — 4 étapes
3. Utilisation — commandes
4. Distribution — plateformes
5. Architecture — schéma
6. Modules

Cette structure fonctionne — ne pas surcharger.

---

## Emojis authorized (usage limité)

| Emoji | Usage |
|-------|-------|
| ⚡ | Power, performance |
| 🎮 | Gaming |
| 💾 | RAM, mémoire |
| 🧹 | Cleanup |
| 🌐 | Network |
| 🚫 | Désactivation |

Pas d'emoji dans les titres ou le nom du projet. Usage fonctionnel uniquement.

---

*Auteurs : Lévy (vision) + Zoe (implémentation) — MIT 2026*