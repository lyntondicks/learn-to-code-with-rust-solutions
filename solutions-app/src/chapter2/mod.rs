#![allow(unused_variables)]
#![allow(dead_code)]
const TAX_RATE: f64 = 0.07; // Constant for tax rate, accessible throughout the file

type Meters = i32; // Type alias

// rustfmt a file
// cargo fmt all files in the project

/*
Multi-line comment
 */

// #[allow(unused_variables)]
pub fn main() {
    println!("Chapter 2");

    print!("1.");
    print!("2.");
    println!("This is a simple Rust program.");
    let apples_in_garden = 50;
    let oranges = 14 + 6;
    let fruits = apples_in_garden + oranges;
    println!("Total fruits in the garden: {}", fruits - 10);
    println!("Oranges in the garden: {oranges} and apples: {apples_in_garden}");
    println!(
        "Apples: {0}, Oranges: {1}. These {0} apples are tasty",
        apples_in_garden, oranges
    );
    let _unused_variable = 42; // This variable is not used anywhere, so prefix with _ to avoid warnings
    // Variables are limited to function scope, but constants can be used at the file level

    // Using a mutable variable
    let mut gym_reps = 10;
    gym_reps += 5; // Incrementing the number of reps
    println!("Total gym reps: {gym_reps}");

    // Variable shadowing
    let grams_of_protein: &'static str = "100.345";
    let grams_of_protein = 100.345;
    let mut grams_of_protein = 100;
    grams_of_protein += 50; // Incrementing the grams of protein
    println!("Grams of protein: {grams_of_protein}");
    // nested block scope
    {
        let grams_of_protein = 200; // Separate scope, so separate variable from the outer one
        println!("Shadowed grams of protein: {grams_of_protein}");
        #[allow(unused_variables)]
        let cookie_price = 1.99; // This variable is not used outside this block and is not accessible.
    }

    let multiplier = 3;
    // Using a block expression to calculate a value
    let calculation = {
        let value = 5 + 4;
        value * multiplier
    };

    let mile_race_length: Meters = 1600;
    let two_mile_race_length: Meters = 3200;
}
