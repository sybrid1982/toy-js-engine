# Toy JS Engine

A simple JavaScript interpreter built in Rust for learning and experimentation.

## Current Features
- Basic math operators: `+`, `-`, `*`, `/`, `**`, parentheses
- Logic operators: `&&`, `||`, `!`
- Comparators: `>`, `<`, `>=`, `<=`, `==`, `!=`
- Increment and decrement prefixes (`--x`, `++x`)
- Assignment operators (`=`, `*=`, `/=`, `+=`, `-=`)
- Works with booleans, strings and numbers
- Automatic string and boolean coercion when adding
- Function declarations with `return` statements
- Function hoisting inside given scope
- Calling defined functions
- `if`, `else if`, and `else` statements
- `while` loops

## Getting Started
Build the project with:
```bash
cargo build
```

Run the interactive prompt:
```bash
cargo run
```
At the prompt try:
```text
3 + 5 * 6;
```
which should print `33`. Or try:
```text
let x = 3 * 5;
x + 3;
```
which should output `18`.

See `src/integration_tests.rs` for more examples of code that can be executed.

## Running Tests
Execute the tests with:
```bash
cargo test
```

## To Do
- Fill out more operators from the operator precedence chart
- For loops

## Future Ideas
- Consider replacing some statement enum variants with structs to better restrict what values they hold
