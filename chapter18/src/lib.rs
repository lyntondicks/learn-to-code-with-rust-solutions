mod associated_constants;
mod trait_def_and_impl;

use associated_constants::associated_constants;
use trait_def_and_impl::trait_def_and_impl;

pub fn chapter18() {
    println!("Chapter 18: Traits");
    trait_def_and_impl();
    associated_constants();
}
