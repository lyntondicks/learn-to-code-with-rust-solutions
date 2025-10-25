#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_assignments)]

pub fn main() {
    chapter_11_generics();
    chapter_11_project_solution();
}

// Chapter 11 generics
fn identity<T>(value: T) -> T {
    value
}

fn make_tuple<T, U>(first: T, second: U) -> (T, U) {
    (first, second)
}

#[derive(Debug)]
struct TreasureChest<T> {
    captain: String,
    treasure: T,
}

impl TreasureChest<String> {
    // define methods that only apply to String treasures
    fn clean_treasure(&mut self) {
        self.treasure = self.treasure.trim().to_string();
    }
}

impl TreasureChest<[&str; 3]> {
    // define methods that only apply to 3 element arrays of ref strings
    fn treasure_summary(&self) -> String {
        self.treasure.join(", ")
    }

    fn amount_of_treasure(&self) -> usize {
        self.treasure.len()
    }
}

impl<T> TreasureChest<T> {
    // define methods available for all treasure types
    fn capital_captain(&self) -> String {
        self.captain.to_uppercase()
    }
}

enum CheeseSteak<T> {
    Plain,
    Topping(T),
}

fn chapter_11_generics() {
    // Turbofish operator = ::<T>
    println!("{}", identity::<u8>(5));
    println!("{}", identity(13.14));
    println!("{}", identity::<&str>("Hello, world!"));
    println!("{}", identity(String::from("Generics in Rust")));

    make_tuple(5, "Hello");
    make_tuple(3.14, 42);

    let gold_chest = TreasureChest {
        captain: String::from("Captain Jack Sparrow"),
        treasure: "Gold",
    };
    println!(
        "{} is the captain of the gold chest.",
        gold_chest.capital_captain()
    );

    // gold_chest.clean_treasure(); // Compile error: only defined for TreasureChest<String>
    println!("{:?}", gold_chest);

    let mut silver_chest = TreasureChest {
        captain: String::from("Captain Silver"),
        treasure: String::from("         Silver Coins          "),
    };
    println!(
        "{} is the captain of the silver chest.",
        silver_chest.capital_captain()
    );
    silver_chest.clean_treasure();

    println!("{:?}", silver_chest);

    let special_chest = TreasureChest {
        captain: String::from("Captain Special"),
        treasure: ["Gold", "Silver", "Platinum"],
    };
    println!(
        "{} is the captain of the special chest.",
        special_chest.capital_captain()
    );
    println!(
        "Summary: {}. Amount of treasure: {}",
        special_chest.treasure_summary(),
        special_chest.amount_of_treasure()
    );

    println!("{:?}", special_chest);

    let mushroom = CheeseSteak::Topping("Mushroom");
    let onions = CheeseSteak::Topping("Onions".to_string());
    let topping = "bacon".to_string();
    let bacon = CheeseSteak::Topping(&topping);
    let mut plain: CheeseSteak<String> = CheeseSteak::Plain;
    plain = CheeseSteak::Topping("Plain".to_string());
}

#[derive(Debug)]
enum DigitalContent {
    AudioFile,
    VideoFile,
}

#[derive(Debug)]
struct ChatMessage<T> {
    content: T,
    time: String,
}

impl ChatMessage<DigitalContent> {
    fn consume_entertainment(&self) {
        println!("Watching the {:?}", self.content);
    }
}

impl<T> ChatMessage<T> {
    fn retrieve_time(&self) -> String {
        self.time.clone()
    }
}

fn chapter_11_project_solution() {
    let message = ChatMessage {
        content: "Hi, lol",
        time: String::from("2025-01-01 12:00:00"),
    };
    println!("{}", message.retrieve_time());

    let notification = ChatMessage {
        content: String::from("What's your favorite pizza topping?"),
        time: String::from("2025-01-02 12:00:00"),
    };
    println!("{}", notification.retrieve_time());

    let audio = ChatMessage {
        content: DigitalContent::AudioFile,
        time: String::from("2025-01-03 12:00:00"),
    };
    audio.consume_entertainment();
    println!("{}", audio.retrieve_time());
}
