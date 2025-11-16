use fake::Dummy;

/// Represents a product category in the warehouse
#[derive(Debug, Clone, Copy, Dummy)]
pub enum ProductCategory {
    Ladder,
    Hammer,
}

/// Represents an item in the warehouse
#[derive(Debug, Dummy)]
pub struct Item {
    pub name: String,
    pub category: ProductCategory,
    pub quantity: u32,
}

impl Item {
    /// Create a new item
    pub fn new(name: String, category: ProductCategory, quantity: u32) -> Self {
        super::talk_to_manager(); // Call the function from the parent module, the function doesn't need to be public similar to inner scope accessing variables from outer scope
        Self {
            name,
            category,
            quantity,
        }
    }
}
