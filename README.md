
This repository contains :

- a simple test to check the associativity of the + operation on floats in Rust
- a reproduction of the investment problem and how rounding errors can be catastrophic

You will need to clone the repository to test the code.

```bash
git clone git@github.com:TheBloodMan49/S9-Reproductibility-of-experimentations.git
```

# Associativity test

## Experimentation results

You can find the results of the experimentations in the file [associativity_test](associativity/associativity_test_original.txt) or in the [releases](https://github.com/TheBloodMan49/S9-Reproductibility-of-experimentations/releases) section of this repository.

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

### Using Nix

If you have [Nix](https://nixos.org/download.html) installed, you can use it to build and run the code.

Navigate to the cloned repository and run:

```bash
nix-shell
cd associativity
cargo run --release
```

### Using Nix Flakes

Run :

```bash
cd associativity
nix --extra-experimental-features nix-command --extra-experimental-features flakes build
./result/bin/test
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
docker run --rm -v .:/app/tmp investment
```

### Using Nix

If you have [Nix](https://nixos.org/download.html) installed, you can use it to build and run the code.

Navigate to the cloned repository and run:

```bash
nix-shell
cd investment
cargo run --release
```

### Using Nix Flakes

Run :

```bash
cd investment
nix --extra-experimental-features nix-command --extra-experimental-features flakes build
./result/bin/test
```
