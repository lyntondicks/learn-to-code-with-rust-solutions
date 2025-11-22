mod hash_maps;
mod hash_sets;

use std::collections::HashMap;

use hash_maps::hash_maps;
use hash_sets::hash_sets;

pub fn chapter16() {
    hash_maps();
    hash_sets();

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
