/// Inventory management module
pub mod inventory;

/// Order management module
pub mod orders;

// use inventory::products;
// use inventory::products::{self, ProductCategory}; // self here refers to the products submodule, so that products::Item can be used below
pub use inventory::{
    FLOOR_SPACE, Item, MANAGER as INVENTORY_MANAGER, ProductCategory, talk_to_manager,
};
pub use orders::MANAGER as ORDERS_MANAGER;
