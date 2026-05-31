use std::env;

// run with: cargo run -p chapter21 --bin cmd_line_args_1 -- arg1 arg2
pub fn main() {
    let args = env::args();

    for arg in args {
        println!("{arg}");
    }
}
