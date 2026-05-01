pub fn reduce() {
    println!("Chapter 21: Reduce");

    let earnings: [i32; 0] = [];

    let sum = earnings
        .into_iter()
        .reduce(|total, current| total + current);
    println!("{sum:?}");

    let address_portions = [
        String::from("123 Elm Street"),
        String::from("Suburbia"),
        String::from("New Jersey"),
    ];

    // 123 Elm Street, Suburbia, New Jersey
    println!("{}", address_portions.join(", "));

    let address = address_portions.into_iter().reduce(|mut acc, portion| {
        acc.push_str(", ");
        acc.push_str(&portion);
        acc
    });
    println!("{address:?}");
}
