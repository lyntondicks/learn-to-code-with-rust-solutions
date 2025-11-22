use std::collections::{HashMap, HashSet};

pub fn chapter16() {
    let mut menu: HashMap<String, f64> = HashMap::new();
    menu.insert(String::from("Burger"), 5.99);
    menu.insert(String::from("Fries"), 2.99);
    menu.insert(String::from("Soda"), 1.49);

    for (item, price) in &menu {
        println!("{}: ${:.2}", item, price);
    }

    let data = [
        ("USA", "Washington, D.C."),
        ("Canada", "Ottawa"),
        ("UK", "London"),
    ];
    let mut country_capitals = HashMap::<&str, &str>::from(data);
    country_capitals.insert("France", "Paris");
    println!("{:#?}", country_capitals);
    let uk_capital = country_capitals.remove("UK");
    println!("Some UK Capital: {:?}", uk_capital);
    println!("UK Capital: {:?}", uk_capital.unwrap());
    println!("Length: {}", country_capitals.len());

    // ownership
    let mut coffee_pairings: HashMap<&str, &str> = HashMap::new();
    let drink = String::from("Latte");
    let milk = String::from("Oat Milk");
    coffee_pairings.insert(&drink, &milk); // deref coercion; &String -> &str
    coffee_pairings.insert("Espresso", "Whole Milk");
    coffee_pairings.insert("Latte", "Pistachio Milk"); // replace existing key/value for Latte
    println!("{drink} {milk}"); // can access drink and milk because using reference instead of taking ownership

    let value = coffee_pairings["Espresso"]; // will panic if key doesn't exist
    println!("Espresso is paired with: {}", value);

    let value = coffee_pairings.get("Cappuccino").copied();
    println!(
        "Cappuccino is paired with: {}",
        value.unwrap_or("No pairing found")
    );

    coffee_pairings.entry("Cappuccino").or_insert("Almond Milk"); // will not replace existing value
    println!("{:#?}", coffee_pairings);

    let mut concert_queue: HashSet<&str> = HashSet::new();
    concert_queue.insert("Alice");
    concert_queue.insert("Bob");
    concert_queue.insert("Charlie");
    println!("{:#?}", concert_queue);
    println!("{}", concert_queue.len());
    concert_queue.insert("Alice");
    println!("{:#?}", concert_queue);
    concert_queue.remove("Bob");
    println!("{:#?}", concert_queue);

    println!("{}", concert_queue.contains("Alice"));
    println!("{:#?}", concert_queue.get("Charlie"));

    let mut movie_queue: HashSet<&str> = HashSet::new();
    movie_queue.insert("Alice");
    movie_queue.insert("Bob");
    movie_queue.insert("Charlie");
    movie_queue.insert("David");

    println!("{:#?}", concert_queue.union(&movie_queue));
    println!("{:#?}", concert_queue.difference(&movie_queue));
    println!("{:#?}", concert_queue.symmetric_difference(&movie_queue));
    println!("{:#?}", concert_queue.is_disjoint(&movie_queue)); // sets have no elements in common
    println!("{:#?}", concert_queue.is_subset(&movie_queue)); // all elements in concert_queue are also in movie_queue
    println!("{:#?}", movie_queue.is_superset(&concert_queue)); // all elements in movie_queue are also in concert_queue

    chapter_16_project_solution();
}

fn chapter_16_project_solution() {
    let mut sauces_to_meals = HashMap::from([
        ("Ketchup", vec!["French Fries", "Burgers", "Hot Dogs"]),
        ("Mayonnaise", vec!["Sandwiches", "Burgers", "Coleslaw"]),
    ]);

    sauces_to_meals.insert("Mustard", vec!["Hot Dog", "Burgers", "Pretzels"]);
    println!("{:#?}", sauces_to_meals.remove("Mayonnaise").unwrap());

    let mustard_meals = sauces_to_meals.get("Mustard");
    match mustard_meals {
        Some(meals) => println!("Mustard goes well with: {:?}", meals),
        None => println!("No meals found for Mustard"),
    }

    sauces_to_meals
        .entry("Soy Sauce")
        .or_insert(vec!["Sushi", "Dumplings"]);
    println!("{:#?}", sauces_to_meals);
}
