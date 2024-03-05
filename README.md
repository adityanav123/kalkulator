
# Kalkulator

`kalkulator` is a versatile tool for mathematical expression evaluation, offering both a command-line interface for direct usage and a library for integration into Rust projects.

## Features

- Evaluate mathematical expressions with support for basic arithmatic operations, factorial, logical operations
- Convert expressions to postfix notation.
- Extendable for future operations and functionalities.

### Further Extensions

- [ ] Support for Logarithmic and Trignometric Functions
- [☑](done) Exponentiation (x^y)
- [ ] Absolute Value, Rounding Functions
- [ ] Complex Numbers Support
- [ ] Computation history support


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

To show all available operations:
```bash
kalkulator --show-ops
```

## Library Usage

Here is a basic example of using `kalkulator` to evaluate an expression within a Rust project:

```rust
use kalkulator::Expression;

let mut expr = Expression::new("3+4*2");
expr.infix_to_postfix().unwrap(); // Converts to postfix notation
expr.compute_expression().unwrap(); // Evaluates the expression
assert_eq!(expr.get_result().unwrap(), 11); // The result is 11
```

Using the Expression struct to evaluate an expression with factorial and division:
```rust
use kalkulator::Expression;

let mut expr = Expression::new("4!/(2+3)");
expr.infix_to_postfix().unwrap(); // Converts to postfix notation
expr.compute_expression().unwrap(); // Evaluates the expression
assert_eq!(expr.get_result().unwrap(), 24); // The result is 24 (120 / 5)
```

For more detailed usage, check documentation: [Docs](https://docs.rs/kalkulator/latest/kalkulator/)
# Contributing

Contributions are welcome!, Whether it's adding new features, improving existing ones, or reporting bugs, your input is valuable. Check out the project's repository on GitHub
