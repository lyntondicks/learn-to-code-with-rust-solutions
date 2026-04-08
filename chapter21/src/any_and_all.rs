pub fn any_and_all() {
    println!("Chapter 21: Any and All");

    let even_numbers = [2, 4, 6, 8, 10];
    // The "|&x|" below dereferences the reference to get the value
    let are_all_even = even_numbers.iter().all(|&x| x % 2 == 0);
    let are_any_odd = even_numbers.iter().any(|x| x % 2 != 0);
    println!("All numbers are even: {}", are_all_even);
    println!("Any number is odd: {}", are_any_odd);
}
