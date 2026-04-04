#[allow(clippy::needless_as_bytes)]
pub fn string_iteration() {
    println!("Chapter 21: String Iteration");

    let seafood = "Oyster🦪";
    for byte in seafood.bytes() {
        println!("{byte}/");
    }
    println!();
    for character in seafood.chars() {
        print!("{character}/");
    }

    println!("Number of bytes: {}", seafood.bytes().len());
    println!("Number of chars: {}", seafood.chars().count());
}
