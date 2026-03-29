use std::collections::HashMap;

#[allow(clippy::for_kv_map)]
pub fn hashmap_iter() {
    println!("Chapter 21: HashMap Iteration");

    let mut todos = HashMap::new();
    todos.insert("Pick up groceries", false);
    todos.insert("Study Rust", true);
    todos.insert("Sleep", false);

    for (_, completion_status) in &mut todos {
        *completion_status = true;
    }

    println!("{todos:?}");
}
