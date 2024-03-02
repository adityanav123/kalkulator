
# Kalkulator

`kalkulator` is a versatile tool for mathematical expression evaluation, offering both a command-line interface for direct usage and a library for integration into Rust projects.

## Features

- Evaluate mathematical expressions with support for basic arithmetic operations and factorials.
- Convert expressions to postfix notation.
- Extendable for future operations and functionalities.

## Installation

### As a Command-Line Tool

Ensure you have Rust and Cargo installed on your system. If you don't have Rust installed, you can install it from [the official site](https://www.rust-lang.org/tools/install).

You can install `kalkulator` directly from crates.io by running:

```bash
cargo install kalkulator
```
This command installs the `kalkulator` binary, making it available for use in your terminal


### As a library

Add `kalkulator` as a dependency in your `Cargo.toml` to use it in your Rust project.

```toml
Eg.
[dependencies]
kalkulator = "0.1.1"
```

## Usage

To convert an expression to postfix notation without evaluating:

```bash
kalkulator --expr "2+3/4" -p
```
To evaluate the result of an expression:

```bash
kalkulator --expr "2+3/4"
```

# Example

## Command-Line Interface
Evaluate an expression with basic arithmetic operations:

Command: 
```bash 
    kalkulator --expr "3+4^2"
```

Output:
```makefile
    Result = 11
```

Evaluate an Expression involving factorial

Command:
```bash
    kalkulator --expr "5!/(2+3)"
```

Output:
```makefile
    Result = 24
```

## Library

Here is a basic example of using `kalkulator` to evaluate an expression within a Rust project:

```rust
use kalkulator::Expression;

fn main() {
    let mut expr = Expression::new("3+4*2");
    expr.infix_to_postfix().unwrap();
    expr.compute_expression().unwrap();

    println!("The result is = {}", expr.get_result().unwrap());
}
```
For more detailed usage, check documentation: [Docs](https://docs.rs/kalkulator/latest/kalkulator/)
# Contributing

Contributions are welcome!
