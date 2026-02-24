use std::f64;

#[derive(Debug, PartialEq, Eq)]
struct Flight {
    origin: String,
    destination: String,
    time: String,
}

impl Flight {
    fn new(origin: &str, destination: &str, time: &str) -> Self {
        Self {
            origin: origin.to_string(),
            destination: destination.to_string(),
            time: time.to_string(),
        }
    }
}

#[allow(clippy::eq_op)]
#[allow(clippy::zero_divided_by_zero)]
pub fn eq_trait() {
    println!("Implementing the Eq trait");

    let a = Flight::new("New York", "Cape Town", "08:00");
    let b = Flight::new("New York", "Cape Town", "08:00");
    let c = Flight::new("New York", "Cape Town", "08:00");

    // Rules for Eq trait
    println!("{}", a == a); // Reflexive a == a
    println!("{}", a == b);
    println!("{}", b == a); // symmetric a==b implies b==a
    println!("{}", a == c); // transitive, a == b, b == c, implies a == c

    // why, not all types naturally support Eq, e.g. float because of NaN
    let division = 0.0 / 0.0;
    println!("{}", division);

    let value = f64::consts::PI;
    println!("{}", value == value); // true
    println!("{}", division == division); // false... NaN can not be compared to NaN
}
