pub fn zip() {
    println!("Chapter 21: Zip");

    let first_names = ["Bob", "Mary", "Keven", "Dan"];
    let last_names = ["Smith", "Johnson", "Williams"];

    for (first, last) in first_names.iter().zip(last_names) {
        println!("Full name: {} {}", first, last);
    }

    let complete_names: Vec<String> = first_names
        .iter()
        .zip(last_names)
        .map(|(first, last)| format!("{} {}", first, last))
        .collect();
    println!("Complete names: {:?}", complete_names);
}
