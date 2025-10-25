pub fn main() {
    println!("Chapter 5: Control Flow");
    branching();
    iteration();
    recursion();
}

fn branching() {
    let season = "spring";

    if season == "summer" {
        println!("School's out!");
    } else if season == "winter" {
        println!("Brr, it's cold!");
    } else if season == "fall" {
        println!("Leaves are falling!");
    } else {
        println!("Spring is in the air!");
    }

    // if expression
    let _is_summer = if season == "summer" { true } else { false };

    // match expression
    match season {
        "summer" => {
            println!("It's summer!")
        } // block expression
        "winter" => println!("It's winter!"),
        "fall" => println!("It's fall!"),
        "spring" => println!("It's spring!"),
        _ => println!("Unknown season!"),
    }

    let evaluation = false;
    let value = match evaluation {
        true => "Evaluation is true",
        false => "Evaluation is false",
    };
    println!("Match evaluation: {}", value);

    let number: u8 = 8;
    match number {
        2 | 4 | 6 | 8 => println!("{} is an even number", number),
        1 | 3 | 5 | 7 => println!("{} is an odd number", number),
        _ => println!("{number} is not a single-digit number"),
    }

    match number {
        value if value % 2 == 0 => println!("{value} is an even number"),
        x if x % 2 != 0 => println!("{x} is an odd number"),
        // _ => println!("{number} is not an even number"),
        _ => unreachable!(), // This will panic if reached, as all cases are covered
    }
}

fn iteration() {
    let numbers = [1, 2, 3, 4, 5];
    for number in numbers.iter() {
        println!("Number: {}", number);
    }

    let mut index = 0;
    while index < numbers.len() {
        println!("Index: {}, Value: {}", index, numbers[index]);
        index += 1;
    }

    let mut count = 0;
    loop {
        if count == 2 {
            println!("Skipping count 2");
            count += 1; // Increment count to avoid infinite loop
            continue;
        }
        if count >= 5 {
            break; // Exit the loop when count reaches 5
        }
        println!("Count: {}", count);
        count += 1;
    }

    // loop is also an expression
    let mut counter = 0;
    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2; // returning value from loop expression
        }
    };
    println!("The result of the loop expression is: {}", result);

    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");

    for number in 1..4 {
        println!("Number: {}", number);
    }

    for number in (4..=6).rev() {
        println!("Reverse Number: {}", number);
    }
}

fn recursion() {
    fn factorial(n: u32) -> u32 {
        if n == 0 { 1 } else { n * factorial(n - 1) }
    }

    let num = 5;
    let result = factorial(num);
    println!("Factorial of {num} is {result}");
}