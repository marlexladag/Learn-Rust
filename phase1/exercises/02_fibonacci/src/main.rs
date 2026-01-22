// Exercise 2: Fibonacci Sequence
//
// Implement three Fibonacci functions.
// Run with: cargo run

fn main() {
    println!("=== Fibonacci Generator ===\n");

    // Test recursive version
    println!("Single values (recursive):");
    println!("F(0) = {}", fibonacci_recursive(0));
    println!("F(1) = {}", fibonacci_recursive(1));
    println!("F(10) = {}", fibonacci_recursive(10));
    println!("F(20) = {}", fibonacci_recursive(20));
    // Note: recursive is slow for large n, so we stop at 20

    // Test iterative version
    println!("\nSingle values (iterative):");
    println!("F(0) = {}", fibonacci_iterative(0));
    println!("F(1) = {}", fibonacci_iterative(1));
    println!("F(10) = {}", fibonacci_iterative(10));
    println!("F(20) = {}", fibonacci_iterative(20));
    println!("F(50) = {}", fibonacci_iterative(50));

    // Test sequence generation
    println!("\nSequence (first 15 numbers):");
    println!("{:?}", fibonacci_sequence(14));
}

/// Returns the nth Fibonacci number using recursion
/// F(0) = 0, F(1) = 1, F(n) = F(n-1) + F(n-2)
fn fibonacci_recursive(n: u32) -> u64 {
    // TODO: Implement using recursion
    // Hint: Use match with cases for 0, 1, and _
    0
}

/// Returns the nth Fibonacci number using iteration
/// More efficient than recursion for large n
fn fibonacci_iterative(n: u32) -> u64 {
    // TODO: Implement using a loop
    // Hint: Keep track of the previous two numbers
    0
}

/// Returns a vector containing the first n+1 Fibonacci numbers
/// fibonacci_sequence(4) returns [0, 1, 1, 2, 3]
fn fibonacci_sequence(n: u32) -> Vec<u64> {
    // TODO: Implement to return a vector of Fibonacci numbers
    // Hint: Start with vec![0, 1] and push new values
    vec![]
}
