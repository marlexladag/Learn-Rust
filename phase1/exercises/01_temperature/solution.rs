// Solution: Temperature Converter

fn main() {
    println!("=== Temperature Converter ===\n");

    println!("Celsius to Fahrenheit:");
    println!("0°C = {:.1}°F", celsius_to_fahrenheit(0.0));
    println!("100°C = {:.1}°F", celsius_to_fahrenheit(100.0));
    println!("-40°C = {:.1}°F", celsius_to_fahrenheit(-40.0));
    println!("37°C = {:.1}°F", celsius_to_fahrenheit(37.0));

    println!("\nFahrenheit to Celsius:");
    println!("32°F = {:.1}°C", fahrenheit_to_celsius(32.0));
    println!("212°F = {:.1}°C", fahrenheit_to_celsius(212.0));
    println!("-40°F = {:.1}°C", fahrenheit_to_celsius(-40.0));
    println!("98.6°F = {:.1}°C", fahrenheit_to_celsius(98.6));
}

/// Converts Celsius to Fahrenheit
/// Formula: F = (C × 9/5) + 32
fn celsius_to_fahrenheit(celsius: f64) -> f64 {
    (celsius * 9.0 / 5.0) + 32.0
}

/// Converts Fahrenheit to Celsius
/// Formula: C = (F - 32) × 5/9
fn fahrenheit_to_celsius(fahrenheit: f64) -> f64 {
    (fahrenheit - 32.0) * 5.0 / 9.0
}
