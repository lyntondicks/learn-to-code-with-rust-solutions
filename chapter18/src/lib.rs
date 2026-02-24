mod associated_constants;
mod clone_trait;
mod copy_trait;
mod display_trait;
mod eq_trait;
mod partial_eq;
mod trait_def_and_impl;
mod traits_must_be_in_scope;

use associated_constants::associated_constants;
use clone_trait::clone_trait;
use copy_trait::copy_trait;
use display_trait::display_trait;
use eq_trait::eq_trait;
use partial_eq::partial_eq;
use trait_def_and_impl::trait_def_and_impl;
use traits_must_be_in_scope::traits_must_be_in_scope;

pub fn chapter18() {
    println!("Chapter 18: Traits");
    trait_def_and_impl();
    associated_constants();
    traits_must_be_in_scope();
    display_trait();
    clone_trait();
    copy_trait();
    partial_eq();
    eq_trait();
}
