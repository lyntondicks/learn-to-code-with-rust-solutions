pub fn iter() {
    println!("Chapter 21: Iter");

    let my_vector: Vec<i32> = (1..=6).collect();
    // let my_iterator = my_vector.iter();
    // for item in my_iterator {
    //     println!("{}", item);
    // }

    for number in &my_vector {
        // borrowing will automatically call .iter()
        println!("{number}");
    }

    println!("{my_vector:?}"); // ownership is still preserved

    let cities = vec![String::from("Phoenix"), String::from("Dallas")];
    for city in &cities {
        println!("{city}");
    }
}
