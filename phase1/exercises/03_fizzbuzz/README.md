# Exercise 3: FizzBuzz

The classic programming challenge!

## Rules

For numbers 1 to n:
- If divisible by 3, print "Fizz"
- If divisible by 5, print "Buzz"
- If divisible by both 3 and 5, print "FizzBuzz"
- Otherwise, print the number

## Task

Implement two versions:

1. `fizzbuzz_if(n: u32) -> String`
   - Use if/else statements

2. `fizzbuzz_match(n: u32) -> String`
   - Use match with tuple pattern matching

## Expected Output

```
=== FizzBuzz ===

Using if/else:
1 2 Fizz 4 Buzz Fizz 7 8 Fizz Buzz 11 Fizz 13 14 FizzBuzz 16 17 Fizz 19 Buzz

Using match:
1 2 Fizz 4 Buzz Fizz 7 8 Fizz Buzz 11 Fizz 13 14 FizzBuzz 16 17 Fizz 19 Buzz

Individual tests:
fizzbuzz(3) = "Fizz"
fizzbuzz(5) = "Buzz"
fizzbuzz(15) = "FizzBuzz"
fizzbuzz(7) = "7"
```

## Hints

<details>
<summary>Hint 1: Modulo operator</summary>

Use `%` to check divisibility:
- `n % 3 == 0` means n is divisible by 3
</details>

<details>
<summary>Hint 2: Order matters in if/else</summary>

Check "divisible by both" FIRST, before checking individually.
</details>

<details>
<summary>Hint 3: Match with tuples</summary>

```rust
match (n % 3, n % 5) {
    (0, 0) => ???,  // divisible by both
    (0, _) => ???,  // divisible by 3 only
    (_, 0) => ???,  // divisible by 5 only
    _ => ???,       // neither
}
```
</details>

<details>
<summary>Hint 4: Converting number to String</summary>

Use `n.to_string()` to convert a number to a String.
</details>

## Run Your Code

```bash
cargo run
```

## Check Solution

Once you've tried, compare with `solution.rs`
