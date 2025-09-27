#![allow(dead_code, unused_variables)]
mod arrays;
mod conditionals;
mod data_types;
mod operators;

pub fn main() {
    println!("Chapter 3: Data types");
    data_types::main();
    operators::main();
    conditionals::main();
    arrays::main();
    tuples();
    range();
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
