use std::{f64, ops::Add};

#[derive(Debug)]
struct Lunch {
    cost: f64,
}

impl Add for Lunch {
    type Output = Lunch;

    fn add(self, rhs: Self) -> Self::Output {
        Lunch {
            cost: self.cost + rhs.cost,
        }
    }
}

fn add_two_numbers<T: Add<Output = T>>(a: T, b: T) -> T {
    a + b
}

pub fn associated_types() {
    println!("Associated Types");

    let lunch1 = Lunch { cost: 10.0 };
    let lunch2 = Lunch { cost: 15.0 };
    let total_cost = lunch1 + lunch2;
    println!("Total lunch cost: ${:.2}", total_cost.cost);

    println!("{}", add_two_numbers(f64::consts::PI, 2.3));
    println!("{}", add_two_numbers(2, 3));
}
