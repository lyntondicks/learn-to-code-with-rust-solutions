#![allow(dead_code, unused_variables)]
mod data_types;

pub fn main() {
    println!("Chapter 3: Data types");
    data_types::main();
    operators();
    conditionals();
    arrays();
    tuples();
    range();
}



#[allow(unused_assignments)]
fn operators() {
    let a = 10;
    let b = 20;
    let sum = a + b; // Addition
    let difference = b - a; // Subtraction
    let product = a * b; // Multiplication
    let quotient = b / a; // Division
    let remainder = b % a; // Modulus

    println!(
        "Sum: {}, Difference: {}, Product: {}, Quotient: {}, Remainder: {}",
        sum, difference, product, quotient, remainder
    );

    let floor_division = 7 / 2; // Integer division, result is 3
    println!("Floor division of 7 by 2: {}", floor_division);

    let decimal_division = 7.0 / 2.0; // Floating-point division, result is 3.5
    println!("Decimal division of 7.0 by 2.0: {}", decimal_division);

    let mut year = 2025;
    year = year + 1;
    year += 1; // Incrementing the year by 1

    let age: i32 = 21;
    let is_young = age < 30; // Using a comparison operator
    let is_old = !is_young; // Negating the boolean value
    println!("Is the person young? {}", is_young);
    println!("{} {}", age.is_positive(), age.is_negative());
}

fn conditionals() {
    let purchased_ticket = true;
    let plane_on_time = true;
    let making_event = purchased_ticket && plane_on_time;
    if making_event {
        println!("You can attend the event!");
    } else {
        println!("You cannot attend the event.");
    }
}

fn arrays() {
    let mut numbers: [i32; 5] = [1, 2, 3, 4, 5];
    numbers[0] = 10; // Changing the first element
    println!("First number: {}", numbers[0]);
    println!("Array length: {}", numbers.len());
    println!("Array contents: {:?}", numbers); // Using debug format to print the array, debug trait

    let seasons = ["Spring", "Summer", "Autumn", "Winter"];
    println!("Seasons: {seasons:?}");
    println!("Pretty print seasons:\n{seasons:#?}"); // Pretty print the array
    dbg!(seasons); // Using dbg! macro to print the array with debug information
    dbg!(2 + 2); // Using dbg! macro to print an expression and its value
}

fn tuples() {
    let person: (&str, i32, f64) = ("Alice", 30, 5.5); // Tuple with different types
    println!(
        "Name: {}, Age: {}, Height: {}",
        person.0, person.1, person.2
    );

    let (name, age, height) = person; // Destructuring the tuple
    println!(
        "Destructured - Name: {}, Age: {}, Height: {}",
        name, age, height
    );

    println!(
        "Tuples implement the Debug trait, so we can print them directly or use the debug macro: {:#?}",
        person
    );
    dbg!(person); // Using dbg! macro to print the tuple with debug information
}

fn range() -> () {
    let range: std::ops::Range<i32> = 1..31; //   1 to 30
    dbg!(range);
    let range = 1..=31; // 1 to 31 inclusive

    for number in range {
        print!("{number}, ");
    }
    println!(); // New line after printing the range

    let letters = 'a'..='z'; // Range of characters from 'a' to 'z'
    for letter in letters {
        print!("{letter}, ");
    }
    println!(); // New line after printing the letters

    let colors = ["Red", "Green", "Blue"];
    for color in colors {
        print!("{color}, ");
    }
    println!(); // New line after printing the colors
}