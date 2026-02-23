struct BusTrip {
    origin: String,
    destination: String,
    time: String,
}

impl BusTrip {
    fn new(origin: &str, destination: &str, time: &str) -> Self {
        Self {
            origin: origin.to_string(),
            destination: destination.to_string(),
            time: time.to_string(),
        }
    }
}

impl PartialEq for BusTrip {
    fn eq(&self, other: &Self) -> bool {
        self.origin == other.origin && self.destination == other.destination
    }
}

// deriving PartialEq will implement equality for all fields
#[derive(Debug)]
struct Flight {
    origin: String,
    destination: String,
    time: String,
}

impl PartialEq<Flight> for BusTrip {
    fn eq(&self, other: &Flight) -> bool {
        self.time == other.time
    }
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

impl PartialEq for Flight {
    fn eq(&self, other: &Self) -> bool {
        self.origin == other.origin && self.destination == other.destination
    }
}

impl PartialEq<BusTrip> for Flight {
    fn eq(&self, other: &BusTrip) -> bool {
        self.time == other.time
    }
}

#[derive(Debug)]
enum Musician {
    SingerSongWriter(String),
    Band(u32),
}

use Musician::*; // expose all variants in scope

impl PartialEq for Musician {
    fn eq(&self, other: &Self) -> bool {
        match self {
            SingerSongWriter(name) => match other {
                SingerSongWriter(other_name) => name == other_name,
                Band(_) => false,
            },
            Band(count) => match other {
                SingerSongWriter(_) => false,
                Band(other_count) => count == other_count,
            },
        }
    }
}

pub fn partial_eq() {
    println!("Partial Eq");

    let flight1 = Flight::new("New York", "Los Angeles", "10:00 AM");
    let flight2 = Flight::new("New York", "Los Angeles", "2:00 PM");
    let flight3 = Flight::new("Chicago", "Miami", "11:00 AM");

    println!("flight1 == flight2: {}", flight1 == flight2); // true
    println!("flight1 == flight3: {}", flight1 == flight3); // false
    println!("{}", flight1 != flight3); // true
    println!("{}", flight1.eq(&flight2)); // true
    println!("{}", flight1.ne(&flight3)); // true

    let bus_trip = BusTrip::new("New York", "Los Angeles", "10:00 AM");
    println!("flight1 == bus_trip: {}", flight1 == bus_trip); // true
    println!("flight1.eq(&bus_trip): {}", flight1.eq(&bus_trip)); // true
    println!("bus_trip.eq(&flight1): {}", bus_trip.eq(&flight1)); // true
    println!("bus_trip == flight1: {}", bus_trip == flight1); // true

    let musician1 = SingerSongWriter("John".to_string());
    let musician2 = SingerSongWriter("John".to_string());
    let musician3 = Band(3);

    println!("musician1 == musician2: {}", musician1 == musician2); // true
    println!("musician1 == musician3: {}", musician1 == musician3); // false
    println!("musician1 != musician3: {}", musician1 != musician3); // true
    println!("musician1.eq(&musician2): {}", musician1.eq(&musician2)); // true
    println!("musician1.ne(&musician3): {}", musician1.ne(&musician3)); // true
}
