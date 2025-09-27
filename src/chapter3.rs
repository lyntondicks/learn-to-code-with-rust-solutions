#![allow(dead_code, unused_variables)]
mod conditionals;
mod data_types;
mod operators;

pub fn main() {
    println!("Chapter 3: Data types");
    data_types::main();
    operators::main();
    conditionals::main();
    arrays();
    tuples();
    range();
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
