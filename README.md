# Advent of Code 2023 Solutions

This repository contains my solutions for the [Advent of Code 2023](https://adventofcode.com/2023) challenge, implemented in Rust.

## Project Structure

The project is organized into separate crates for each day's challenge:

```
aoc-2023/
├── day1/
├── day2/
├── day3/
...
└── day12/
```

Each day's directory is a standalone Rust crate containing:
- Source code in the `src` directory
- A `Cargo.toml` file with package configuration

## Requirements

- Rust (2021 edition)
- Cargo (Rust's package manager)

## Getting Started

1. Clone the repository:
```bash
git clone [repository-url]
cd aoc-2023
```

2. Run a specific day's solution:
```bash
cd day1  # or any other day
cargo run
```

3. Run tests for a specific day:
```bash
cd day1  # or any other day
cargo test
```

## Running All Solutions

From the root directory, you can use cargo workspace commands to build or test all solutions:

```bash
cargo build --all    # Build all days
cargo test --all     # Run all tests
```

## Solutions Overview

Each day's solution is implemented as a separate Rust package with its own set of tests and input handling. The solutions focus on:

- Clean, readable code
- Efficient algorithms
- Proper error handling
- Unit tests for validation

## Project Structure Details

- `day1/` through `day12/`: Individual day solutions
- Each day's crate follows the standard Rust project structure
- Solutions are organized to handle both parts of each day's challenge

## Contributing

While this represents my personal solutions, suggestions for improvements are welcome:

1. Fork the repository
2. Create a feature branch
3. Submit a Pull Request

## License

This project is open source and available under the MIT License.

---
Created with ❤️ using Rust 🦀