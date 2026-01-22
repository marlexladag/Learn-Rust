# 04 - Data Types

Rust is **statically typed** - every variable has a known type at compile time.

## Scalar Types

Scalar types represent a single value.

### Integers

| Length | Signed | Unsigned |
|--------|--------|----------|
| 8-bit | `i8` | `u8` |
| 16-bit | `i16` | `u16` |
| 32-bit | `i32` | `u32` |
| 64-bit | `i64` | `u64` |
| 128-bit | `i128` | `u128` |
| arch | `isize` | `usize` |

- **Signed** (`i`): Can be negative. Range: -(2^(n-1)) to 2^(n-1)-1
- **Unsigned** (`u`): Only positive. Range: 0 to 2^n-1
- **arch**: Depends on architecture (64-bit on modern systems)

```rust
let a: i32 = -42;       // Signed 32-bit
let b: u8 = 255;        // Unsigned 8-bit (0-255)
let c: usize = 100;     // Pointer-sized unsigned
```

### Integer Literals

```rust
let decimal = 98_222;       // Underscores for readability
let hex = 0xff;             // Hexadecimal
let octal = 0o77;           // Octal
let binary = 0b1111_0000;   // Binary
let byte = b'A';            // u8 from ASCII character
```

### Floating-Point

```rust
let x = 2.0;        // f64 (default)
let y: f32 = 3.0;   // f32
```

- `f64`: 64-bit, double precision (default)
- `f32`: 32-bit, single precision

### Boolean

```rust
let t = true;
let f: bool = false;
```

### Character

```rust
let c = 'z';
let emoji = '😀';
let heart = '❤';
```

- 4 bytes (Unicode scalar value)
- Use single quotes (not double)

## Compound Types

Compound types group multiple values.

### Tuples

Fixed-length collection of different types:

```rust
let tup: (i32, f64, u8) = (500, 6.4, 1);

// Destructuring
let (x, y, z) = tup;

// Index access (zero-based)
let first = tup.0;
let second = tup.1;
```

Unit tuple `()` represents "no value" (like void).

### Arrays

Fixed-length collection of same type:

```rust
let arr = [1, 2, 3, 4, 5];        // Type inferred: [i32; 5]
let arr: [i32; 5] = [1, 2, 3, 4, 5];  // Explicit type

// Initialize with same value
let zeros = [0; 5];  // [0, 0, 0, 0, 0]

// Access elements
let first = arr[0];
let second = arr[1];
```

**Arrays vs Vectors:**
- Array: Fixed size, stack allocated
- Vec: Dynamic size, heap allocated (covered later)

## Type Inference

Rust usually infers types:

```rust
let x = 5;          // i32 (default integer)
let y = 2.0;        // f64 (default float)
let z = true;       // bool
let s = "hello";    // &str
```

Sometimes you need to annotate:

```rust
let guess: u32 = "42".parse().expect("Not a number!");
```

## Type Conversion

Rust requires explicit conversions:

```rust
let a: i32 = 5;
let b: i64 = a as i64;   // Cast with `as`

let c: f64 = 3.9;
let d: i32 = c as i32;   // Truncates to 3

let e: u8 = 255;
let f: i8 = e as i8;     // Wraps to -1 (be careful!)
```

## Numeric Operations

```rust
// Addition
let sum = 5 + 10;

// Subtraction
let difference = 95.5 - 4.3;

// Multiplication
let product = 4 * 30;

// Division
let quotient = 56.7 / 32.2;
let floored = 5 / 3;       // Integer division: 1

// Remainder
let remainder = 43 % 5;
```

## Try It

Run the example:

```bash
cargo run
```

Experiments:
1. Try to add an `i32` and `i64` without casting
2. See what happens with integer overflow in debug mode
3. Create a tuple with 4 different types

---

Next: [05 - Functions](../05_functions/) - Organizing code into reusable units
