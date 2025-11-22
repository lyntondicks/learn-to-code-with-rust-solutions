use std::collections::HashMap;

pub fn hash_maps() {
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

    // get wont panic if key doesn't exist.
    // copied() method is used to create a copy of the value associated with the key.
    // so that Option<&&str> is turned into Option<&str>
    let value: Option<&str> = coffee_pairings.get("Cappuccino").copied();
    println!(
        "Cappuccino is paired with: {}",
        value.unwrap_or("No pairing found")
    );

    // .entry() method returns an Entry enum which can be used to insert or update a value
    coffee_pairings.entry("Cappuccino").or_insert("Almond Milk"); // will not replace existing value
    println!("{:#?}", coffee_pairings);
}
