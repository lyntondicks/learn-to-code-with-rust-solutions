use std::fmt::{Debug, Display, Formatter, Result};
use std::fs;
use std::ops::Drop;

enum AppleType {
    GrannySmith,
    Fuji,
    Honeycrisp,
}

impl Display for AppleType {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> Result {
        let kind = match self {
            AppleType::GrannySmith => "🍎 Granny Smith 🍎",
            AppleType::Fuji => "🗻 Fuji 🍏",
            AppleType::Honeycrisp => "🍯 Honeycrisp 🍏",
        };
        write!(formatter, "{}", kind)
    }
}

impl Debug for AppleType {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> Result {
        let kind = match self {
            AppleType::GrannySmith => "AppleType::GrannySmith",
            AppleType::Fuji => "AppleType::Fuji",
            AppleType::Honeycrisp => "AppleType::Honeycrisp",
        };
        write!(formatter, "{}", kind)
    }
}

pub struct Apple {
    kind: AppleType,
    price: f64,
}

impl Display for Apple {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> Result {
        write!(formatter, "{} 🍏 costs ${:.2}", self.kind, self.price)
    }
}

impl Debug for Apple {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> Result {
        formatter
            .debug_struct("Apple")
            .field("kind", &self.kind)
            .field("price", &self.price)
            .finish()
    }
}

impl Drop for Apple {
    fn drop(&mut self) {
        match fs::remove_file("apple.txt") {
            Ok(_) => println!("Successfully removed apple.txt"),
            Err(e) => eprintln!("Error removing apple.txt: {}", e),
        }
    }
}

pub fn display_trait() {
    println!("Implementing Debug and Display Traits");

    let lunch_snack = Apple {
        kind: AppleType::GrannySmith,
        price: 1.04,
    };
    println!("{}", lunch_snack);
    println!("{:?}", lunch_snack);

    let dinner_snack = Apple {
        kind: AppleType::Fuji,
        price: 1.29,
    };
    println!("{}", dinner_snack);
    println!("{:?}", dinner_snack);

    let dessert_snack = Apple {
        kind: AppleType::Honeycrisp,
        price: 1.49,
    };
    println!("{}", dessert_snack);
    println!("{:?}", dessert_snack);
}
