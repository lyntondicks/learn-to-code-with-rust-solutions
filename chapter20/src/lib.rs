mod fn_mut_trait;
mod fn_once_trait;

use fn_mut_trait::fn_mut_trait;
use fn_once_trait::fn_once_trait;

pub fn chapter20() {
    println!("Chapter 20: Closures");
    fn_once_trait();
    fn_mut_trait();
}
