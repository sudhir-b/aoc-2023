# Advent of Code Solutions in Rust

This repository contains solutions to Advent of Code challenges implemented in Rust. Each day's solution is organized in its own directory with a consistent structure.

## Project Structure

```
.
├── day1/
│   ├── src/
│   │   ├── main.rs
│   │   └── input.txt
├── day2/
│   └── ...
└── ...
```

Each day's solution is contained in its own directory with a standard Rust project structure. The input files are read from `input.txt` within each day's directory.

## Prerequisites

- Rust (latest stable version)
- Cargo (comes with Rust)

## Setup

1. Clone the repository:
```bash
git clone <repository-url>
cd advent-of-code-rust
```

2. Navigate to any day's directory:
```bash
cd day1
```

3. Run the solution:
```bash
cargo run --release
```

## Solutions Overview

- **Day 1**: String manipulation and digit extraction
- **Day 2**: Game simulation and constraint checking
- **Day 3**: Grid parsing and number extraction
- **Day 4**: Set operations and card game simulation
- **Day 5**: Range mapping and transformations
- **Day 6**: Race optimization problem
- **Day 7**: Card game hand classification
- **Day 8**: Graph traversal and cycle detection
- **Day 9**: Sequence extrapolation
- **Day 10**: Pipe maze navigation and flood fill
- **Day 11**: Galaxy expansion and Manhattan distance
- **Day 12**: Dynamic programming with spring arrangements

## Implementation Details

Each solution typically follows this pattern:
- Input is read from `input.txt`
- Solution is split into `part1()` and `part2()`
- Results are printed to stdout

Common patterns used across solutions:
- Grid operations using Vec<Vec<char>>
- Dynamic programming with memoization
- Graph traversal algorithms
- String parsing and manipulation

## Running Tests

Each day's solution can be tested using:
```bash
cargo test
```

## Performance

Solutions are optimized for reasonable performance. Use the `--release` flag when running solutions for optimal performance:
```bash
cargo run --release
```

## Contributing

Feel free to suggest improvements or alternative solutions through pull requests. Please maintain the existing code structure and include appropriate comments.

## License

This project is open source and available under the MIT License.