use std::fs;
use std::io;

pub fn lines() {
    println!("Chapter 21: Lines iterator");

    print_lines().expect("Reading lines from 'chapter21_story.txt' failed");
}

pub fn print_lines() -> io::Result<()> {
    let contents = fs::read_to_string("chapter21_story.txt")?; // ? try operator

    for line in contents.lines() {
        println!("{line}");
    }

    Ok(())
}
