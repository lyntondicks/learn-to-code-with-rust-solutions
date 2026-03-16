mod fn_mut_trait;
mod fn_once_trait;
mod project_solution;

use fn_mut_trait::fn_mut_trait;
use fn_once_trait::fn_once_trait;
use project_solution::project_solution;

pub fn chapter20() {
    println!("Chapter 20: Closures");
    fn_once_trait();
    fn_mut_trait();
    fn_argument();
    project_solution();
}

#[allow(clippy::unwrap_or_default)]
fn fn_argument() {
    println!("Demonstrate passing a function argument to a fn trait param");
    let option: Option<Vec<String>> = None;
    let collection = option.unwrap_or_else(Vec::new); // Pass in Vec::new function as closure
    println!("{collection:?}");
}
