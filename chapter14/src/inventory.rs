// 3 options for declaring submodule for products
// 1. Inline module
// 2. inventory/products.rs
// 3. inventory/products/mod.rs
pub mod products; // convention to list modules and submodules at the top of the file

// Can export items from the products module directly from inventory module
pub use products::{Item, ProductCategory};

pub const FLOOR_SPACE: i32 = 10000;
pub const MANAGER: &str = "Ivan Inventory";

pub fn talk_to_manager() {
    println!("Hey, {MANAGER}, how's your coffee?");
    println!(
        "Absolute module path to MANAGER is: {}",
        crate::inventory::MANAGER
    ); // absolute path to module
    println!("Relative module path: {:#?}", ProductCategory::Ladder); // relative path to submodule
}
