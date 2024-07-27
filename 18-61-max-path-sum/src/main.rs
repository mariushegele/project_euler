use std::{fs, path::Path};

use max_path_sum::*;

fn main() {
    let triangle_file_name  = std::env::args().nth(1).expect("no path to triangle given");
    let triangle_path = Path::new(&triangle_file_name);
    let triangle_str = fs::read_to_string(triangle_path)
        .expect(&format!("failed to read triangle file {triangle_file_name}"));

    let triangle = Triangle::from_string(triangle_str);
    let max_path_sum = triangle.into_dag().max_path_sum();

    println!("The maximum path sum of the triangle is {max_path_sum}");
}
