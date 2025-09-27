pub fn main() {
    tuples();
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
