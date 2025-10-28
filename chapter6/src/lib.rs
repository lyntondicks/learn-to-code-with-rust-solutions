pub fn chapter6() {
    println!("Chapter 6: Ownership, References, and Borrowing");
    strings();
    copy_trait();
    move_trait();
    clone_trait();
    references_and_borrowing();
}

fn strings() {
    let mut greeting = String::from("Hello");
    greeting.push_str(", world!");
    println!("{}", greeting);

    let name = "Alice";
    let age = 30;
    let introduction = format!("My name is {} and I am {} years old.", name, age);
    println!("{}", introduction);
}

fn copy_trait() {
    // stored on the stack, so it implements the Copy trait
    let x = 5; // i32 implements the Copy trait
    let y = x; // x is copied, not moved
    println!("x: {}, y: {}", x, y);

    let a = 10.5; // f64 also implements the Copy trait
    let b = a; // a is copied, not moved
    println!("a: {}, b: {}", a, b);
}

fn move_trait() {
    // stored on the heap, so it does not implement the Copy trait
    let s1 = String::from("Hello"); // String does not implement the Copy trait
    let s2 = s1; // s1 is moved to s2, s1 is no longer valid
    // println!("s1: {}", s1); // This would cause a compile-time error
    println!("s2: {}", s2); // s2 is valid and can be used

    drop(s2); // Explicitly dropping s2 to free memory
    // println!("s2 after drop: {}", s2); // This would cause a compile-time error as s2 is no longer valid after being dropped
}

fn clone_trait() {
    // Clone trait is used to create a deep copy of data
    let s1 = String::from("Hello, world!");
    let s2 = s1.clone(); // s1 is cloned to s2, both are valid
    println!("s1: {}, s2: {}", s1, s2); // Both s1 and s2 can be used
}

fn references_and_borrowing() {
    let my_stack_value = 2;
    let my_integer_reference = &my_stack_value; // Immutable reference to my_stack_value
    let _reference_copy = my_integer_reference; // Copying the reference, not the value, copy trait is implemented for references
    println!("Reference to stack value: {}", my_integer_reference); // References implement the Display trait so they will print the value they point to
    println!(
        "Use Dereference operator to access the value: {}",
        *my_integer_reference
    ); // Using dereference operator to access the value

    let my_heap_value = String::from("Toyota");
    let my_heap_reference = &my_heap_value; // Immutable reference to my_heap_value
    println!("Reference to heap value: {}", my_heap_reference); // References implement the Display trait so they will print the value they point to

    let oranges = String::from("Oranges");
    transfer_ownership(oranges); // Transferring ownership of oranges to the function
    // println!("Oranges after transfer: {}", oranges); // This would cause a compile-time error as oranges is no longer valid after being moved

    let mut cake = bake_cake(); // The function returns a String, which is owned by the caller
    println!("Cake baked: {}", cake); // The caller can use the returned value

    bake_cake(); // Calling the function without using the returned value, it will be dropped immediately
    let my_meal = String::from("Pasta");
    show_my_meal(&my_meal); // Borrowing my_meal, passing a reference to the function
    // The ownership of my_meal is not transferred, so it can still be used after borrowing
    println!("My meal after borrowing: {}", my_meal); // This is still valid

    add_flour(&mut cake); // Borrowing cake mutably, passing a mutable reference to the function
}

fn transfer_ownership(mut value: String) -> () {
    println!("Transferring ownership of value: {}", value);
    // The value is moved here, and the original owner can no longer use it
    value.push_str(" - Updated"); // Modifying the value
    println!("Value after modification: {}", value);
}

fn bake_cake() -> String {
    // let cake = String::from("Chocolate Mousse");
    // return cake;
    String::from("Chocolate Mousse") // Implicit return, no need for the return keyword
}

fn show_my_meal(meal: &String) -> () {
    println!("My meal is: {}", meal);
    // The meal is borrowed, so the ownership is not transferred
}

fn add_flour(meal: &mut String) -> () {
    meal.push_str(" with flour");
    println!("Meal after adding flour: {}", meal);
}
