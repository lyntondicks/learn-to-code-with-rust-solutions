pub fn flat_map() {
    println!("Chapter 21: Flat Map");

    let attendees = ["Bob, Mary, Keven", "Mike, Robbie, Mat", "Piers, Liam"];

    let attendees: Vec<&'static str> = attendees
        .iter()
        .flat_map(|group| group.split(", "))
        .collect();

    println!("Attendees: {:?}", attendees);
}
