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

pub fn option_enum() {
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
    println!("{:?}", bass); // Possible because Option implements Copy trait
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
