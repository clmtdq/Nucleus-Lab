# AtomGame

Un simulateur interactif d'atomes développé en **Rust (2024)** qui simule les interactions physiques entre particules subatomiques (protons, neutrons et électrons).

## Description

AtomGame est un projet éducatif qui simule en temps réel le comportement de particules chargées selon les principes de l'électromagnétisme. Les particules se repoussent en fonction de leurs charges, et lorsque des protons et des neutrons se rapprochent suffisamment, ils forment des noyaux atomiques.

## Fonctionnalités

- 🎯 Simulation physique des forces de répulsion électrostatique
- 🧬 Formation automatique de noyaux (protons + neutrons)
- 📊 Rendu graphique en temps réel avec macroquad
- 🎮 Gestion dynamique des particules et atomes

## Technologies

- **Langage** : Rust (2024)
- **Framework graphique** : [macroquad](https://github.com/not-fl3/macroquad) 0.4.14
- **Générateur aléatoire** : rand 0.8

## Installation

Assurez-vous d'avoir [Rust](https://www.rust-lang.org/) installé, puis :

```bash
git clone <votre-repo>
cd AtomGame
cargo build --release
```

## Utilisation

Pour lancer le simulateur :

```bash
cargo run --release
```

## Structure du Projet

```
src/
├── main.rs          # Logique principale du simulateur
Cargo.toml           # Configuration du projet
```

## Licence

Libre d'utilisation
