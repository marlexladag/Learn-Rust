fn main() {
    // ========================================
    // IMMUTABLE VARIABLES (default)
    // ========================================
    let x = 5;
    println!("x is: {}", x);

    // Uncomment to see the error:
    // x = 6;  // ERROR: cannot assign twice to immutable variable

    // ========================================
    // MUTABLE VARIABLES
    // ========================================
    let mut y = 10;
    println!("y is: {}", y);
    y = 20; // OK because y is mutable
    println!("y is now: {}", y);

    // ========================================
    // CONSTANTS
    // ========================================
    const MAX_SCORE: u32 = 100;
    const PI: f64 = 3.14159;
    println!("Max score: {}, PI: {}", MAX_SCORE, PI);

    // ========================================
    // SHADOWING
    // ========================================
    let z = 1;
    println!("z = {}", z);

    let z = z + 1; // Shadow z with new value
    println!("z = {} (after +1)", z);

    let z = z * 2; // Shadow again
    println!("z = {} (after *2)", z);

    // Shadowing allows changing type
    let spaces = "   "; // &str
    println!("spaces (string): '{}'", spaces);

    let spaces = spaces.len(); // Now usize
    println!("spaces (length): {}", spaces);

    // ========================================
    // SCOPE
    // ========================================
    let outer = "I'm outside";

    {
        let inner = "I'm inside";
        println!("In block: outer = '{}', inner = '{}'", outer, inner);

        // Shadowing in inner scope
        let outer = "I'm shadowed outer";
        println!("Shadowed outer = '{}'", outer);
    }

    // inner is now out of scope
    println!("After block: outer = '{}'", outer); // Original outer

    // ========================================
    // DECLARATION PATTERNS
    // ========================================

    // With type annotation
    let explicit: i32 = 42;
    println!("explicit: {}", explicit);

    // Tuple destructuring
    let (a, b, c) = (1, 2.0, "three");
    println!("a = {}, b = {}, c = {}", a, b, c);

    // Underscore for unused
    let _unused = "I won't cause a warning";

    // Declare now, assign later
    let later;
    later = "assigned later";
    println!("later: {}", later);
}
