use fake::{Fake, Faker};

// use std::{
//     fmt,
//     io::{self, stdin, stdout},
// };

// use std::collections::*;    // import all collections. (glob operator). Not recommended to import everything. Increases chance of name collisions
// use std::prelude::rust_2024::*;  // implicitly imported prelude

use chapter14::{
    FLOOR_SPACE, INVENTORY_MANAGER, Item, ORDERS_MANAGER, ProductCategory, talk_to_manager,
};

/// Main entry point for the chapter14 application
/// Run with: cargo run --bin chapter14
/// Generate docs: cargo doc --no-deps
/// View docs: cargo doc --open --no-deps
fn main() {
    println!(
        "Our managers are {} and {}. We have {} square feet of floor space.",
        // inventory::MANAGER,
        // orders::MANAGER,
        INVENTORY_MANAGER,
        ORDERS_MANAGER,
        FLOOR_SPACE
    );

    talk_to_manager();

    let favorite_category = ProductCategory::Hammer;
    println!("My favorite product category is: {:?}", favorite_category);

    let tall_ladder = Item::new(String::from("Ladder-o-matic 2000"), favorite_category, 100);
    println!("{:#?}", tall_ladder);

    let fake_item: Item = Faker.fake();
    println!("{:#?}", fake_item);
}
