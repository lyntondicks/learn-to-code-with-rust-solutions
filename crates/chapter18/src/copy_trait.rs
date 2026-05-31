#![allow(dead_code)]

// This module demonstrates the Copy trait in Rust
// The Copy trait allows for types to be duplicated simply by assignment

#[derive(Debug, Copy, Clone)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug, Clone, Copy)]
struct Duration {
    hours: u32,
    minutes: u32,
    seconds: u32,
}

impl Duration {
    fn new(hours: u32, minutes: u32, seconds: u32) -> Self {
        Self {
            hours,
            minutes,
            seconds,
        }
    }
}

pub fn copy_trait() {
    let p1 = Point { x: 10, y: 20 };
    let p2 = p1; // p1 is copied, not moved

    println!("p1: {:?}", p1);
    println!("p2: {:?}", p2);

    let one_hour = Duration::new(1, 0, 0);
    let another_hour = one_hour;
    println!("one_hour: {:?}", one_hour);
    println!("another_hour: {:?}", another_hour);
}
