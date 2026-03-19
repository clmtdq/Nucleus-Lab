# 🚀 AtomGame

Un petite projet sympa pour voir comment les atomes se comportent dans un simulateur graphique. Développé en **Rust (2024)** avec une bonne grosse dose d'électromagnétisme et de chaos.

## C'est quoi ?

Faut imaginer des trucs minuscules (protons, neutrons, électrons) qui se repoussent pas mal les uns les autres et qui essaient de former des atomes. C'est visuellement rigolo et c'est pas mal pour comprendre comment ça marche un atome sans avoir besoin d'un microscope.

## What's in the box ?

- Des particules qui se repoussent selon les lois de la physique
- Des noyaux qui se forment automatiquement quand c'est le moment
- Du rendu temps réel avec macroquad (on voit vraiment ce qu'il se passe)
- Voilà, c'est sympa

## Stack

- **Rust (2024)** - le langage du projet
- **[macroquad](https://github.com/not-fl3/macroquad) 0.4.14** - pour les jolis graphiques
- **rand 0.8** - pour les nombres un peu aléatoires

## Comment le lancer ?

T'as besoin de Rust, alors installe-le d'abord si c'est pas fait.

```bash
git clone <ton-repo>
cd AtomGame
cargo run --release
```

Boom, c'est lancé.

## Y'a quoi à l'intérieur ?

```
src/
├── main.rs          # Le cœur du truc
Cargo.toml           # La config
```

## License

T'en fais ce que tu veux
