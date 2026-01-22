fn main() {
    // ========================================
    // INTEGER TYPES
    // ========================================
    let a: i8 = -128; // Signed 8-bit: -128 to 127
    let b: u8 = 255; // Unsigned 8-bit: 0 to 255
    let c: i32 = -2_147_483_648; // Signed 32-bit (default)
    let d: u64 = 18_446_744_073_709_551_615; // Unsigned 64-bit
    let e: isize = -100; // Architecture-dependent signed
    let f: usize = 100; // Architecture-dependent unsigned

    println!("=== Integers ===");
    println!("i8:    {}", a);
    println!("u8:    {}", b);
    println!("i32:   {}", c);
    println!("u64:   {}", d);
    println!("isize: {}", e);
    println!("usize: {}", f);

    // Integer literals
    let decimal = 1_000_000; // Underscores for readability
    let hex: u32 = 0xCAFE_BABE; // Hexadecimal
    let octal = 0o755; // Octal (file permissions style)
    let binary = 0b1010_1010; // Binary
    let byte = b'A'; // Byte (u8) from ASCII

    println!("\n=== Integer Literals ===");
    println!("decimal: {}", decimal);
    println!("hex:     {} (0xCAFE_BABE)", hex);
    println!("octal:   {} (0o755)", octal);
    println!("binary:  {} (0b1010_1010)", binary);
    println!("byte:    {} (b'A')", byte);

    // ========================================
    // FLOATING-POINT TYPES
    // ========================================
    let g = 2.0; // f64 (default)
    let h: f32 = 3.14159; // f32
    let pi: f64 = std::f64::consts::PI;

    println!("\n=== Floats ===");
    println!("f64 (default): {}", g);
    println!("f32:           {}", h);
    println!("PI constant:   {}", pi);

    // ========================================
    // BOOLEAN TYPE
    // ========================================
    let is_rust_awesome = true;
    let is_hard: bool = false;

    println!("\n=== Booleans ===");
    println!("is_rust_awesome: {}", is_rust_awesome);
    println!("is_hard: {}", is_hard);

    // ========================================
    // CHARACTER TYPE
    // ========================================
    let letter = 'A';
    let emoji = '🦀'; // Rust mascot: Ferris the crab!
    let chinese = '中';
    let heart = '❤';

    println!("\n=== Characters (4 bytes each, Unicode) ===");
    println!("letter:  {}", letter);
    println!("emoji:   {}", emoji);
    println!("chinese: {}", chinese);
    println!("heart:   {}", heart);

    // ========================================
    // TUPLE TYPE
    // ========================================
    let tup: (i32, f64, char) = (500, 6.4, '!');

    // Destructuring
    let (x, y, z) = tup;
    println!("\n=== Tuples ===");
    println!("Destructured: x={}, y={}, z={}", x, y, z);

    // Index access
    println!("tup.0 = {}", tup.0);
    println!("tup.1 = {}", tup.1);
    println!("tup.2 = {}", tup.2);

    // Unit type (empty tuple)
    let unit: () = ();
    println!("Unit type: {:?}", unit);

    // ========================================
    // ARRAY TYPE
    // ========================================
    let arr = [1, 2, 3, 4, 5]; // [i32; 5]
    let months = [
        "Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec",
    ];
    let zeros = [0; 5]; // [0, 0, 0, 0, 0]

    println!("\n=== Arrays ===");
    println!("arr: {:?}", arr);
    println!("arr[0] = {}, arr[4] = {}", arr[0], arr[4]);
    println!("months: {:?}", months);
    println!("First month: {}", months[0]);
    println!("zeros: {:?}", zeros);
    println!("Array length: {}", arr.len());

    // ========================================
    // TYPE CONVERSION
    // ========================================
    let integer: i32 = 42;
    let float: f64 = integer as f64;
    let back: i32 = 3.9_f64 as i32; // Truncates!

    println!("\n=== Type Conversion ===");
    println!("i32 {} -> f64 {}", integer, float);
    println!("f64 3.9 -> i32 {} (truncated!)", back);

    // ========================================
    // NUMERIC OPERATIONS
    // ========================================
    println!("\n=== Numeric Operations ===");
    println!("5 + 10 = {}", 5 + 10);
    println!("95.5 - 4.3 = {}", 95.5 - 4.3);
    println!("4 * 30 = {}", 4 * 30);
    println!("56.7 / 32.2 = {}", 56.7 / 32.2);
    println!("5 / 3 = {} (integer division)", 5 / 3);
    println!("43 % 5 = {} (remainder)", 43 % 5);

    // ========================================
    // TYPE INFERENCE EXAMPLES
    // ========================================
    let inferred_int = 100; // i32
    let inferred_float = 3.14; // f64
    let inferred_bool = true; // bool
    let inferred_char = 'x'; // char

    println!("\n=== Type Inference ===");
    println!(
        "Rust infers: int={}, float={}, bool={}, char={}",
        inferred_int, inferred_float, inferred_bool, inferred_char
    );
}
