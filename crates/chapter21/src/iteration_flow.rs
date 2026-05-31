pub fn iteration_flow() {
    println!("Chapter 21: Iteration Flow");

    let fifty_numbers = 1..=50;

    // reverses the iterator: fifty_numbers.rev()
    for number in fifty_numbers.clone().skip(5).take(15).step_by(2) {
        print!("{number}/");
    }
    println!();
    println!("{fifty_numbers:?}");
}
