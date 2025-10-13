
This repository contains :

- a simple test to check the associativity of the + operation on floats in Rust
- a reproduction of the investment problem and how rounding errors can be catastrophic

You will need to clone the repository to test the code.

```bash
git clone <>
```

# Associativity test

## Build and Run

### Using your local Rust installation

If you don't have Rust install it using [rustup](https://rustup.rs/).

Then navigate to the cloned repository and run:

```bash
cd associativity
cargo run --release
```

### Using Docker

You can also use Docker to build and run the code without installing Rust locally.

Make sure you have [Docker](https://get.docker.com/) installed and running.

Then navigate to the cloned repository and run:

```bash
docker build -t associativity associativity
docker run --rm associativity
```

# Investment problem

## Build and Run

### Using your local Rust installation

If you don't have Rust install it using [rustup](https://rustup.rs/).

Then navigate to the cloned repository and run:

```bash
cd investment_problem
cargo run --release
```

### Using Docker

You can also use Docker to build and run the code without installing Rust locally.

Make sure you have [Docker](https://get.docker.com/) installed and running.

Then navigate to the cloned repository and run:

```bash
docker build -t investment_problem investment_problem
docker run --rm investment_problem
```
