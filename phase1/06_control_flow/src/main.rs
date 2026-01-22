fn main() {
    // ========================================
    // IF EXPRESSIONS
    // ========================================
    println!("=== if Expressions ===");

    let number = 7;

    if number < 5 {
        println!("{} is less than 5", number);
    } else if number > 10 {
        println!("{} is greater than 10", number);
    } else {
        println!("{} is between 5 and 10", number);
    }

    // if as expression (returns value)
    let condition = true;
    let value = if condition { 5 } else { 6 };
    println!("Conditional value: {}", value);

    // Ternary-style usage
    let abs_value = if number < 0 { -number } else { number };
    println!("Absolute value of {}: {}", number, abs_value);

    // ========================================
    // LOOP - INFINITE LOOP
    // ========================================
    println!("\n=== loop ===");

    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 5 {
            break counter * 2; // Return value from loop
        }
    };
    println!("Loop result: {} (counter was {})", result, counter);

    // Loop with labels
    println!("\nNested loops with labels:");
    let mut count = 0;
    'outer: loop {
        println!("  Outer loop");
        let mut remaining = 3;

        'inner: loop {
            println!("    Inner loop, remaining: {}", remaining);
            if remaining == 0 {
                break 'inner;
            }
            if count == 4 {
                println!("    Breaking outer!");
                break 'outer;
            }
            remaining -= 1;
            count += 1;
        }
    }
    println!("Final count: {}", count);

    // ========================================
    // WHILE LOOP
    // ========================================
    println!("\n=== while loop ===");

    let mut countdown = 5;
    while countdown > 0 {
        print!("{}... ", countdown);
        countdown -= 1;
    }
    println!("Liftoff!");

    // ========================================
    // FOR LOOP
    // ========================================
    println!("\n=== for loop ===");

    // Iterate over array
    let arr = [10, 20, 30, 40, 50];
    print!("Array elements: ");
    for element in arr {
        print!("{} ", element);
    }
    println!();

    // Range (exclusive end)
    print!("Range 0..5: ");
    for i in 0..5 {
        print!("{} ", i);
    }
    println!();

    // Range (inclusive end)
    print!("Range 1..=5: ");
    for i in 1..=5 {
        print!("{} ", i);
    }
    println!();

    // Reverse range
    print!("Reverse 1..4: ");
    for i in (1..4).rev() {
        print!("{} ", i);
    }
    println!();

    // Enumerate (index + value)
    println!("Enumerated array:");
    for (index, value) in arr.iter().enumerate() {
        println!("  arr[{}] = {}", index, value);
    }

    // ========================================
    // MATCH EXPRESSION
    // ========================================
    println!("\n=== match expression ===");

    let dice_roll = 4;
    match dice_roll {
        1 => println!("You rolled a one!"),
        2 => println!("You rolled a two!"),
        3 => println!("You rolled a three!"),
        4 | 5 | 6 => println!("You rolled {}", dice_roll),
        _ => println!("Invalid dice roll!"),
    }

    // Match with ranges
    let age = 25;
    let life_stage = match age {
        0..=12 => "child",
        13..=19 => "teenager",
        20..=64 => "adult",
        65..=120 => "senior",
        _ => "unknown",
    };
    println!("Age {}: {}", age, life_stage);

    // Match with guards
    let temperature = 72;
    let comfort = match temperature {
        t if t < 50 => "cold",
        t if t < 70 => "cool",
        t if t < 80 => "comfortable",
        t if t < 90 => "warm",
        _ => "hot",
    };
    println!("{}°F feels {}", temperature, comfort);

    // Match returning values
    let boolean = true;
    let binary = match boolean {
        true => 1,
        false => 0,
    };
    println!("Boolean {} as binary: {}", boolean, binary);

    // ========================================
    // IF LET
    // ========================================
    println!("\n=== if let ===");

    let some_number = Some(42);

    // Verbose match
    match some_number {
        Some(n) => println!("Match found: {}", n),
        None => (),
    }

    // Cleaner if let
    if let Some(n) = some_number {
        println!("If let found: {}", n);
    }

    // if let with else
    let no_number: Option<i32> = None;
    if let Some(n) = no_number {
        println!("Found: {}", n);
    } else {
        println!("No number found!");
    }

    // ========================================
    // WHILE LET
    // ========================================
    println!("\n=== while let ===");

    let mut stack = vec![1, 2, 3, 4, 5];
    print!("Popping stack: ");
    while let Some(top) = stack.pop() {
        print!("{} ", top);
    }
    println!("(empty)");

    // ========================================
    // BREAK AND CONTINUE
    // ========================================
    println!("\n=== break and continue ===");

    print!("Skip 3, stop at 7: ");
    for i in 0..10 {
        if i == 3 {
            continue; // Skip this iteration
        }
        if i == 7 {
            break; // Exit loop
        }
        print!("{} ", i);
    }
    println!();

    // ========================================
    // PRACTICAL EXAMPLE: FizzBuzz
    // ========================================
    println!("\n=== FizzBuzz (1-20) ===");
    for n in 1..=20 {
        let result = match (n % 3, n % 5) {
            (0, 0) => String::from("FizzBuzz"),
            (0, _) => String::from("Fizz"),
            (_, 0) => String::from("Buzz"),
            _ => n.to_string(),
        };
        print!("{} ", result);
    }
    println!();
}
