use std::fmt::{Debug, Display, Formatter};

#[allow(dead_code)]
trait Drinkable {
    fn consume(&mut self);
    fn get_data(&self) -> String;
    fn stats(&self) {
        println!("{}", self.get_data());
    }
}

#[allow(dead_code)]
#[derive(Debug)]
enum Milk {
    Whole,
    Oat,
    Almond,
}

struct Coffee<T> {
    kind: T,
    milk: Milk,
    ounces: u32,
}

impl<T> Coffee<T> {
    fn new(kind: T, milk: Milk, ounces: u32) -> Self {
        Self { kind, milk, ounces }
    }
}

impl<T: Debug> Debug for Coffee<T> {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("Coffee")
            .field("kind", &self.kind)
            .field("milk", &self.milk)
            .field("ounces", &self.ounces)
            .finish()
    }
}

impl<T: Display> Drinkable for Coffee<T> {
    fn consume(&mut self) {
        self.ounces = 0;
    }

    fn get_data(&self) -> String {
        format!(
            "Coffee: {}, Milk: {:?}, Ounces left: {}",
            self.kind, self.milk, self.ounces
        )
    }
}

#[derive(Debug)]
struct Soda {
    calories: u32,
    price: f64,
    flavor: String,
    percentage: u32,
}

impl Soda {
    fn new(calories: u32, price: f64, flavor: &str, percentage: u32) -> Self {
        Self {
            calories,
            price,
            flavor: flavor.to_string(),
            percentage,
        }
    }
}

impl Drinkable for Soda {
    fn consume(&mut self) {
        self.percentage = 0;
    }

    fn get_data(&self) -> String {
        format!(
            "Soda: {}, Calories: {}, Price: ${:.2}, Percentage: {}%",
            self.flavor, self.calories, self.price, self.percentage
        )
    }
}

impl Display for Soda {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            formatter,
            "Soda: {}, Calories: {}, Price: ${:.2}, Percentage: {}%",
            self.flavor, self.calories, self.price, self.percentage
        )
    }
}

impl Clone for Soda {
    fn clone(&self) -> Self {
        Self {
            calories: self.calories,
            price: self.price,
            flavor: self.flavor.clone(),
            percentage: self.percentage,
        }
    }
}

impl PartialEq for Soda {
    fn eq(&self, other: &Self) -> bool {
        self.calories == other.calories
    }
}

impl Eq for Soda {}

pub fn project_solution() {
    println!("Chapter 18 Project Solution");

    let mut latte = Coffee::new("Latte", Milk::Oat, 32);
    println!("{:?}", latte);
    latte.consume();
    println!("{:?}", latte);

    let cappuccino = Coffee::new(String::from("Cappuccino"), Milk::Whole, 16);
    println!("{}", cappuccino.get_data());

    let pepsi = Soda::new(300, 2.99, "Cherry Soda", 100);
    println!("{}", pepsi);

    let mut coke = pepsi.clone();
    println!("Are the two soda equal in price? {}", pepsi == coke);
    coke.consume();
    println!("After consuming coke: {:?}", coke);
}
