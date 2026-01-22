# 05 - Functions

Functions are the building blocks of Rust programs.

## Basic Syntax

```rust
fn function_name() {
    // body
}
```

Rust uses **snake_case** for function and variable names.

## Function with Parameters

```rust
fn greet(name: &str) {
    println!("Hello, {}!", name);
}

fn add(x: i32, y: i32) {
    println!("{} + {} = {}", x, y, x + y);
}
```

**Important:** You MUST declare the type of each parameter.

## Return Values

Use `->` to declare return type:

```rust
fn add(x: i32, y: i32) -> i32 {
    x + y  // No semicolon = return this value
}

fn subtract(x: i32, y: i32) -> i32 {
    return x - y;  // Explicit return also works
}
```

### Expressions vs Statements

- **Statement**: Performs action, no value (`let x = 5;`)
- **Expression**: Evaluates to a value (`5 + 6`)

```rust
fn main() {
    // Statement (no return value)
    let x = 5;

    // Expression (returns a value)
    let y = {
        let x = 3;
        x + 1  // No semicolon - this block returns 4
    };

    println!("y = {}", y);  // 4
}
```

**The semicolon rule:**
- `x + 1` → Expression, returns value
- `x + 1;` → Statement, returns `()`

## Early Return

Use `return` to exit early:

```rust
fn check_positive(n: i32) -> &'static str {
    if n < 0 {
        return "negative";
    }
    "positive or zero"  // Implicit return
}
```

## Functions that Return Nothing

Functions without `->` return the unit type `()`:

```rust
fn say_hello() {
    println!("Hello!");
    // Implicitly returns ()
}

fn explicit_unit() -> () {
    println!("Same thing");
}
```

## Multiple Return Values (Tuples)

```rust
fn divide(dividend: i32, divisor: i32) -> (i32, i32) {
    let quotient = dividend / divisor;
    let remainder = dividend % divisor;
    (quotient, remainder)
}

fn main() {
    let (q, r) = divide(17, 5);
    println!("17 / 5 = {} remainder {}", q, r);
}
```

## Function Call Order

Unlike C, you can call functions defined later in the file:

```rust
fn main() {
    helper();  // OK! Defined below
}

fn helper() {
    println!("I'm helping!");
}
```

## Nested Functions

Functions can be defined inside other functions:

```rust
fn outer() {
    fn inner() {
        println!("Inner function");
    }

    inner();  // Can only be called within outer()
}
```

## Diverging Functions

Functions that never return use `!`:

```rust
fn forever() -> ! {
    loop {
        // Never exits
    }
}

fn crash() -> ! {
    panic!("This function never returns");
}
```

## Documentation Comments

```rust
/// Adds two numbers together.
///
/// # Arguments
///
/// * `a` - First number
/// * `b` - Second number
///
/// # Returns
///
/// The sum of a and b
///
/// # Example
///
/// ```
/// let result = add(2, 3);
/// assert_eq!(result, 5);
/// ```
fn add(a: i32, b: i32) -> i32 {
    a + b
}
```

Generate docs with: `cargo doc --open`

## Try It

Run the example:

```bash
cargo run
```

Experiments:
1. Write a function that returns multiple values
2. Create a function that calls another function
3. Experiment with expression vs statement returns

---

Next: [06 - Control Flow](../06_control_flow/) - Making decisions in code
