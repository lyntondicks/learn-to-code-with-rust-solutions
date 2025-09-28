pub fn main() {
    println!("Chapter 4: Functions");
    open_store("Downtown", 5);    
}

fn open_store(neighborhood: &str, store_count: u32) -> () {
    println!("Opening store in {neighborhood} neighborhood.");
    let result = square(13);
    println!("The square of 13 is: {result}");
}

fn square(x: i32) -> i32 {
    // return x * x;
    x * x // implied return, no need for the return keyword
}
