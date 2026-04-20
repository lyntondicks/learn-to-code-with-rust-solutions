pub fn partition() {
    println!("Chapter 21: Partition");

    // Partition method splits an iterator into two collections based on a predicate
    let even_odd_numbers = (0..10).collect::<Vec<i32>>();
    let (even, odd): (Vec<i32>, Vec<i32>) = even_odd_numbers.into_iter().partition(|&x| x % 2 == 0);
    println!("Even numbers: {:?}", even);
    println!("Odd numbers: {:?}", odd);
}
