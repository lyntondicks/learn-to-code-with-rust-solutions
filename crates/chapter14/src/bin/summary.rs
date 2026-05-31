use chapter14::{FLOOR_SPACE, INVENTORY_MANAGER, ORDERS_MANAGER};

// cargo run -p chapter14 --bin summary

/// Print a summary of the chapter14 warehouse
fn main() {
    println!(
        "Our managers are {} and {}. We have {} square feet of floor space.",
        INVENTORY_MANAGER, ORDERS_MANAGER, FLOOR_SPACE
    );
}
