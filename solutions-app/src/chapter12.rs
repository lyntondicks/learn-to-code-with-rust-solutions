#![allow(dead_code)]

mod option_enum;
mod project_solution;
mod result_enum;

use option_enum::option_enum;
use project_solution::chapter_12_project_solution;
use result_enum::result_enum;

pub fn main() {
    option_enum();
    result_enum();
    chapter_12_project_solution();
}
