mod hashmap_iter;
mod into_iter;
mod iter;
mod iter_mut;
mod solving_problem_word_count;
mod string_iteration;

pub fn chapter21() {
    println!("Chapter 21: Iterators");
    into_iter::into_iter();
    iter::iter();
    iter_mut::iter_mut();
    hashmap_iter::hashmap_iter();
    string_iteration::string_iteration();
    solving_problem_word_count::solving_problem_word_count();
}
