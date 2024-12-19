# Rust Daily Challenges

This repository contains a collection of Rust programming challenges organized into daily modules. Each day represents a separate challenge or set of exercises implemented in Rust.

## Project Structure

The project is organized as a Rust workspace with multiple crates:

```
project/
├── Cargo.toml       # Workspace configuration
├── Cargo.lock
├── day1/           # Day 1 challenge
├── day2/           # Day 2 challenge
├── ...
└── day12/          # Day 12 challenge
```

Each day's directory contains:
- `Cargo.toml`: Crate-specific configuration and dependencies
- `src/`: Source code for the day's challenge

## Prerequisites

To run this project, you need:

1. Rust (latest stable version)
2. Cargo (comes with Rust)

## Installation

1. Clone the repository:
```bash
git clone [repository-url]
cd project
```

2. Build the project:
```bash
cargo build
```

## Running the Challenges

You can run any day's challenge using:

```bash
# Run a specific day (replace X with day number 1-12)
cargo run -p dayX

# Run tests for a specific day
cargo test -p dayX

# Run all tests
cargo test
```

## Development

Each day's challenge is contained in its own crate, making it easy to work on challenges independently. To create a new day's challenge:

1. Create a new directory: `dayX/`
2. Add a `Cargo.toml` file with necessary dependencies
3. Create the source code in `src/`
4. Add the new day to the workspace members in the root `Cargo.toml`

## Contributing

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/AmazingFeature`)
3. Commit your changes (`git commit -m 'Add some AmazingFeature'`)
4. Push to the branch (`git push origin feature/AmazingFeature`)
5. Open a Pull Request

## License

This project is open source and available under the [MIT License](LICENSE).