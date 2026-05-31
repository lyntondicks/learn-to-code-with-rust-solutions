use std::io;

pub fn chapter15() {
    let mut full_name = String::from("Sylvester");
    let last_name = String::from("Stallone");
    full_name.push(' '); // Push a single character
    full_name.push_str(&last_name); // &String -> &str; deref coercion
    println!("{full_name}");

    let first_name = String::from("Sylvester ");
    let full_name = first_name + &last_name;
    println!("{full_name}");
    // println!("{first_name}"); // first_name is no longer valid after the + operation; ownership has been transferred, '+' calls String::add

    let first_name = "Sylvester ".to_string(); // format!("Sylvester ");
    let last_name = "Stallone".to_string(); // format!("Stallone");
    let icon = format!("{first_name}{last_name}");
    println!("{icon}, {first_name}: {last_name}");

    let mut music_genres = "     Jazz, Blues, Rock, Classical    ";
    println!("{}", music_genres.trim()); // trim whitespace from both ends
    println!("{}", music_genres.trim_start()); // trim whitespace from start
    println!("{}", music_genres.trim_end()); // trim whitespace from end
    music_genres = music_genres.trim(); // reassign trimmed value
    println!("{}", music_genres.to_uppercase()); // convert to uppercase
    println!("{}", music_genres.to_lowercase()); // convert to lowercase
    println!("{}", music_genres.replace("a", "@"));
    let genres: Vec<&str> = music_genres.split(", ").collect();
    println!("{:#?}", genres);

    let mut name = String::new();
    println!("What is your name?");
    match io::stdin().read_line(&mut name) {
        // .expect("Failed to read line");
        Ok(_) => name = name.trim().to_string(), // trim to remove newline character
        Err(e) => println!("Error reading name: {}", e),
    }
    println!("Hello, {}!", name);

    let collection = elements("Gold!Silver!Bronze");
    println!("{:#?}", collection);

    chapter15_project_solution();
}

fn elements(slice: &str) -> Vec<&str> {
    slice.split("!").collect()
}

fn chapter15_project_solution() {
    let mut amount = String::from("40");
    make_money(&mut amount);
    println!("New amount: {}", amount);

    let banana = trim_and_capitalize("   banana   ");
    println!("Trimmed and capitalized: {}", banana);

    let full_name = get_identity();
    println!("Full name: {}", full_name);
}

fn make_money(text: &mut String) {
    text.push_str("$$$");
}

fn trim_and_capitalize(text: &str) -> String {
    text.trim().to_uppercase()
}

fn get_identity() -> String {
    let mut first_name = String::new();
    let mut last_name = String::new();
    let input = io::stdin();
    println!("Enter your first name:");
    input
        .read_line(&mut first_name)
        .expect("Failed to read first name");
    println!("Enter your last name:");
    input
        .read_line(&mut last_name)
        .expect("Failed to read last name");
    format!("{} {}", first_name.trim(), last_name.trim())
}
