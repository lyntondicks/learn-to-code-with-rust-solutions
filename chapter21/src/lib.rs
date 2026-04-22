mod any_and_all;
mod cloned;
mod enumerate;
mod filter_and_find;
mod filter_map;
mod flat_map;
mod flatten;
mod fold;
mod hashmap_iter;
mod into_iter;
mod iter;
mod iter_mut;
mod map_adapter_and_collect;
mod partition;
mod solving_problem_word_count;
mod string_iteration;
mod zip;

pub fn chapter21() {
    println!("Chapter 21: Iterators");
    into_iter::into_iter();
    iter::iter();
    iter_mut::iter_mut();
    hashmap_iter::hashmap_iter();
    string_iteration::string_iteration();
    solving_problem_word_count::solving_problem_word_count();
    map_adapter_and_collect::map_adapter_and_collect();
    filter_and_find::filter_and_find();
    any_and_all::any_and_all();
    cloned::cloned();
    filter_map::filter_map();
    flatten::flatten();
    flat_map::flat_map();
    enumerate::enumerate();
    partition::partition();
    zip::zip();
    fold::fold();
}
