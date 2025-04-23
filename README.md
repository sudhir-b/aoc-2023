# Advent of Code 2023 - Rust Solutions

This repository contains Rust implementations for the [Advent of Code 2023](https://adventofcode.com/2023) programming puzzles.

## Repository Structure

This project is organized as a Rust workspace with multiple crates, each representing a solution for a specific day's challenge:

```
aoc-2023/
├── day1/        # Solution for Day 1 puzzle
├── day2/        # Solution for Day 2 puzzle
...
└── day12/       # Solution for Day 12 puzzle
```

Each day's directory follows the same structure:
- `src/main.rs` - Contains the solution code with `part1()` and `part2()` functions
- `src/input.txt` - Contains the puzzle input data
- `Cargo.toml` - Package configuration for the specific day

## How to Run

To run a specific day's solution:

```bash
# Navigate to the repository root
cd aoc-2023

# Run a specific day's solution
cargo run -p day1  # Replace "day1" with the day you want to run
```

To run all solutions:

```bash
cargo run --workspace
```

## Implementation Details

Each solution follows the same pattern:
1. Read puzzle input from the `input.txt` file using Rust's `include_str!` macro
2. Implement two parts of the daily puzzle (`part1` and `part2` functions)
3. Process input according to puzzle-specific requirements
4. Output results to the console

## Technologies Used

- Rust programming language (2021 edition)
- Cargo for project management
- No external dependencies in individual day solutions

## Translations

- [French (Français)](README.fr.md)
- [Spanish (Español)](README.es.md)