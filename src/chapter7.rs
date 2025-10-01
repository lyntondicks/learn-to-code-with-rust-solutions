#![allow(dead_code, unused_variables, unused_mut)]

pub fn main() {
    println!("Chapter 7: References and Borrowing");
    mutable_reference_restrictions();
    dangling_reference();
    array_and_tuple_ownership(); 
}

fn mutable_reference_restrictions() {
    let mut coffee = String::from("Mocha");
    let a = &mut coffee; // Mutable reference to coffee
    println!("{a}");
    let b = a;
    println!("{b}"); // b is a mutable reference to coffee
    // Lifetime of mutable references is restricted to one mutable reference at a time
    // Multiple immutable references can exist at the same time, but not mutable and immutable references together
}

fn dangling_reference() {
    let _dangling_ref: &String;
    {
        let temp = String::from("Hello");
        _dangling_ref = &temp; // dangling_ref points to temp
    } // temp goes out of scope here
    // println!("Dangling reference: {}", dangling_ref); // This would cause a compile-time error
}

fn array_and_tuple_ownership() {
    let registrations = [true, false, true];
    let first = registrations[0];
    println!("{first} and {registrations:#?}");

    let languages = [String::from("Rust"), String::from("Javascript")];
    let first = languages[0].clone(); // Cloning the first element to avoid moving it
    let second = &languages[1]; // Borrowing the second element
    println!("First language: {first}, Second language: {second}");

    let tuple = (true, 42, String::from("Hello"));
    let first = tuple.0; // Accessing the first element, copy trait
    let second = tuple.1; // Accessing the second element, copy trait
    let third = &tuple.2; // Borrowing the third element, heap memory, no move occurs.
    let third_clone = third.clone(); // Cloning the third element to avoid moving it
}