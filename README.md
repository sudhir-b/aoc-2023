# Advent of Code 2023 - Rust Solutions

This repository contains my Rust solutions for the [Advent of Code 2023](https://adventofcode.com/2023) programming challenges.

## Project Structure

The repository is organized as a Rust workspace with multiple crates, each representing a solution for a specific day's challenge:

- `day1` through `day12`: Individual crates for each day's puzzle
- Each day follows the same structure:
  - `src/main.rs`: Contains solution code with `part1()` and `part2()` functions
  - `src/input.txt`: Contains the puzzle input data
  - `Cargo.toml`: Package configuration

## Technical Details

- **Language**: Rust (2021 edition)
- **Build System**: Cargo
- **Dependencies**: No external dependencies in the solution crates

## How to Run

You can run any day's solution using Cargo:

```bash
# Run a specific day (e.g., day1)
cargo run -p day1

# Run with release optimizations
cargo run --release -p day1
```

## Solutions Implemented

- Day 1: Trebuchet?!
- Day 2: Cube Conundrum
- Day 3: Gear Ratios
- Day 4: Scratchcards
- Day 5: If You Give A Seed A Fertilizer
- Day 6: Wait For It
- Day 7: Camel Cards
- Day 8: Haunted Wasteland
- Day 9: Mirage Maintenance
- Day 10: Pipe Maze
- Day 11: Cosmic Expansion
- Day 12: Hot Springs

## About Advent of Code

Advent of Code is an annual set of Christmas-themed programming puzzles that can be solved in any programming language. Each puzzle consists of two parts, with the second part building upon the first.

For more information, visit [adventofcode.com](https://adventofcode.com/).