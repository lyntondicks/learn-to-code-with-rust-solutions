#[allow(clippy::iter_cloned_collect, clippy::map_clone, clippy::clone_on_copy)]
pub fn cloned() {
    println!("Chapter 21: Cloned");

    // copied vs cloned. Copied is for types that implement Copy trait, cloned is for types that implement Clone trait, normally for heap allocated data.
    let numbers = [1, 2, 3, 4, 5];
    let copied_numbers: Vec<i32> = numbers.iter().copied().collect(); // cloned works here too, also "numbers.to_vec()"
    println!("Copied numbers: {:?}", copied_numbers);
    println!("Original numbers: {:?}", numbers);

    let strings = vec!["one".to_string(), "two".to_string(), "three".to_string()];
    let cloned_strings: Vec<String> = strings.iter().cloned().collect();
    println!("Cloned strings: {:?}", cloned_strings);
    println!("Original strings: {:?}", strings);

    // equivalent with map, manual clone
    let mapped_cloned_strings: Vec<String> = strings.iter().map(|s| s.clone()).collect();
    println!("Mapped cloned strings: {:?}", mapped_cloned_strings);

    // array can also be cloned, which clones each element
    let cloned_array: [i32; 5] = numbers.clone();
    println!("Cloned array: {:?}", cloned_array);

    // it is more efficient to filter before cloning so that we don't clone unnecessary elements
    let even_cloned_numbers: Vec<i32> = numbers.iter().filter(|&&x| x % 2 == 0).cloned().collect();
    println!("Even cloned numbers: {:?}", even_cloned_numbers);
}
