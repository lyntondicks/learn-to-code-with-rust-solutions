#[derive(Debug)]
struct Location {
    name: String,
    treasures: u32,
}

struct Map<'a> {
    locations: &'a [Location],
}

impl<'a> Map<'a> {
    fn explore<F>(&self, mut action: F)
    where
        F: FnMut(&Location),
    {
        let mut current_index = 0;
        while current_index < self.locations.len() {
            let current_location = &self.locations[current_index];
            action(current_location);
            current_index += 1;
        }
    }
}

pub fn fn_mut_trait() {
    println!("Chapter 20: FnMut trait");
    let locations = [
        Location {
            name: String::from("Enchanted forest"),
            treasures: 5,
        },
        Location {
            name: String::from("Mystic Mountain"),
            treasures: 10,
        },
    ];
    let map = Map {
        locations: &locations,
    };

    let mut total_treasures = 0;
    map.explore(|location| total_treasures += location.treasures);
    println!("Total treasures collected: {total_treasures}");

    let mut location_names: Vec<String> = Vec::new();
    map.explore(|location| location_names.push(location.name.clone()));
    println!("Location names: {:?}", location_names);
}
