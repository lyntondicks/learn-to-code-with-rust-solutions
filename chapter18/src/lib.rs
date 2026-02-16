mod associated_constants;
mod trait_def_and_impl;
mod traits_must_be_in_scope;

use associated_constants::associated_constants;
use trait_def_and_impl::trait_def_and_impl;
use traits_must_be_in_scope::traits_must_be_in_scope;

pub fn chapter18() {
    println!("Chapter 18: Traits");
    trait_def_and_impl();
    associated_constants();
    traits_must_be_in_scope();
}
