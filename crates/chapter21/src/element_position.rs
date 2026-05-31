#[allow(clippy::iter_nth)]
pub fn element_position() {
    println!("Chapter 21: Element Position");

    let performers = ["Rustful Five", "Rust in Peace", "Rustin Bieber"];

    let last = performers.iter().last().unwrap();
    println!("{last}");

    let second = performers.iter().nth(1).unwrap();
    println!("nth {second}");
    // Clippy: let second = performers.get(1).unwrap();

    let second_to_last = performers.iter().nth_back(1).unwrap();
    println!("nth_back {second_to_last}");

    let target_index = performers
        .iter()
        .position(|element| *element == "Rustin Bieber");
    println!("{target_index:?}");
}
