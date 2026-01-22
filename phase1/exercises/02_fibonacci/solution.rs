// Solution: Fibonacci Sequence

fn main() {
    println!("=== Fibonacci Generator ===\n");

    println!("Single values (recursive):");
    println!("F(0) = {}", fibonacci_recursive(0));
    println!("F(1) = {}", fibonacci_recursive(1));
    println!("F(10) = {}", fibonacci_recursive(10));
    println!("F(20) = {}", fibonacci_recursive(20));

    println!("\nSingle values (iterative):");
    println!("F(0) = {}", fibonacci_iterative(0));
    println!("F(1) = {}", fibonacci_iterative(1));
    println!("F(10) = {}", fibonacci_iterative(10));
    println!("F(20) = {}", fibonacci_iterative(20));
    println!("F(50) = {}", fibonacci_iterative(50));

    println!("\nSequence (first 15 numbers):");
    println!("{:?}", fibonacci_sequence(14));
}

/// Returns the nth Fibonacci number using recursion
fn fibonacci_recursive(n: u32) -> u64 {
    match n {
        0 => 0,
        1 => 1,
        _ => fibonacci_recursive(n - 1) + fibonacci_recursive(n - 2),
    }
}

/// Returns the nth Fibonacci number using iteration
fn fibonacci_iterative(n: u32) -> u64 {
    if n == 0 {
        return 0;
    }
    if n == 1 {
        return 1;
    }

    let mut prev = 0u64;
    let mut curr = 1u64;

    for _ in 2..=n {
        let next = prev + curr;
        prev = curr;
        curr = next;
    }

    curr
}

/// Returns a vector containing the first n+1 Fibonacci numbers
fn fibonacci_sequence(n: u32) -> Vec<u64> {
    if n == 0 {
        return vec![0];
    }

    let mut sequence = vec![0, 1];

    for i in 2..=n {
        let next = sequence[(i - 1) as usize] + sequence[(i - 2) as usize];
        sequence.push(next);
    }

    sequence
}
