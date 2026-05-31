pub fn flatten() {
    println!("Chapter 21: Flatten");

    let spreadsheet = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
    let flattened: Vec<i32> = spreadsheet.into_iter().flatten().collect();
    println!("Flattened: {:?}", flattened);
}
