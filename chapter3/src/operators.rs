pub fn main() {
    operators();
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
