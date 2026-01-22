// Solution: FizzBuzz

fn main() {
    println!("=== FizzBuzz ===\n");

    println!("Using if/else:");
    for i in 1..=20 {
        print!("{} ", fizzbuzz_if(i));
    }
    println!("\n");

    println!("Using match:");
    for i in 1..=20 {
        print!("{} ", fizzbuzz_match(i));
    }
    println!("\n");

    println!("Individual tests:");
    println!("fizzbuzz(3) = \"{}\"", fizzbuzz_match(3));
    println!("fizzbuzz(5) = \"{}\"", fizzbuzz_match(5));
    println!("fizzbuzz(15) = \"{}\"", fizzbuzz_match(15));
    println!("fizzbuzz(7) = \"{}\"", fizzbuzz_match(7));
}

/// FizzBuzz using if/else
fn fizzbuzz_if(n: u32) -> String {
    if n % 3 == 0 && n % 5 == 0 {
        String::from("FizzBuzz")
    } else if n % 3 == 0 {
        String::from("Fizz")
    } else if n % 5 == 0 {
        String::from("Buzz")
    } else {
        n.to_string()
    }
}

/// FizzBuzz using match
fn fizzbuzz_match(n: u32) -> String {
    match (n % 3, n % 5) {
        (0, 0) => String::from("FizzBuzz"),
        (0, _) => String::from("Fizz"),
        (_, 0) => String::from("Buzz"),
        _ => n.to_string(),
    }
}
