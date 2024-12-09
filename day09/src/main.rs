//use std::env;
use std::fs;

use day09::part1::get_compact_files;
use day09::part1::get_final_result;
use day09::part1::line_to_blocks;
use day09::part2::get_compact_p2;

const INPUT_PATH: &str = "input.txt";
const PART: u32 = 2;

fn read_input(filename: &str) -> String {
    fs::read_to_string(filename)
        .expect("File not found.")
        .trim()
        .to_owned()
}

fn main() {
    //env::set_var("RUST_BACKTRACE", "1");
    let line = read_input(INPUT_PATH);
    let block_representation = line_to_blocks(line);
    //println!("{:?}", block_representation);
    let compact = if PART == 1 {
        get_compact_files(block_representation)
    } else {
        get_compact_p2(block_representation)
    };
    println!("COMPACT = {:?}", compact);
    let result = get_final_result(compact);
    println!("{}", result);
}
