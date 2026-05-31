pub fn enumerate() {
    println!("Chapter 21: Enumerate");

    let applicants = ["Alice", "Bob", "Charlie", "David", "Eve", "Frank", "Grace"];
    let winners = applicants
        .into_iter()
        .enumerate()
        .filter_map(
            |(index, name)| {
                if index % 3 == 0 { Some(name) } else { None }
            },
        )
        .collect::<Vec<&'static str>>();
    println!("Winners: {:?}", winners);
}
