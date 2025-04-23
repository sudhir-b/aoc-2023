# Advent of Code 2023 - Solutions en Rust

Ce dépôt contient des implémentations en Rust pour les puzzles de programmation [Advent of Code 2023](https://adventofcode.com/2023).

## Structure du Dépôt

Ce projet est organisé comme un espace de travail Rust avec plusieurs crates, chacun représentant une solution pour le défi d'un jour spécifique :

```
aoc-2023/
├── day1/        # Solution pour le puzzle du Jour 1
├── day2/        # Solution pour le puzzle du Jour 2
...
└── day12/       # Solution pour le puzzle du Jour 12
```

Chaque répertoire journalier suit la même structure :
- `src/main.rs` - Contient le code de solution avec les fonctions `part1()` et `part2()`
- `src/input.txt` - Contient les données d'entrée du puzzle
- `Cargo.toml` - Configuration du package pour le jour spécifique

## Comment Exécuter

Pour exécuter la solution d'un jour spécifique :

```bash
# Naviguer vers la racine du dépôt
cd aoc-2023

# Exécuter la solution d'un jour spécifique
cargo run -p day1  # Remplacer "day1" par le jour que vous souhaitez exécuter
```

Pour exécuter toutes les solutions :

```bash
cargo run --workspace
```

## Détails d'Implémentation

Chaque solution suit le même modèle :
1. Lire l'entrée du puzzle à partir du fichier `input.txt` en utilisant la macro `include_str!` de Rust
2. Implémenter deux parties du puzzle quotidien (fonctions `part1` et `part2`)
3. Traiter l'entrée selon les exigences spécifiques du puzzle
4. Afficher les résultats dans la console

## Technologies Utilisées

- Langage de programmation Rust (édition 2021)
- Cargo pour la gestion de projet
- Pas de dépendances externes dans les solutions individuelles

## Traductions

- [Anglais (English)](README.md)
- [Espagnol (Español)](README.es.md)