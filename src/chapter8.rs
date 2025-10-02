pub fn main() {
    println!("Chapter 8: Slices");
    string_slices();
    array_slice();
    chapter8_project_solution();
}

fn string_slices() {
    let literal_action_hero: &str = "Arnold Schwarzenegger"; // String literal, stored in program binary
    let _full_slice = &literal_action_hero[..];
    println!("{literal_action_hero} is an action hero. ");
    let action_hero = String::from("Arnold Schwarzenegger");
    let first_name = &action_hero[..6]; // Slicing the string
    println!("{first_name}");

    let last_name = &action_hero[7..];
    println!("{last_name}");

    let block_first_name = {
        let action_hero = "Arnold Schwarzenegger";
        &action_hero[..6] // Slicing the string within a block
    };
    println!(
        "Block first name: {block_first_name}, byte length: {}",
        block_first_name.len()
    );
    dereference_coercion();
}

fn dereference_coercion() {
    let action_hero = String::from("Arnold Schwarzenegger");
    let hero_ref: &str = &action_hero; // Coercing &String to &str
    println!("Hero reference: {}", hero_ref);

    dereference_coercion_str(&action_hero); // Passing the reference to a function that expects a &str, coercion occurs here
    let literal_hero: &str = "Arnold Schwarzenegger";
    dereference_coercion_str(literal_hero); // Passing a string literal directly
}

fn dereference_coercion_str(value: &str) {
    println!("Dereferenced value: {}", value);
}

fn array_slice() {
    let numbers: [isize; 6] = [1, 2, 3, 4, 5, 6];
    let slice: &[isize] = &numbers[1..4]; // Slicing the array
    println!("Slice of numbers: {:#?}", slice); // Printing the slice

    let full_reference: &[isize; 6] = &numbers; // Full reference to the array
    println!("Full reference to numbers: {:#?}", full_reference); // Printing the full reference

    let slice_of_three: &[isize] = &numbers[..3]; // Slicing the first three elements
    println!("Slice of first three numbers: {:#?}", slice_of_three); // Printing the slice of first three numbers

    print_array_length(full_reference); // Passing the full reference to a function that prints the length of the array slice
    print_array_length(slice_of_three); // Passing the slice of three elements to the same function
    print_array_length(&numbers); // Passing the entire array as a slice to the function

    mutable_array_slice(); // Calling the function to demonstrate mutable array slices
}

fn print_array_length(reference: &[isize]) {
    println!("Array length: {}", reference.len()); // Printing the length of the array slice
}

fn mutable_array_slice() {
    let mut numbers: [isize; 6] = [1, 2, 3, 4, 5, 6];
    let slice: &mut [isize] = &mut numbers[1..4]; // Mutable slice of the array
    println!("Mutable slice of numbers before modification: {:#?}", slice); // Printing the mutable slice

    for number in slice.iter_mut() {
        *number += 10; // Modifying each element in the mutable slice
    }
    println!("Mutable slice of numbers after modification: {:#?}", slice); // Printing the modified mutable slice

    slice[0] = 100; // Changing the first element of the mutable slice
}

fn chapter8_project_solution() {
    let mut cereals: [String; 5] = [
        String::from("Cookie Crisp"),
        String::from("Cinnamon Toast Crunch"),
        String::from("Frosted Flakes"),
        String::from("Cocoa Puffs"),
        String::from("Captain Crunch"),
    ];

    let first_two: &[String] = &cereals[..2];
    println!("First two cereals: {first_two:#?}");

    let mid_three: &[String] = &cereals[1..4];
    println!("Middle three cereals: {mid_three:#?}");

    let last_three: &mut [String] = &mut cereals[2..];
    println!("Last three cereals: {last_three:#?}");

    last_three[2] = String::from("Lucky Charms");

    let cookie_crisp: &String = &cereals[0];
    let cookie = &cookie_crisp[0..5];
    println!("Cookie Crisp: {cookie_crisp}, Cookie: {cookie}");

    let cocoa_puffs: &String = &cereals[3];
    let puffs: &str = &cocoa_puffs[6..];
    println!("Cocoa Puffs: {cocoa_puffs}, Puffs: {puffs}");
}