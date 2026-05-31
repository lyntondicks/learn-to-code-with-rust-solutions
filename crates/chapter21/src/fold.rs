use std::collections::HashMap;

struct SupportStaff {
    day: String,
    employee: String,
}

pub fn fold() {
    println!("Chapter 21: Fold");

    let earnings: [i32; 4] = [100, 200, 300, 400];

    let sum = earnings.into_iter().fold(0, |total, current| {
        println!("Total so far: {total}, current earning: {current}");
        total + current
    });

    println!("Total earnings: {sum}");

    let week = [
        SupportStaff {
            day: String::from("Monday"),
            employee: String::from("Alice"),
        },
        SupportStaff {
            day: String::from("Tuesday"),
            employee: String::from("Bob"),
        },
        SupportStaff {
            day: String::from("Wednesday"),
            employee: String::from("Charlie"),
        },
        SupportStaff {
            day: String::from("Thursday"),
            employee: String::from("Diana"),
        },
        SupportStaff {
            day: String::from("Friday"),
            employee: String::from("Eve"),
        },
    ];

    let map = week.into_iter().fold(HashMap::new(), |mut acc, entry| {
        acc.insert(entry.day, entry.employee);
        acc
    });
    println!("{map:?}");
}
