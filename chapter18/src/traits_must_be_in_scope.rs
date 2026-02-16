use std::ops::Add;
use std::str::FromStr;

pub fn traits_must_be_in_scope() {
    let a: i32 = 5;
    let b: i32 = 10;
    let sum = a.add(b); // Add trait must be in scope for this to work
    println!("Sum: {}", sum);

    let numeric_count = u64::from_str("5");
    println!("Numeric count: {:?}", numeric_count.unwrap()); // FromStr trait must be in scope for this to work
}
