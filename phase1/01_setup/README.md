# 01 - Setup & Tooling

## Installing Rust

The recommended way to install Rust is through `rustup`, the official installer.

### macOS / Linux

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Follow the prompts (default installation is fine).

### Verify Installation

```bash
rustc --version    # Rust compiler
cargo --version    # Package manager & build tool
rustup --version   # Toolchain manager
```

You should see version numbers for each.

## Understanding the Tools

### rustc - The Compiler

`rustc` compiles Rust source files directly:

```bash
rustc main.rs      # Compiles to executable
./main             # Run the executable
```

You rarely use `rustc` directly. Instead, use Cargo.

### Cargo - The Build System

Cargo is Rust's package manager and build tool. It handles:
- Creating projects
- Building code
- Managing dependencies
- Running tests
- Publishing crates

### Essential Cargo Commands

```bash
cargo new project_name    # Create new project
cargo build               # Compile (debug mode)
cargo build --release     # Compile (optimized)
cargo run                 # Build and run
cargo check               # Check without building (fast!)
cargo test                # Run tests
cargo doc --open          # Generate and open documentation
```

## Project Structure

When you run `cargo new hello`, it creates:

```
hello/
├── Cargo.toml      # Project configuration
└── src/
    └── main.rs     # Source code entry point
```

### Cargo.toml

```toml
[package]
name = "hello"
version = "0.1.0"
edition = "2021"    # Rust edition (use latest)

[dependencies]
# External crates go here
```

### src/main.rs

```rust
fn main() {
    println!("Hello, world!");
}
```

## Useful rustup Commands

```bash
rustup update              # Update Rust
rustup doc                 # Open local documentation
rustup component add rustfmt   # Add code formatter
rustup component add clippy    # Add linter
```

## Editor Setup

Recommended: Use VS Code with the `rust-analyzer` extension.

It provides:
- Code completion
- Inline type hints
- Error highlighting
- Go to definition
- Refactoring tools

## Try It

1. Create a new project:
   ```bash
   cargo new my_first_project
   cd my_first_project
   ```

2. Run it:
   ```bash
   cargo run
   ```

3. You should see: `Hello, world!`

---

Next: [02 - Hello World](../02_hello_world/) - Understanding your first program
