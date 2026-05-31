pub fn map_adapter_and_collect() {
    println!("Chapter 21: Map Adapter and Collect");

    let numbers = vec![1, 2, 3, 4, 5];
    let my_iterator = numbers.iter();
    let squares_iter = my_iterator.map(|x: &i32| x.pow(2));

    // prints "Map { iter: Iter([1, 2, 3, 4, 5]) }" because it's lazy.
    // Map is an iterator adapter that returns a new iterator.
    println!("{squares_iter:?}");

    // collect() consums the iterator and produces a collection, in this case a Vec<i32>
    let squares: Vec<i32> = squares_iter.collect();
    // let squares: Vec<_> = squares_iter.collect(); // type can be inferred
    // // turbo fish
    // let squares = squares_iter.collect::<Vec<i32>>();
    println!("{squares:?}"); // prints "[1, 4, 9, 16, 25]"

    // original vector is unchanged: "[1, 2, 3, 4, 5]"
    println!("{numbers:?}");

    let names = [
        String::from("Alice"),
        String::from("Bob"),
        String::from("Cathy"),
    ];

    let name_lengths = names
        .iter()
        .map(|name| name.to_lowercase())
        .map(|name| name.replace("i", "@@"))
        .map(|name| name.len())
        .collect::<Vec<usize>>();
    println!("{name_lengths:?}"); // [6, 3, 5]
}
