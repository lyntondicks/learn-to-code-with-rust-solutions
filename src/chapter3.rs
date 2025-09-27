#![allow(dead_code, unused_variables)]

pub fn main() {
    println!("Chapter 3: Data types");
    data_types();
    casting();
    operators();
    conditionals();
    arrays();
    tuples();
    range();
}

fn data_types() {
    let eight_bit: i8 = -122;
    let eight_bit_unsigned: u8 = 255;
    let sixteen_bit: i16 = -32768;
    let type_assert = 20u16;
    let sixteen_bit_signed: i16 = -32_768;
    let days: usize = 365; // usize is platform-dependent, typically 32 or 64 bits
    let years: isize = -1000; // isize is also platform-dependent, typically 32 or 64 bits

    println!("Dear Emily,\nHow have you been?");
    println!("\tThis is a tabbed line.");

    let raw_string = r"This is a raw string.\t\\r\nIt can contain special characters like \n and \t without escaping.";

    let value_methods: i32 = -15;
    println!("{}", value_methods.abs()); // Using the abs() method to get the absolute value

    let empty_space = "      my content     ";
    println!("Trimmed content: '{}'", empty_space.trim()); // Using trim() to remove whitespace
    println!("{}", value_methods.pow(2)); // Using the pow() method to raise to the power of 2

    let pi: f64 = 3.1415926535897932384; // Floating-point number
    println!("Pi rounded down: {}", pi.floor()); // Using floor() to round down
    println!("Pi rounded up: {}", pi.ceil()); // Using ceil() to round up
    println!("Pi rounded: {}", pi.round()); // Using round() to round to the nearest integer
    println!("The current value of pi is {pi:.2}"); // Using format specifier to limit decimal places

    let is_raining: bool = true; // Boolean type

    let first_initial: char = 'A'; // Character type. UTF-8
    let emoji = '🎧';
    println!(
        "Is initial alphabetic: {}. Is emoji {emoji} alphabetic: {}",
        first_initial.is_alphabetic(),
        emoji.is_alphabetic()
    );
    println!(
        "Is initial uppercase: {}. Is emoji uppercase: {}",
        first_initial.is_uppercase(),
        emoji.is_uppercase()
    );
}

fn casting() {
    let miles_away = 50;
    let miles_away_i8 = miles_away as i8; // Casting from i32 to i8
    let integer: i32 = 10;
    let float: f64 = integer as f64; // Casting from i32 to f64
    println!("Integer as float: {}", float);

    let large_number: u64 = 1_000_000_000_000;
    let small_number: u32 = large_number as u32; // Casting from u64 to u32, may lose data if too large
    println!("Large number casted to small number: {}", small_number);
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