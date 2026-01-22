# Exercise 2: Fibonacci Sequence

Generate Fibonacci numbers using different approaches.

## Background

The Fibonacci sequence: 0, 1, 1, 2, 3, 5, 8, 13, 21, 34, ...

Each number is the sum of the two preceding ones:
- F(0) = 0
- F(1) = 1
- F(n) = F(n-1) + F(n-2) for n > 1

## Task

Implement three versions:

1. `fibonacci_recursive(n: u32) -> u64`
   - Use recursion (simple but slow for large n)

2. `fibonacci_iterative(n: u32) -> u64`
   - Use a loop (efficient)

3. `fibonacci_sequence(n: u32) -> Vec<u64>`
   - Return first n+1 Fibonacci numbers as a vector

## Expected Output

```
=== Fibonacci Generator ===

Single values (recursive):
F(0) = 0
F(1) = 1
F(10) = 55
F(20) = 6765

Single values (iterative):
F(0) = 0
F(1) = 1
F(10) = 55
F(20) = 6765
F(50) = 12586269025

Sequence (first 15 numbers):
[0, 1, 1, 2, 3, 5, 8, 13, 21, 34, 55, 89, 144, 233, 377]
```

## Hints

<details>
<summary>Hint 1: Recursive approach</summary>

```rust
fn fibonacci_recursive(n: u32) -> u64 {
    match n {
        0 => ???,
        1 => ???,
        _ => ??? + ???,
    }
}
```
</details>

<details>
<summary>Hint 2: Iterative approach</summary>

Keep track of two variables: the previous two Fibonacci numbers.
Update them in a loop.
</details>

<details>
<summary>Hint 3: Building a sequence</summary>

```rust
let mut sequence = Vec::new();
// or
let mut sequence = vec![0, 1];
```
</details>

## Run Your Code

```bash
cargo run
```

## Note

The recursive version is slow for n > 30. That's expected!
The iterative version handles large n efficiently.

## Check Solution

Once you've tried, compare with `solution.rs`
