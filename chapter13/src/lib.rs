mod project_solution;

use project_solution::chapter13_project_solution;

pub fn chapter13() {
    println!("Chapter 13: Vectors");
    chapter13_vectors();
    chapter13_project_solution();
}

fn chapter13_vectors() {
    let mut numbers = Vec::new();
    numbers.push(1);
    numbers.push(2);
    numbers.push(3);
    println!("{:?}", numbers);

    let _pizza_diameters = Vec::<i32>::new();
    let mut pizza_diameters = vec![8, 10, 12, 14];
    pizza_diameters.push(16);
    pizza_diameters.push(18);

    pizza_diameters.insert(0, 4);

    let last_pizza_diameter = pizza_diameters.pop(); // Removes the last element and returns it
    println!("{:?}", last_pizza_diameter);

    let _third_diameter_from_start = pizza_diameters.remove(2);

    println!("{:?}", pizza_diameters);

    let pepperoni = String::from("Pepperoni");
    let mushroom = String::from("Mushroom");
    let sausage = String::from("Sausage");
    let pizza_toppings = vec![pepperoni, mushroom, sausage];

    let _value = pizza_diameters[2]; // i32 implements copy trait, so value is a copy of pizza_diameters[2]
    let reference = &pizza_toppings[2]; // reference to the third element in pizza_toppings, String does not implement copy trait
    println!("{reference}");

    let _pizza_slice = &pizza_diameters[1..3]; // Slicing the vector to get a slice of the second and third elements
    let option = pizza_toppings.get(2); // Some variant with reference String
    match option {
        Some(topping) => println!("The topping is {topping}"),
        None => println!("No topping found"),
    }

    // transfer ownership from the pizza_toppings variable to the delicious_toppings variable
    let mut delicious_toppings = pizza_toppings; // pizza_toppings is now moved, cannot be used after this point

    delicious_toppings[1] = String::from("Olives");
    let target_topping = &mut delicious_toppings[2]; // mutable reference to the third element, does not change ownership
    // due to lifetimes, cannot have mutable and immutable references to the same data at the same time
    // let reference = &delicious_toppings[2];  // Will fail
    target_topping.push_str(" and Meatballs");
    // But previous line is the last time the mutable reference is used, so we can create an immutable reference now
    let _reference = &delicious_toppings[2];
    println!("{:#?}", delicious_toppings);

    let mut seasons: Vec<&str> = Vec::with_capacity(4);
    println!(
        "Length: {}. Capacity: {}",
        seasons.len(),
        seasons.capacity()
    );

    seasons.push("Summer");
    seasons.push("Fall");
    seasons.push("Winter");
    seasons.push("Spring");
    println!(
        "Length: {}. Capacity: {}",
        seasons.len(),
        seasons.capacity()
    );

    seasons.push("Monsoon");
    println!(
        "Length: {}. Capacity: {}",
        seasons.len(),
        seasons.capacity()
    );
}
