#![allow(dead_code, unused_variables)]
mod arrays;
mod conditionals;
mod data_types;
mod operators;
mod range;
mod tuples;

pub fn main() {
    println!("Chapter 3: Data types");
    data_types::main();
    operators::main();
    conditionals::main();
    arrays::main();
    tuples::main();
    range::main();
}
