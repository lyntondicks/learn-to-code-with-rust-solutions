#![allow(dead_code)]

pub fn main() {
    option_enum();
    result_enum();
    chapter_12_project_solution();
}

#[derive(Debug, Copy, Clone)]
enum MyOption {
    Some(i32),
    None,
}

impl MyOption {
    fn unwrap(self) -> i32 {
        match self {
            MyOption::Some(value) => value,
            MyOption::None => panic!("Called `MyOption::unwrap()` on a `None` value"),
        }
    }

    fn unwrap_or(self, default: i32) -> i32 {
        match self {
            MyOption::Some(value) => value,
            MyOption::None => default,
        }
    }
}

fn option_enum() {
    let _a = Some(5);
    let _b = Option::Some("hello");
    let _a: Option<i8> = Option::Some(5);
    let _a = Option::<i16>::Some(5);
    let _d: Option<&str> = None;
    let _d = Option::<&str>::None;

    let musical_instruments = [
        String::from("Guitar"),
        String::from("Drums"),
        String::from("Bass"),
    ];

    let bass = musical_instruments.get(2);
    println!("{:?}", bass);

    let invalid_instrument = musical_instruments.get(100);
    println!("{:?}", invalid_instrument);

    let valid_instrument = bass.unwrap(); // not idiomatic and can panic at runtime if the Option is None
    println!("Valid instrument: {}", valid_instrument);
    let _expect_valid_instrument = bass.expect("Expected a valid instrument, but got None"); // Panic with custom message

    play(bass);
    println!("{:?}", bass);
    play(invalid_instrument);

    let present_value = Some(13);
    let missing_value = Option::<i32>::None;
    println!("{}", present_value.unwrap_or(0));
    println!("{}", missing_value.unwrap_or(100));

    let some_option = MyOption::Some(100);
    println!("{}", some_option.unwrap());

    let none_option = MyOption::None;
    // println!("{}", none_option.unwrap()); // panic
    println!("{}", none_option.unwrap_or(0));
}

fn play(instrument_option: Option<&String>) {
    // Option implements Copy trait
    match instrument_option {
        Some(instrument) => println!("Playing the {}", instrument),
        None => println!("No instrument to play"),
    }
}

fn result_enum() {
    let ok: Result<i8, &str> = Ok(5);
    println!("{ok:?}");
    let disaster: Result<i32, &str> = Err("Something went wrong");
    println!("{:?}", disaster);

    let text_as_number = "50".parse::<i32>();
    println!("{:?}", text_as_number); // Result<i32, ParseIntError>
    let text_as_number = "oops".parse::<i32>();
    println!("{:?}", text_as_number);

    let result = divide(10.0, 2.0);
    match result {
        // match &result (borrow to avoid clones below)
        Ok(value) => println!("Result of division: {}", value),
        Err(ref err) => println!("Error occurred: {}", err),
    }
    println!("{}", result.clone().unwrap());
    println!("{}", result.clone().expect("Unable to parse calculation"));
    println!("{}", result.clone().unwrap_or(0.0)); // If Ok, return value, if Err, return 0.0
    println!("{}", result.is_ok()); // Check if the result is Ok
    println!("{}", result.is_err()); // Check if the result is Err

    let my_result = operation(true);
    let _content = match &my_result {
        Ok(message) => message,
        Err(error) => error,
    };
    println!("{}", my_result.as_ref().unwrap());
    println!("{}", my_result.as_ref().unwrap());
}

fn divide(numerator: f64, denominator: f64) -> Result<f64, String> {
    if denominator == 0.0 {
        Err("Division by zero error".into())
    } else {
        Ok(numerator / denominator)
    }
}
// -> Result<&'static str, &'static str> Lifetimes
fn operation(great_success: bool) -> Result<String, String> {
    if great_success {
        Ok("Operation was successful".to_string())
    } else {
        Err("Operation failed".to_string())
    }
}

// Chapter 12 project solution
#[derive(Debug)]
struct Food {
    name: String,
}

#[derive(Debug)]
struct Restaurant {
    reservations: u32,
    has_mice_infestation: bool,
}

impl Restaurant {
    fn chef_special(&self) -> Option<Food> {
        if self.has_mice_infestation {
            return None;
        }

        if self.reservations < 12 {
            Some(Food {
                name: String::from("Uni Sashimi"),
            })
        } else {
            Some(Food {
                name: String::from("Strip Steak"),
            })
        }
    }

    fn deliver_burger(&self, address: &str) -> Result<Food, String> {
        if self.has_mice_infestation {
            return Err(String::from("Sorry, we have a mice problem"));
        }

        if address.is_empty() {
            return Err(String::from("No delivery address specified"));
        }

        Ok(Food {
            name: String::from("Burger"),
        })
    }
}

fn chapter_12_project_solution() {
    let marios = Restaurant {
        reservations: 11,
        has_mice_infestation: true,
    };
    println!("{:?}", marios.chef_special());
    println!("{:?}", marios.deliver_burger("123 Elm Street"));

    let angelos = Restaurant {
        reservations: 15,
        has_mice_infestation: false,
    };
    println!("{:?}", angelos.chef_special());
    println!("{:?}", angelos.deliver_burger(""));
    println!("{:?}", angelos.deliver_burger("456 Oak Avenue"));
}
