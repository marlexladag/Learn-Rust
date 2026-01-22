# 03 - Variables and Mutability

Rust variables are **immutable by default**. This is a key safety feature.

## Immutable Variables

```rust
fn main() {
    let x = 5;
    println!("x is: {}", x);

    x = 6;  // ERROR! Cannot assign twice to immutable variable
}
```

## Mutable Variables

Use `mut` to make a variable mutable:

```rust
fn main() {
    let mut x = 5;
    println!("x is: {}", x);

    x = 6;  // OK! x is mutable
    println!("x is now: {}", x);
}
```

## Why Immutable by Default?

1. **Safety**: Prevents accidental changes
2. **Concurrency**: Immutable data is thread-safe
3. **Clarity**: You know a value won't change

## Constants

Constants are always immutable and must have a type annotation:

```rust
const MAX_POINTS: u32 = 100_000;
const PI: f64 = 3.14159;
```

**Constants vs Variables:**
| Feature | `const` | `let` |
|---------|---------|-------|
| Mutable | Never | With `mut` |
| Type annotation | Required | Optional |
| Set at | Compile time | Runtime |
| Scope | Any (often global) | Block |
| Naming | SCREAMING_SNAKE_CASE | snake_case |

## Shadowing

You can declare a new variable with the same name:

```rust
fn main() {
    let x = 5;
    let x = x + 1;      // x is now 6 (new variable)
    let x = x * 2;      // x is now 12 (another new variable)

    println!("x is: {}", x);
}
```

### Shadowing vs Mutability

```rust
// Shadowing: can change type
let spaces = "   ";        // &str
let spaces = spaces.len(); // usize - OK!

// Mutability: same type only
let mut spaces = "   ";
spaces = spaces.len();     // ERROR! Type mismatch
```

## Scope

Variables are valid within their scope (block):

```rust
fn main() {
    let outer = 1;

    {
        let inner = 2;
        println!("inner: {}", inner);  // OK
        println!("outer: {}", outer);  // OK - outer is in scope
    }

    println!("outer: {}", outer);      // OK
    // println!("inner: {}", inner);   // ERROR - inner is out of scope
}
```

## Variable Declaration Patterns

```rust
fn main() {
    // Basic declaration
    let a = 1;

    // With type annotation
    let b: i32 = 2;

    // Mutable
    let mut c = 3;

    // Multiple (tuple destructuring)
    let (x, y) = (4, 5);

    // Unused variable (prefix with _)
    let _unused = 6;

    // Declare now, assign later
    let d;
    d = 7;  // Must assign before use
}
```

## Try It

Run the example:

```bash
cargo run
```

Experiments:
1. Try to reassign an immutable variable (see the error)
2. Use shadowing to change a variable's type
3. Create a nested scope and observe variable lifetimes

---

Next: [04 - Data Types](../04_data_types/) - Understanding Rust's type system
