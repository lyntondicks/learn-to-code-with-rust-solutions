use std::{collections::HashMap, fmt::Display};

pub trait Accommodation {
    fn book(&mut self, name: &str, nights: u32);
}

pub trait Description {
    /// Returns a description of the accommodation.
    /// Default implementation returns a generic description.
    fn get_description(&self) -> String {
        format!("A great place to stay!")
    }
}

#[derive(Debug)]
pub struct Hotel<T> {
    pub name: T,
    pub reservations: HashMap<String, u32>, // guest name -> number of nights
}

impl<T> Hotel<T> {
    pub fn new(name: T) -> Self {
        Self {
            name,
            reservations: HashMap::new(),
        }
    }
}

impl<T: Display> Hotel<T> {
    pub fn summarize(&self) -> String {
        format!("{}: {}", self.name, self.get_description())
    }
}

impl<T> Description for Hotel<T> {}

impl<T> Accommodation for Hotel<T> {
    // use default implementation for get_description
    // fn get_description(&self) -> String {
    //     format!("{} is the pinnacle of luxury", self.name)
    // }

    fn book(&mut self, name: &str, nights: u32) {
        self.reservations.insert(name.to_string(), nights);
    }
}

#[derive(Debug)]
pub struct AirBnB {
    pub host: String,
    pub guests: Vec<(String, u32)>, // guest name and number of nights
}

impl AirBnB {
    pub fn new(host: &str) -> Self {
        Self {
            host: host.to_string(),
            guests: vec![],
        }
    }
}

impl Accommodation for AirBnB {
    fn book(&mut self, name: &str, nights: u32) {
        self.guests.push((name.to_string(), nights));
    }
}

impl Description for AirBnB {
    fn get_description(&self) -> String {
        format!("AirBnB hosted by {}", self.host)
    }
}

pub fn trait_def_and_impl() {
    let mut hotel = Hotel::new("The Grand Hotel");
    let mut airbnb = AirBnB::new("John Doe");

    println!("Hotel summary: {}", hotel.summarize());
    println!("AirBnB description: {}", airbnb.get_description());

    hotel.book("Alice", 3);
    airbnb.book("Bob", 2);

    println!("Hotel reservations: {:?}", hotel.reservations);
    println!("AirBnB guests: {:?}", airbnb.guests);
}
