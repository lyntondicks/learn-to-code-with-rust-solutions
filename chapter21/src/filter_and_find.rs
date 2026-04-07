pub fn filter_and_find() {
    println!("Chapter 21: Filter and Find");
    filter_and_find_simple_types();
}

pub fn filter_and_find_simple_types() {
    let numbers = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    let evens: Vec<i32> = numbers.iter().filter(|x| *x % 2 == 0).copied().collect();
    println!("Even numbers: {:?}", evens);
    println!("Numbers: {:?}", numbers);

    let first_even = numbers.into_iter().find(|x| x % 2 == 0);
    println!("First even number: {:?}", first_even);
    println!("Numbers: {:?}", numbers); // numbers is still usable here because [i32] implements Copy

    let first_odd = numbers.into_iter().find(|x| x % 2 != 0);
    println!("First odd number: {:?}", first_odd);

    let nothing = numbers.into_iter().find(|x| *x > 100);
    println!("Nothing: {:?}", nothing);

    let last_even = numbers.into_iter().rfind(|x| x % 2 == 0);
    println!("Last even number: {:?}", last_even);
}
