// Exercise 3: FizzBuzz
//
// Implement two versions of FizzBuzz.
// Run with: cargo run

fn main() {
    println!("=== FizzBuzz ===\n");

    // Test with if/else version
    println!("Using if/else:");
    for i in 1..=20 {
        print!("{} ", fizzbuzz_if(i));
    }
    println!("\n");

    // Test with match version
    println!("Using match:");
    for i in 1..=20 {
        print!("{} ", fizzbuzz_match(i));
    }
    println!("\n");

    // Individual tests
    println!("Individual tests:");
    println!("fizzbuzz(3) = \"{}\"", fizzbuzz_match(3));
    println!("fizzbuzz(5) = \"{}\"", fizzbuzz_match(5));
    println!("fizzbuzz(15) = \"{}\"", fizzbuzz_match(15));
    println!("fizzbuzz(7) = \"{}\"", fizzbuzz_match(7));
}

/// FizzBuzz using if/else
/// Returns "Fizz", "Buzz", "FizzBuzz", or the number as string
fn fizzbuzz_if(n: u32) -> String {
    // TODO: Implement using if/else
    // Remember to check "divisible by both" FIRST!
    // Use n.to_string() to convert number to String
    String::new()
}

/// FizzBuzz using match
/// Returns "Fizz", "Buzz", "FizzBuzz", or the number as string
fn fizzbuzz_match(n: u32) -> String {
    // TODO: Implement using match
    // Hint: match on (n % 3, n % 5) tuple
    // Pattern (0, 0) means divisible by both
    String::new()
}
