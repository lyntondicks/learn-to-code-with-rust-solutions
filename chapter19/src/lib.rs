//! # Chapter 19: References and Lifetimes
//! This module contains examples and explanations of references and lifetimes in Rust.

mod references_as_function_parameters;
use crate::references_as_function_parameters::references_as_function_parameters;

pub fn chapter19() {
    println!("Chapter 19: References and Lifetimes");
    references_as_function_parameters();
}

#[derive(Debug)]
#[allow(dead_code)]
struct TrainStation<'a> {
    name: &'a str,
}
