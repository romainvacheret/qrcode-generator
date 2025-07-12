use crate::{patterns::{PatternHelper}, utils::{Matrix, Pos}};

mod masking;
mod utils;
mod patterns;


fn main() {
    println!("Hello, world!");
    let mut matrix = Matrix::new(21 as usize);

    PatternHelper::new(false).apply_patterns(&mut matrix, 1);
    matrix.display();
}
