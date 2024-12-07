use std::{
    fs,
    io::{prelude::*, BufReader},
};

use day07::part1::get_right_combinations;
use day07::part1::parse_lines;

const INPUT_PATH: &str = "input.txt";
const PART: u32 = 1;

fn read_input(filename: &str) -> Vec<String> {
    let file = fs::File::open(filename).expect("No such file.");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|line| line.expect("Could not parse line."))
        .collect()
}

fn main() {
    let lines = read_input(INPUT_PATH);
    let (tested, remaining) = parse_lines(lines);
    let result: u64 = if PART == 1 {
        get_right_combinations(tested, remaining)
    } else {
        0
    };
    println!("{}", result);
}
