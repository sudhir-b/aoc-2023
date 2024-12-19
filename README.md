# Advent of Code 2023 Solutions

This repository contains my solutions for the [Advent of Code 2023](https://adventofcode.com/2023) programming puzzles, implemented in Rust. Each day's solution is organized as a separate crate within a Rust workspace.

## Project Overview

Advent of Code is an annual set of Christmas-themed programming puzzles that cover a variety of skill sets and skill levels. This repository contains solutions for the 2023 edition, with each day's solution implemented as a separate Rust project.

## Prerequisites

To run these solutions, you'll need:

- [Rust](https://www.rust-lang.org/tools/install) (latest stable version)
- Cargo (comes with Rust)

## Installation

1. Clone the repository:
   ```bash
   git clone https://github.com/yourusername/aoc-2023.git
   cd aoc-2023
   ```

2. Build all solutions:
   ```bash
   cargo build --release
   ```

## Project Structure

The repository is organized as a Rust workspace with multiple crates:

```
aoc-2023/
├── Cargo.toml           # Workspace configuration
├── Cargo.lock
├── day1/               # Solution for Day 1
│   ├── Cargo.toml
│   └── src/
│       ├── main.rs
│       └── input.txt
├── day2/               # Solution for Day 2
│   └── ...
└── ...                # And so on for each day
```

Each day's solution is contained in its own directory with:
- A `main.rs` file containing the solution code
- An `input.txt` file containing the puzzle input
- A `Cargo.toml` file with crate-specific configurations

## Running Solutions

To run a specific day's solution:

```bash
# Run a specific day (e.g., day 1)
cargo run -p day1

# Or cd into the day's directory and run
cd day1
cargo run
```

To run all solutions:

```bash
# From the root directory
for d in day{1..12}; do echo "Running $d..."; cargo run -p $d; done
```

## Development

Each day's solution is independent, making it easy to work on them separately. To add a new day's solution:

1. Create a new directory for the day
2. Add a new Cargo.toml file
3. Create a src directory with main.rs and input.txt
4. Add the new day to the workspace members in the root Cargo.toml

## Contributing

Feel free to explore the solutions or suggest improvements. Each solution includes the original puzzle input and the solution code.

## License

This project is available under the MIT License. See the LICENSE file for more details.

## Acknowledgments

- Thanks to [Advent of Code](https://adventofcode.com) for creating these engaging puzzles
- The Rust programming language community for excellent documentation and tools