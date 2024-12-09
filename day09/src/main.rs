use std::fs;

use day09::part1::get_compact_files;
use day09::part1::get_final_result;
use day09::part1::line_to_blocks;

const INPUT_PATH: &str = "input.txt";
const PART: u32 = 1;

fn read_input(filename: &str) -> String {
    fs::read_to_string(filename)
        .expect("File not found.")
        .trim()
        .to_owned()
}

fn main() {
    let line = read_input(INPUT_PATH);
    let block_representation = line_to_blocks(line);
    let compact = get_compact_files(block_representation);
    let result = get_final_result(compact);
    println!("{}", result);
}
