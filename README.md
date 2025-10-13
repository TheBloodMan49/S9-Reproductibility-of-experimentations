
This repository contains :

- a simple test to check the associativity of the + operation on floats in Rust
- a reproduction of the investment problem and how rounding errors can be catastrophic

You will need to clone the repository to test the code.

```bash
git clone git@github.com:TheBloodMan49/S9-Reproductibility-of-experimentations.git
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
docker run --rm -it -v .:/app/tmp associativity
```

# Investment problem

## Build and Run

### Using your local Rust installation

If you don't have Rust install it using [rustup](https://rustup.rs/).

Then navigate to the cloned repository and run:

```bash
cd investment
cargo run --release
```

### Using Docker

You can also use Docker to build and run the code without installing Rust locally.

Make sure you have [Docker](https://get.docker.com/) installed and running.

Then navigate to the cloned repository and run:

```bash
docker build -t investment investment
docker run --rm investment
```
