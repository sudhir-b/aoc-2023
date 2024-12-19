# Advent of Code 2023

This repository contains my solutions to the [Advent of Code 2023](https://adventofcode.com/2023) challenges, implemented in Rust. Each day's solution is organized as a separate package within a Rust workspace.

## Project Structure

The project is organized as a Rust workspace with individual crates for each day's challenge:

```
.
├── Cargo.toml
├── Cargo.lock
├── day1/
├── day2/
├── day3/
├── day4/
├── day5/
├── day6/
├── day7/
├── day8/
├── day9/
├── day10/
├── day11/
└── day12/
```

Each day's directory contains its own `Cargo.toml` and source code in the `src` directory.

## Prerequisites

- Rust (latest stable version)
- Cargo (comes with Rust)

You can install Rust using [rustup](https://rustup.rs/):

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

## Building the Project

To build all solutions:

```bash
cargo build
```

To build a specific day's solution (e.g., day 1):

```bash
cargo build -p day1
```

## Running the Solutions

To run a specific day's solution:

```bash
cargo run -p day<N>
```

Replace `<N>` with the day number (1-12).

For example, to run day 1's solution:

```bash
cargo run -p day1
```

## Tests

To run tests for all solutions:

```bash
cargo test
```

To run tests for a specific day:

```bash
cargo test -p day<N>
```

## Project Structure Details

Each day's solution is structured as follows:

```
day<N>/
├── Cargo.toml
└── src/
    ├── main.rs
    └── lib.rs (if applicable)
```

## Contributing

This repository contains my personal solutions to Advent of Code 2023. While it's public for reference, I'm not accepting direct contributions as these are meant to be individual solutions.

## License

This project is available under the MIT License. See the LICENSE file for more details.

## Acknowledgments

- Thanks to [Eric Wastl](http://was.tl/) and the Advent of Code team for creating these engaging programming puzzles.
- Built with [Rust](https://www.rust-lang.org/) 🦀

## Progress

- [x] Day 1
- [x] Day 2
- [x] Day 3
- [x] Day 4
- [x] Day 5
- [x] Day 6
- [x] Day 7
- [x] Day 8
- [x] Day 9
- [x] Day 10
- [x] Day 11
- [x] Day 12

## Notes

- Each day's solution includes both parts 1 and 2 of the challenge
- Input files are not included in the repository per Advent of Code's request
- Solutions are optimized for readability and maintainability while keeping performance in mind