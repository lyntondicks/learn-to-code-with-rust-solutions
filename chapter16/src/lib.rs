mod hash_maps;

use hash_maps::hash_maps;
use std::collections::{HashMap, HashSet};

pub fn chapter16() {
    hash_maps();
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
