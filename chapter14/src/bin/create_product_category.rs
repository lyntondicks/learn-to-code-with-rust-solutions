use fake::{Fake, Faker};

use chapter14::ProductCategory;

// cargo run -p chapter14 --bin create_product_category

/// Create a random product category
fn main() {
    let random_category: ProductCategory = Faker.fake();
    println!("Random product category: {:?}", random_category);
}
