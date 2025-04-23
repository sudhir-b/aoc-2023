# Advent of Code 2023 - Solutions en Rust

Ce dépôt contient mes solutions en Rust pour les défis de programmation [Advent of Code 2023](https://adventofcode.com/2023).

## Structure du Projet

Le dépôt est organisé comme un espace de travail Rust avec plusieurs crates, chacun représentant une solution pour un défi quotidien spécifique :

- `day1` à `day12` : Crates individuels pour chaque puzzle quotidien
- Chaque jour suit la même structure :
  - `src/main.rs` : Contient le code de solution avec les fonctions `part1()` et `part2()`
  - `src/input.txt` : Contient les données d'entrée du puzzle
  - `Cargo.toml` : Configuration du package

## Détails Techniques

- **Langage** : Rust (édition 2021)
- **Système de Build** : Cargo
- **Dépendances** : Aucune dépendance externe dans les crates de solution

## Comment Exécuter

Vous pouvez exécuter la solution de n'importe quel jour en utilisant Cargo :

```bash
# Exécuter un jour spécifique (par exemple, day1)
cargo run -p day1

# Exécuter avec des optimisations de version
cargo run --release -p day1
```

## Solutions Implémentées

- Jour 1 : Trebuchet?!
- Jour 2 : Le Casse-tête des Cubes
- Jour 3 : Ratios d'Engrenages
- Jour 4 : Cartes à Gratter
- Jour 5 : Si Vous Donnez une Graine à un Fertilisant
- Jour 6 : Attendez un Peu
- Jour 7 : Cartes de Chameau
- Jour 8 : Terres Désolées Hantées
- Jour 9 : Maintenance du Mirage
- Jour 10 : Labyrinthe de Tuyaux
- Jour 11 : Expansion Cosmique
- Jour 12 : Sources Chaudes

## À Propos d'Advent of Code

Advent of Code est un ensemble annuel de puzzles de programmation sur le thème de Noël qui peuvent être résolus dans n'importe quel langage de programmation. Chaque puzzle se compose de deux parties, la seconde s'appuyant sur la première.

Pour plus d'informations, visitez [adventofcode.com](https://adventofcode.com/).