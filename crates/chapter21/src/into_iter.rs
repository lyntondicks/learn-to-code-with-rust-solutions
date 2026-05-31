use std::collections::HashMap;

#[allow(unused_variables)]
pub fn into_iter() {
    println!("Into Iter");

    let my_vector = vec![1, 2, 3, 4, 5, 6];
    let my_iterator = my_vector.into_iter();
    // println!("{my_vector:?}");  // into_iter moves the vector

    let my_vector = vec![false, true, false];
    let my_iterator = my_vector.into_iter();

    let mut my_hashmap = HashMap::new();
    my_hashmap.insert("CBS", 2);
    let my_iterator = my_hashmap.into_iter();

    let my_vector = vec![1, 2];
    let mut my_iterator = my_vector.into_iter();
    println!("iterator {my_iterator:?}");
    println!("next: {:?}", my_iterator.next()); // Some(1)
    println!("iterator {my_iterator:?}");
    println!("next: {:?}", my_iterator.next()); // Some(2)
    println!("iterator {my_iterator:?}");
    println!("next: {:?}", my_iterator.next()); // None
    println!("iterator {my_iterator:?}");

    let my_vector = vec![1, 2, 3, 4, 5, 6];
    let my_iterator = my_vector.into_iter();
    // Iterator also implements IntoIter trait and returns itself
    for number in my_iterator {
        println!("{number}");
    }
    // println!("{my_iterator:?}"); // for loop took ownership of my_iterator

    let my_vector = vec![12, 123, 5, 4565];
    // for loop calls .into_iter implicitly on owned values
    for number in my_vector {
        println!("{number}");
    }
    // println!("{my_vector:?}"); // ownership moved to iterator and then to for loop
}
