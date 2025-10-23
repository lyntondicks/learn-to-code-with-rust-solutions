pub fn result_enum() {
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

fn operation(great_success: bool) -> Result<String, String> {
    if great_success {
        Ok("Operation was successful".to_string())
    } else {
        Err("Operation failed".to_string())
    }
}
