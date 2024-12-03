use std::fs;

use day03::part1::get_only_mul;
use day03::part1::multiply_and_sum;

const INPUT_PATH: &str = "input.txt";
// const PART: u32 = 2;

fn read_input(filename: &str) -> String {
    fs::read_to_string(filename)
        .expect("File not found.")
        .to_owned()
}

fn main() {
    let content = read_input(INPUT_PATH);
    let mul_to_compute: Vec<(u32, u32)> = get_only_mul(&content);
    let result = multiply_and_sum(mul_to_compute);
    println!("{}", result);
}
