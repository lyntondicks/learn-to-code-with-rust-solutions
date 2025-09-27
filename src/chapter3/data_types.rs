pub fn main() {
  data_types();
  casting();
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