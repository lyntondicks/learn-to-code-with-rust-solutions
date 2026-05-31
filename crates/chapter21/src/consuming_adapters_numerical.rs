#![allow(
    clippy::iter_count,
    clippy::zero_divided_by_zero,
    clippy::eq_op,
    clippy::useless_vec
)]

pub fn consume_adapters_numerical() {
    println!("Chapter 21: Consuming Adapters Numerical");

    let numbers = vec![3, 4, 6, 87, 7, 6];

    let total: i32 = numbers.iter().sum();
    println!("Total: {total}");

    let product: i32 = numbers.iter().product();
    println!("Product: {product}");

    let max = numbers.iter().max().unwrap();
    println!("Max: {max}");

    let min = numbers.iter().min().unwrap();
    println!("Min: {min}");

    let count = numbers.iter().count();
    println!("Count: {count}");

    let numbers = vec![4.6, 8.8, 0.0 / 0.0, 6.2, f64::NAN];
    let total: f64 = numbers.iter().sum();
    println!("f64 Total with NAN: {total}"); // NaN

    // won't work because f64 doesn't implement Ord trait
    // let max = numbers.iter().max().unwrap();

    let total: f64 = numbers
        .iter()
        .filter(|n| !n.is_nan())
        .copied()
        .fold(0.0, |total, current| total + current);
    println!("f64 Total filtered out NaN: {total}");

    let max = numbers
        .iter()
        // .filter(|n| !n.is_nan()) // f64.max will choose any valid float over NaN
        .copied()
        .reduce(|carry, current| carry.max(current));
    println!("Max of f64 vec: {max:?}");
}
