fn main() {
    println!("=== Basic Functions ===");
    greet();
    greet_person("Rustacean");

    println!("\n=== Functions with Parameters ===");
    print_sum(5, 3);
    print_labeled_measurement(5, "meters");

    println!("\n=== Functions with Return Values ===");
    let sum = add(10, 20);
    println!("add(10, 20) = {}", sum);

    let product = multiply(6, 7);
    println!("multiply(6, 7) = {}", product);

    println!("\n=== Expression vs Statement ===");
    let y = {
        let x = 3;
        x + 1 // Expression - no semicolon, returns 4
    };
    println!("Block expression result: {}", y);

    println!("\n=== Multiple Return Values ===");
    let (quotient, remainder) = divide(17, 5);
    println!("17 / 5 = {} remainder {}", quotient, remainder);

    println!("\n=== Early Return ===");
    println!("sign(-5) = {}", sign(-5));
    println!("sign(0) = {}", sign(0));
    println!("sign(10) = {}", sign(10));

    println!("\n=== Calling Other Functions ===");
    let result = calculate(10, 3);
    println!("calculate(10, 3) = {}", result);

    println!("\n=== Nested Functions ===");
    outer_function();

    println!("\n=== Recursive Functions ===");
    println!("factorial(5) = {}", factorial(5));
    println!("fibonacci(10) = {}", fibonacci(10));

    println!("\n=== Higher-Order Functions Preview ===");
    let numbers = [1, 2, 3, 4, 5];
    apply_to_all(&numbers, square);
}

// ========================================
// BASIC FUNCTIONS
// ========================================

fn greet() {
    println!("Hello!");
}

fn greet_person(name: &str) {
    println!("Hello, {}!", name);
}

// ========================================
// FUNCTIONS WITH PARAMETERS
// ========================================

fn print_sum(a: i32, b: i32) {
    println!("{} + {} = {}", a, b, a + b);
}

fn print_labeled_measurement(value: i32, unit: &str) {
    println!("The measurement is: {} {}", value, unit);
}

// ========================================
// FUNCTIONS WITH RETURN VALUES
// ========================================

fn add(x: i32, y: i32) -> i32 {
    x + y // No semicolon = implicit return
}

fn multiply(x: i32, y: i32) -> i32 {
    return x * y; // Explicit return works too
}

// ========================================
// MULTIPLE RETURN VALUES (TUPLES)
// ========================================

fn divide(dividend: i32, divisor: i32) -> (i32, i32) {
    let quotient = dividend / divisor;
    let remainder = dividend % divisor;
    (quotient, remainder)
}

// ========================================
// EARLY RETURN
// ========================================

fn sign(n: i32) -> &'static str {
    if n < 0 {
        return "negative";
    }
    if n > 0 {
        return "positive";
    }
    "zero" // Last expression, no return needed
}

// ========================================
// FUNCTIONS CALLING FUNCTIONS
// ========================================

fn square(x: i32) -> i32 {
    x * x
}

fn add_one(x: i32) -> i32 {
    x + 1
}

fn calculate(x: i32, y: i32) -> i32 {
    let sum = add(x, y);
    let squared = square(sum);
    add_one(squared)
}

// ========================================
// NESTED FUNCTIONS
// ========================================

fn outer_function() {
    fn inner_function() {
        println!("  Called from inner function!");
    }

    println!("In outer function");
    inner_function();
    inner_function();
}

// ========================================
// RECURSIVE FUNCTIONS
// ========================================

fn factorial(n: u64) -> u64 {
    if n <= 1 {
        1
    } else {
        n * factorial(n - 1)
    }
}

fn fibonacci(n: u32) -> u32 {
    match n {
        0 => 0,
        1 => 1,
        _ => fibonacci(n - 1) + fibonacci(n - 2),
    }
}

// ========================================
// HIGHER-ORDER FUNCTIONS (PREVIEW)
// ========================================

fn apply_to_all(arr: &[i32], f: fn(i32) -> i32) {
    print!("Applied function: [");
    for (i, &x) in arr.iter().enumerate() {
        if i > 0 {
            print!(", ");
        }
        print!("{}", f(x));
    }
    println!("]");
}
