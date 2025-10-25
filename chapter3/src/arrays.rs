pub fn main() {
    arrays();
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
