mod project_solution;

use std::fs::{self, File};
use std::io::{self, Read};
use std::process;

pub fn chapter17() {
    println!(
        "{border:#<60}
{border}{title:^58}{border}
{border:#<60}",
        title = "Chapter 17 Error Handling",
        border = "#",
    );
    let file_result = read_file_without_try_operator();
    match file_result {
        Ok(contents) => println!("File contents:\n{}", contents),
        Err(error) => {
            eprintln!("Error reading file: {}", error);
            process::exit(1);
        }
    }

    let new_file_result = read_file();
    match new_file_result {
        Ok(contents) => println!("File contents:\n{}", contents),
        Err(error) => {
            eprintln!("Error reading file: {}", error);
            process::exit(1);
        }
    }

    let simple_file_result = read_file_simple();
    match simple_file_result {
        Ok(contents) => println!("File contents:\n{}", contents),
        Err(error) => {
            eprintln!("Error reading file: {}", error);
            process::exit(1);
        }
    }

    let mut animals = vec!["Giraffe", "Monkey", "Zebra"];
    println!("{:#?}", length_of_last_element(&mut animals));

    project_solution::chapter17_project_solution();
}

fn length_of_last_element(input: &mut Vec<&str>) -> Option<usize> {
    Some(input.pop()?.len())
}

#[allow(clippy::question_mark)]
fn read_file_without_try_operator() -> Result<String, io::Error> {
    println!("Please enter the name of the file you'd like to read:");
    let mut input = String::new();
    let user_requested_file = io::stdin().read_line(&mut input);
    if let Err(error) = user_requested_file {
        return Err(error); // propagate the error to the caller
    }

    let mut file = match File::open(input.trim()) {
        Ok(file) => file,
        Err(error) => {
            return Err(error); // propagate the error to the caller
        }
    };

    let mut file_contents = String::new();
    let read_operation = file.read_to_string(&mut file_contents);
    if let Err(error) = read_operation {
        return Err(error); // propagate the error to the caller
    }

    Ok(file_contents)
}

fn read_file() -> Result<String, io::Error> {
    println!("Please enter the name of the file you'd like to read:");

    let mut input = String::new();
    io::stdin().read_line(&mut input)?; // ? operator to propagate error

    let mut file_contents = String::new();
    File::open(input.trim())?.read_to_string(&mut file_contents)?;

    Ok(file_contents)
}

fn read_file_simple() -> Result<String, io::Error> {
    println!("Please enter the name of the file you'd like to read:");

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;

    fs::read_to_string(input.trim()) // fs::read_to_string returns Result<String, io::Error>
}
