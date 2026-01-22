# 06 - Control Flow

Control flow determines which code runs based on conditions.

## if Expressions

```rust
let number = 7;

if number < 5 {
    println!("less than 5");
} else if number > 10 {
    println!("greater than 10");
} else {
    println!("between 5 and 10");
}
```

**Important:** Conditions MUST be `bool`. Rust doesn't convert other types:

```rust
// ERROR: expected bool, found integer
if number {  // Won't compile!
    println!("won't work");
}

// Correct
if number != 0 {
    println!("number is not zero");
}
```

### if as Expression

`if` returns a value in Rust:

```rust
let condition = true;
let number = if condition { 5 } else { 6 };
// number is 5
```

Both branches must return the same type:

```rust
// ERROR: Different types
let number = if condition { 5 } else { "six" };
```

## Loops

Rust has three loop types: `loop`, `while`, and `for`.

### loop - Infinite Loop

```rust
loop {
    println!("forever!");
    break;  // Exit the loop
}
```

#### Returning from Loops

```rust
let result = loop {
    counter += 1;
    if counter == 10 {
        break counter * 2;  // Return value with break
    }
};
// result is 20
```

#### Loop Labels

```rust
'outer: loop {
    'inner: loop {
        break 'outer;  // Break the outer loop
    }
}
```

### while - Conditional Loop

```rust
let mut number = 3;

while number != 0 {
    println!("{}", number);
    number -= 1;
}
println!("Liftoff!");
```

### for - Iterator Loop

The most common loop in Rust:

```rust
let arr = [10, 20, 30, 40, 50];

for element in arr {
    println!("{}", element);
}
```

#### Range Syntax

```rust
// 0, 1, 2, 3, 4
for i in 0..5 {
    println!("{}", i);
}

// 1, 2, 3, 4, 5 (inclusive)
for i in 1..=5 {
    println!("{}", i);
}

// Reverse
for i in (1..4).rev() {
    println!("{}", i);  // 3, 2, 1
}
```

#### Enumerate

Get index and value:

```rust
for (index, value) in arr.iter().enumerate() {
    println!("{}: {}", index, value);
}
```

## match - Pattern Matching

Rust's powerful pattern matching:

```rust
let number = 3;

match number {
    1 => println!("one"),
    2 => println!("two"),
    3 => println!("three"),
    _ => println!("something else"),  // Default case
}
```

### match Must Be Exhaustive

```rust
let boolean = true;

match boolean {
    true => println!("true"),
    false => println!("false"),
    // Must handle all cases!
}
```

### match with Ranges and Guards

```rust
let age = 25;

match age {
    0..=12 => println!("child"),
    13..=19 => println!("teenager"),
    20..=64 => println!("adult"),
    _ => println!("senior"),
}

// With guard
match age {
    n if n < 0 => println!("invalid"),
    n if n < 18 => println!("minor"),
    _ => println!("adult"),
}
```

### match Returns Value

```rust
let result = match number {
    1 => "one",
    2 => "two",
    _ => "many",
};
```

## if let - Shortcut for Single Pattern

When you only care about one case:

```rust
let some_value = Some(3);

// Instead of:
match some_value {
    Some(x) => println!("got {}", x),
    _ => (),
}

// Use if let:
if let Some(x) = some_value {
    println!("got {}", x);
}
```

## while let

Loop while pattern matches:

```rust
let mut stack = vec![1, 2, 3];

while let Some(top) = stack.pop() {
    println!("{}", top);  // 3, 2, 1
}
```

## Control Flow Keywords

| Keyword | Purpose |
|---------|---------|
| `break` | Exit loop |
| `continue` | Skip to next iteration |
| `return` | Exit function |

```rust
for i in 0..10 {
    if i == 3 {
        continue;  // Skip 3
    }
    if i == 7 {
        break;     // Stop at 7
    }
    println!("{}", i);  // 0, 1, 2, 4, 5, 6
}
```

## Try It

Run the example:

```bash
cargo run
```

Experiments:
1. Create a guessing game loop that breaks on correct guess
2. Use match to categorize numbers
3. Implement countdown with while and for, compare them

---

Next: [Exercises](../exercises/) - Practice what you've learned!
