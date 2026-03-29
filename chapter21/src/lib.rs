mod hashmap_iter;
mod into_iter;
mod iter;
mod iter_mut;

pub fn chapter21() {
    println!("Chapter 21: Iterators");
    into_iter::into_iter();
    iter::iter();
    iter_mut::iter_mut();
    hashmap_iter::hashmap_iter();
}
