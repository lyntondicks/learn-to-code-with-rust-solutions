// when there is only one lifetime parameter, that lifetime is assigned to all output lifetime parameters.
// You can omit the lifetime parameter annotation in this case, but is shown here for example.
#[allow(clippy::needless_lifetimes)]
fn select_first_two_elements<'a>(items: &'a [String]) -> &'a [String] {
    // deref coercion to a slice of a collection of Strings
    let selected_items = &items[..2];
    println!("{selected_items:?}");
    selected_items
}

pub fn references_as_function_parameters() {
    println!("References as function parameters");

    let cities = vec![
        String::from("London"),
        String::from("Paris"),
        String::from("New York"),
    ];

    let two_cities = {
        let cities_reference = &cities;
        select_first_two_elements(cities_reference)
    }; // cities_reference goes out of scope here, lifetime 'a refers to the value, cities, not the reference
    // drop(cities);    // error: borrow of moved value: `cities`

    println!("{two_cities:?}");

    {
        let coffees = [String::from("Latte"), String::from("Mocha")];
        let two_coffees = select_first_two_elements(&coffees);
        println!("{two_coffees:?}");
    }
}
