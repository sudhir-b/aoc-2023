# Advent of Code 2023 Solutions

This repository contains my solutions for the [Advent of Code 2023](https://adventofcode.com/2023) challenges, implemented in Rust.

## Project Structure

The project is organized as a Rust workspace, with each day's solution as a separate crate:

```
.
├── day1/  - Day 1 solution
├── day2/  - Day 2 solution
├── day3/  - Day 3 solution
...
└── day12/ - Day 12 solution
```

Each day's crate contains its own source code and input files.

## Prerequisites

To run these solutions, you'll need:

- [Rust](https://www.rust-lang.org/tools/install) (latest stable version)
- Cargo (comes with Rust)

## Setup

1. Clone the repository:
   ```bash
   git clone https://github.com/yourusername/aoc-2023.git
   cd aoc-2023
   ```

2. Build all solutions:
   ```bash
   cargo build --release
   ```

## Running Solutions

To run a specific day's solution, use:

```bash
cargo run -p day1  # Replace 'day1' with the day you want to run
```

Or to run in release mode for better performance:

```bash
cargo run --release -p day1
```

## Development

Each day's solution is contained in its own crate within the workspace. To create a new day's solution:

1. Create a new directory for the day
2. Add it to the workspace members in `Cargo.toml`
3. Implement your solution
4. Add any input files needed

## Project Features

- Modular structure with separate crates for each day
- Fast compilation with workspace architecture
- Easy to run individual solutions
- Organized input handling

## Contributing

Feel free to open issues or submit pull requests if you have suggestions for improvements or alternative solutions.

## License

This project is available under the MIT License. See the LICENSE file for more details.