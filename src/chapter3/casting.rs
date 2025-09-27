pub fn main() {
    casting();
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
