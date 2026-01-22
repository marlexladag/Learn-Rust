fn main() {
    // Basic hello world
    println!("Hello, world!");

    // String formatting with {}
    println!("Hello, {}!", "Rust");

    // Multiple values
    println!("{} + {} = {}", 2, 3, 2 + 3);

    // Debug formatting with {:?}
    println!("Debug tuple: {:?}", (1, 2, 3));

    // Pretty debug with {:#?}
    println!("Pretty vector: {:#?}", vec![1, 2, 3]);

    // Named arguments
    println!("{name} is learning {language}", name = "You", language = "Rust");

    // Positional arguments
    println!("{0} {1} {0}", "tick", "tock");

    // Padding and alignment
    println!("Right aligned: {:>10}", "hi");
    println!("Left aligned:  {:<10}", "hi");
    println!("Centered:      {:^10}", "hi");

    // Numbers formatting
    println!("Binary: {:b}, Hex: {:x}, Octal: {:o}", 42, 42, 42);
}
