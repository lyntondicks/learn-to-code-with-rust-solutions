use std::fs;
use std::io;
use std::process;

pub fn chapter17_project_solution() {
    match write_to_file() {
        Ok(filename) => println!("Successfully wrote to file: {}", filename),
        Err(error) => {
            eprintln!("Error writing to file: {}", error);
            process::exit(1);
        }
    }
}

fn write_to_file() -> io::Result<String> {
    let input: io::Stdin = io::stdin();

    println!("What file would you like to write to?");
    let mut requested_file = String::new();
    input.read_line(&mut requested_file)?;

    println!("What would you like to write to the file?");
    let mut content = String::new();
    input.read_line(&mut content)?;

    fs::write(requested_file.trim(), content.trim())?;

    Ok(requested_file.trim().to_string())
}
