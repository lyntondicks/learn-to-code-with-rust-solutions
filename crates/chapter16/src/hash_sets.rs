use std::collections::HashSet;

pub fn hash_sets() {
    let mut concert_queue: HashSet<&str> = HashSet::new();
    concert_queue.insert("Alice");
    concert_queue.insert("Bob");
    concert_queue.insert("Charlie");
    println!("{:#?}", concert_queue);
    println!("{}", concert_queue.len());
    concert_queue.insert("Alice");
    println!("{:#?}", concert_queue);
    concert_queue.remove("Bob");
    println!("{:#?}", concert_queue);

    println!("{}", concert_queue.contains("Alice"));
    println!("{:#?}", concert_queue.get("Charlie"));

    let mut movie_queue: HashSet<&str> = HashSet::new();
    movie_queue.insert("Alice");
    movie_queue.insert("Bob");
    movie_queue.insert("Charlie");
    movie_queue.insert("David");

    println!("{:#?}", concert_queue.union(&movie_queue));
    println!("{:#?}", concert_queue.difference(&movie_queue));
    println!("{:#?}", concert_queue.symmetric_difference(&movie_queue));
    println!("{:#?}", concert_queue.is_disjoint(&movie_queue)); // sets have no elements in common
    println!("{:#?}", concert_queue.is_subset(&movie_queue)); // all elements in concert_queue are also in movie_queue
    println!("{:#?}", movie_queue.is_superset(&concert_queue)); // all elements in movie_queue are also in concert_queue
}
