# Exercise 1: Temperature Converter

Convert temperatures between Celsius and Fahrenheit.

## Task

Implement two functions:

1. `celsius_to_fahrenheit(celsius: f64) -> f64`
   - Formula: F = (C × 9/5) + 32

2. `fahrenheit_to_celsius(fahrenheit: f64) -> f64`
   - Formula: C = (F - 32) × 5/9

## Expected Output

```
=== Temperature Converter ===

Celsius to Fahrenheit:
0°C = 32°F
100°C = 212°F
-40°C = -40°F
37°C = 98.6°F

Fahrenheit to Celsius:
32°F = 0°C
212°F = 100°C
-40°F = -40°C
98.6°F = 37°C
```

## Hints

<details>
<summary>Hint 1: Function signature</summary>

```rust
fn celsius_to_fahrenheit(celsius: f64) -> f64 {
    // Your code here
}
```
</details>

<details>
<summary>Hint 2: Arithmetic in Rust</summary>

Use floating-point division: `9.0 / 5.0` not `9 / 5`
</details>

<details>
<summary>Hint 3: Formatting floats</summary>

Use `{:.1}` to show 1 decimal place: `println!("{:.1}", 98.6)`
</details>

## Run Your Code

```bash
cargo run
```

## Check Solution

Once you've tried, compare with `solution.rs`
