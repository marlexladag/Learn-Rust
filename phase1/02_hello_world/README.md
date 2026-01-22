# 02 - Hello World

Let's understand every part of a Rust program.

## The Code

```rust
fn main() {
    println!("Hello, world!");
}
```

Run it: `cargo run`

## Breaking It Down

### `fn main()`

```rust
fn main() {
    // code here
}
```

- `fn` declares a function
- `main` is special - it's the entry point of every Rust program
- `()` means no parameters
- `{}` contains the function body

### `println!`

```rust
println!("Hello, world!");
```

- `println!` is a **macro** (note the `!`)
- Macros are like functions but can do more (we'll learn why later)
- The string `"Hello, world!"` is passed to it
- `;` ends the statement (Rust requires semicolons)

## Macros vs Functions

The `!` indicates a macro. Common macros:

| Macro | Purpose |
|-------|---------|
| `println!` | Print with newline |
| `print!` | Print without newline |
| `format!` | Return formatted string |
| `vec!` | Create a vector |
| `panic!` | Crash with message |

## String Formatting

`println!` supports formatting with `{}`:

```rust
fn main() {
    // Basic substitution
    println!("Hello, {}!", "Rust");

    // Multiple values
    println!("{} + {} = {}", 2, 3, 5);

    // Debug formatting with {:?}
    println!("Debug: {:?}", (1, 2, 3));

    // Pretty debug with {:#?}
    println!("Pretty: {:#?}", vec![1, 2, 3]);

    // Named arguments
    println!("{name} is {age} years old", name="Alice", age=30);

    // Positional arguments
    println!("{0} {1} {0}", "tick", "tock");
}
```

## Comments

```rust
fn main() {
    // Single line comment

    /*
       Multi-line
       comment
    */

    /// Documentation comment (for items below it)
    /// Used to generate docs with `cargo doc`

    //! Inner documentation comment (for the containing item)
}
```

## Try It

Run the example in this directory:

```bash
cargo run
```

Modify `src/main.rs` to:
1. Print your name
2. Print a calculation (e.g., `2 + 2 = {}`)
3. Try the debug format `{:?}`

---

Next: [03 - Variables](../03_variables/) - Storing and using data
