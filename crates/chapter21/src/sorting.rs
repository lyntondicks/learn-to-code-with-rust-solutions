#![allow(dead_code)]

#[derive(Debug)]
struct GasStation {
    snack_count: u32,
    manager: String,
    employee_count: u32,
}

pub fn sorting() {
    println!("Chapter 21: Sorting");

    let mut points = [3, 8, 1, 11, 5];
    println!("Is sorted: {}", points.is_sorted());

    points.sort();
    println!("Sorted points: {:?}", points);
    println!("Is sorted now: {}", points.is_sorted());

    points.reverse();
    println!("Sorted points in reverse: {:?}", points);
    println!("Is reverse sorted now: {}", points.is_sorted());

    // note: string sorting
    // uppercase/titlecase letters are arranged before lowercase
    let mut exercises = ["squat", "bench", "Deadlift"];
    exercises.sort();
    println!("String sorting: {exercises:?}");

    let mobil = GasStation {
        snack_count: 100,
        manager: String::from("Meg Mobile"),
        employee_count: 3,
    };

    let exxon = GasStation {
        snack_count: 130,
        manager: String::from("Eric Exxon"),
        employee_count: 4,
    };

    let shell = GasStation {
        snack_count: 50,
        manager: String::from("Shane Shell"),
        employee_count: 2,
    };

    let mut stops = [mobil, exxon, shell];
    // stops.sort(); // Won't work unless GasStation implements Ord trait
    stops.sort_by_key(|station| -(station.employee_count as i32)); // trick to sort descending -4, -3, -2
    println!("{stops:?}");
}
