#![allow(dead_code)]

pub fn main() {
    println!("--- Chapter 9: Structs and Methods ---");
    named_field_structs();
    struct_methods();
    builder_pattern();
    tuple_struct();
    chapter9_project_solution();
}

#[derive(Debug)]
struct Coffee {
    price: f64,
    name: String,
    is_hot: bool,
}

// 3 kinds of structs, named, tuple, and unit-like
fn named_field_structs() {
    let mut beverage = Coffee {
        price: 4.50,
        name: String::from("Mocha"),
        is_hot: true,
    };

    beverage.name = String::from("Caramel Macchiato");
    beverage.price = 6.99;
    beverage.is_hot = true;

    println!(
        "Coffee Name: {}, Price: ${:.2}, Is Hot: {}",
        beverage.name, beverage.price, beverage.is_hot
    );

    struct Cereal {
        name: String,
        sugar_content: u32,
    }

    let cereals = [
        Cereal {
            name: String::from("Cookie Crisp"),
            sugar_content: 12,
        },
        Cereal {
            name: String::from("Cinnamon Toast Crunch"),
            sugar_content: 10,
        },
        Cereal {
            name: String::from("Frosted Flakes"),
            sugar_content: 11,
        },
        Cereal {
            name: String::from("Cocoa Puffs"),
            sugar_content: 13,
        },
        Cereal {
            name: String::from("Captain Crunch"),
            sugar_content: 14,
        },
    ];

    for cereal in &cereals {
        println!("{} has {}g of sugar.", cereal.name, cereal.sugar_content);
    }

    let name = String::from("Latte"); // name is first owner of String value
    let coffee: Coffee = make_coffee(
        name, // Passing the name to the function, which takes ownership because function parameter is not a reference
        4.75, true,
    );
    println!(
        "Coffee Name: {}, Price: ${:.2}, Is Hot: {}",
        coffee.name, coffee.price, coffee.is_hot
    );

    // Struct update syntax. Be careful of ownership of types with move trait. Use clone() to get new object
    let mut caramel_macchiato = Coffee {
        name: String::from("Caramel Macchiato"),
        ..coffee // Using the existing coffee struct to fill in the rest of the fields. Opposite order from JavaScript
    };
    drink_coffee(&mut caramel_macchiato);

    // println!("{}", caramel_macchiato); // This would cause a compile-time error as Coffee does not implement the Display trait
    println!("{:?}", caramel_macchiato);
    println!("{:#?}", caramel_macchiato);
}

fn make_coffee(name: String, price: f64, is_hot: bool) -> Coffee {
    Coffee {
        name, // ownership is moved to name field from parameter
        price,
        is_hot,
    }
}

fn drink_coffee(coffee: &mut Coffee) {
    println!("Drinking my delicious {}", coffee.name);
    coffee.is_hot = false;
    coffee.price = 10.99;
}

#[derive(Debug)]
struct TaylorSwiftSong {
    title: String,
    release_year: u32,
    duration_secs: u32,
}

// Can separate Associated functions from methods via multiple impl blocks if desired
impl TaylorSwiftSong {
    fn new(title: &str, release_year: u32, duration_secs: u32) -> Self {
        // Associated function (Doesn't have Self parameter)
        Self {
            title: String::from(title),
            release_year,
            duration_secs,
        }
    }
}

impl TaylorSwiftSong {
    fn display_song_info(self: &Self) {
        // or just &self. Method (Takes Self as parameter)
        println!(
            "Song: {}, Released: {}, Duration: {} seconds, Years Since Release: {}",
            self.title,
            self.release_year,
            self.duration_secs,
            self.years_since_release()
        );
    }

    fn play(&self) {
        println!(
            "Playing '{}' released in {} with duration {} seconds.",
            self.title, self.release_year, self.duration_secs
        );
    }

    fn double_length(&mut self) {
        // or self: &mut Self or self: &mut TaylorSwiftSong
        self.duration_secs *= 2; // Doubling the duration of the song
        println!(
            "New duration of '{}' is {} seconds.",
            self.title, self.duration_secs
        );
    }

    fn is_longer_than(&self, other: &Self) -> bool {
        self.duration_secs > other.duration_secs
    }

    fn years_since_release(&self) -> u32 {
        let current_year = 2025;
        current_year - self.release_year
    }
}

fn struct_methods() {
    let mut blank_space = TaylorSwiftSong::new("Love Story", 2008, 233); // Using the associated function to create a new song
    blank_space.display_song_info(); // Method (Takes Self as parameter)
    blank_space.double_length();
    blank_space.play();

    let all_too_well = TaylorSwiftSong {
        title: String::from("All Too Well"),
        release_year: 2012,
        duration_secs: 352,
    };

    if blank_space.is_longer_than(&all_too_well) {
        println!(
            "'{}' is longer than '{}'",
            blank_space.title, all_too_well.title
        );
    } else {
        println!(
            "'{}' is not longer than '{}'",
            blank_space.title, all_too_well.title
        );
    }

    all_too_well.display_song_info();
}

#[derive(Debug)]
struct Computer {
    cpu: String,
    memory: u32,
    hard_drive_capacity: u32,
}

impl Computer {
    fn new(cpu: &str, memory: u32, hard_drive_capacity: u32) -> Self {
        Self {
            cpu: String::from(cpu),
            memory,
            hard_drive_capacity,
        }
    }

    fn upgrade_cpu(mut self, new_cpu: &str) -> Self {
        self.cpu = String::from(new_cpu);
        self
    }

    fn upgrade_memory(mut self, new_memory: u32) -> Self {
        self.memory = new_memory;
        self
    }

    fn upgrade_hard_drive(mut self, new_hard_drive: u32) -> Self {
        self.hard_drive_capacity = new_hard_drive;
        self
    }
}

fn builder_pattern() {
    let computer = Computer::new("Intel i7", 16, 512)
        .upgrade_cpu("Intel i9")
        .upgrade_memory(32)
        .upgrade_hard_drive(1024);

    let computer = computer.upgrade_cpu("Intel i9");

    println!("{:#?}", computer);
}

// Tuple structs
// Hours, minutes
struct ShortDuration(u32, u32);
// Years, months
struct LongDuration(u32, u32);

fn tuple_struct() {
    let work_shift = ShortDuration(8, 0);
    println!("{} hours {} minutes", work_shift.0, work_shift.1);

    let era = LongDuration(5, 3);
    println!("{} years {} months", era.0, era.1);

    go_to_work(work_shift);
    // go_to_work(era); // This would cause a compile-time error as go_to_work expects a ShortDuration
}

fn go_to_work(length: ShortDuration) {
    println!(
        "Going to work for {} hours and {} minutes.",
        length.0, length.1
    );
}

// Unit like structs, for certain design patterns
struct Empty;
// let my_empty_struct = Empty;

#[derive(Debug)]
struct Flight {
    origin: String,
    destination: String,
    price: f64,
    passengers: u32,
}

impl Flight {
    fn new(origin: &str, destination: &str, price: f64, passengers: u32) -> Self {
        Self {
            origin: String::from(origin),
            destination: String::from(destination),
            price,
            passengers,
        }
    }

    fn change_destination(&mut self, new_destination: &str) {
        self.destination = String::from(new_destination);
    }

    fn increase_price(&mut self) {
        self.price *= 1.2;
    }

    fn itinerary(&self) {
        println!("{} -> {}", self.origin, self.destination);
    }
}

fn chapter9_project_solution() {
    let mut flight = Flight::new("New York", "Los Angeles", 300.0, 150);
    flight.change_destination("San Francisco");
    flight.increase_price();
    flight.itinerary();
    println!("Flight details: {:#?}", flight);
    let flight2 = Flight {
        origin: flight.origin.clone(), // Cloning the origin to avoid moving it
        destination: flight.destination.clone(), // Cloning the destination to avoid moving it
        ..flight
    };
    flight2.itinerary();
    println!("Flight 2 details: {:#?}", flight2);
}
