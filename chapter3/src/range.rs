pub fn main() {
    range();
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
