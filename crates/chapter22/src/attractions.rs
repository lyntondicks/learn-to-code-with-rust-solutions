#![allow(clippy::new_without_default)]

pub trait TicketSeller {
    fn sell_ticket(&mut self);
}

#[derive(Debug, PartialEq, Eq)]
pub struct Museum {
    pub paintings: Vec<String>,
    pub revenue: u32,
}

impl Museum {
    const MAXIMUM_CAPACITY: usize = 3;

    /// Creates a new Museum instance.
    ///
    /// # Examples
    /// ```
    /// use chapter22::attractions::Museum;
    ///
    /// let museum = Museum::new();
    /// let empty_vec: Vec<String> = Vec::new();
    ///
    /// assert_eq!(museum.paintings, empty_vec);
    /// assert_eq!(museum.revenue, 0);
    /// ```
    pub fn new() -> Self {
        Museum {
            paintings: vec![],
            revenue: 0,
        }
    }

    /// Buys a painting for the museum.
    ///
    /// # Examples
    /// ```
    /// use chapter22::attractions::Museum;
    ///
    /// let mut museum = Museum::new();
    /// museum.buy_painting("Mona Lisa");
    ///
    /// assert_eq!(museum.paintings, vec!["Mona Lisa".to_string()]);
    /// ```
    pub fn buy_painting(&mut self, painting: &str) {
        if self.paintings.len() >= Self::MAXIMUM_CAPACITY {
            panic!("Museum does not have storage space for another painting!");
        }
        self.paintings.push(painting.to_string());
    }

    pub fn has_impressive_collection(&self) -> bool {
        self.paintings.len() > 2
    }
}

impl TicketSeller for Museum {
    fn sell_ticket(&mut self) {
        self.revenue += if self.has_impressive_collection() {
            35
        } else {
            25
        }
    }
}

#[derive(Debug)]
pub struct MovieTheater {
    pub movies: Vec<String>,
    pub sales: u32,
}

impl MovieTheater {
    pub fn new() -> Self {
        Self {
            movies: vec![],
            sales: 0,
        }
    }

    pub fn add_movie(&mut self, movie: &str) {
        self.movies.push(movie.to_string());
    }
}

impl TicketSeller for MovieTheater {
    fn sell_ticket(&mut self) {
        self.sales += 15
    }
}

// Can run a specific test with: `cargo test museum_sells_ticket_to_increase_revenue`
// or more specific with module name: `cargo test tests::museum_sells_ticket_to_increase_revenue`
// or run tests that contain `ticket` in their name: `cargo test ticket`
#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::{assert_eq, assert_ne};

    #[test]
    fn print_success() {
        // Show any printed output when running the tests: `cargo test -- --show-output`
        println!("Success inside the function");
        assert!(true);
    }

    #[test]
    fn museum_sells_ticket_to_increase_revenue() -> Result<(), String> {
        // super::Museum::new();
        let mut museum = Museum::new();
        museum.sell_ticket();

        if museum.revenue == 25 {
            Ok(())
        } else {
            Err("The revenue from selling 1 ticket did not match expectations.".to_string())
        }
        // assert_eq!(
        //     museum.revenue, 25,
        //     "The revenue from selling 1 ticket did not match expectations."
        // );
    }

    #[test]
    fn museum_with_impressive_art_collection_charges_more_for_admission() -> Result<(), String> {
        let mut museum = Museum::new();
        museum.buy_painting("Mona Lisa");
        museum.buy_painting("The Starry Night");
        museum.buy_painting("Girl with a Pearl Earring");

        museum.sell_ticket();

        if museum.revenue == 35 {
            Ok(())
        } else {
            Err("The revenue from selling a ticket when the collection is impressive did not meet expectations.".to_string())
        }
    }

    #[test]
    fn museum_sells_ticket_to_increase_revenue_ne() {
        let mut museum = Museum::new();
        museum.sell_ticket();
        assert_ne!(museum.revenue, 0);
    }

    #[test]
    fn museum_can_sell_multiple_tickets() {
        let mut museum = Museum::new();
        museum.sell_ticket();
        museum.sell_ticket();
        assert_eq!(museum.revenue, 50);
    }

    #[test]
    fn museum_can_have_impressive_art_collection() -> Result<(), String> {
        let mut museum = Museum::new();
        museum.buy_painting("Mona Lisa");
        museum.buy_painting("The Starry Night");
        museum.buy_painting("Girl with a Pearl Earing");
        if museum.has_impressive_collection() {
            Ok(())
        } else {
            Err("The museum did not have an impressive collection".to_string())
        }
        // assert!(
        //     museum.has_impressive_collection(),
        //     "The museum did not have an impressive collection"
        // );
    }

    #[test]
    #[ignore] // ignore this test during normal test runs
    fn new_museums_are_equal() {
        let museum1 = Museum::new();
        let museum2 = Museum::new();
        assert_eq!(
            museum1, museum2,
            "Two new Museum instances were not found to be equal: {:#?}, {:#?}",
            museum1, museum2
        );
    }

    // If need to silence terminal output. `cargo test -- --quiet`
    #[test]
    #[should_panic(expected = "storage space")] // Contains "storage space". Full: "Museum does not have storage space for another painting!"
    fn museum_prohibits_adding_painting_when_capacity_has_been_reached() {
        let mut museum = Museum::new();
        museum.buy_painting("Mona Lisa");
        museum.buy_painting("The Starry Night");
        museum.buy_painting("Girl with a Pearl Earring");
        museum.buy_painting("Water Lilies");
    }
}
