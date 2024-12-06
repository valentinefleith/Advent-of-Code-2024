use std::{
    fs,
    io::{prelude::*, BufReader},
};

use day05::solution::get_nb_corrected_updates;
use day05::solution::get_nb_good_updates;
use day05::solution::parse_lines;

const INPUT_PATH: &str = "input.txt";
const PART: u32 = 2;

fn read_input(filename: &str) -> Vec<String> {
    let file = fs::File::open(filename).expect("No such file.");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|line| line.expect("Could not parse line."))
        .collect()
}

fn main() {
    let lines = read_input(INPUT_PATH);
    let (rules, updates) = parse_lines(lines);
    // let result = get_nb_good_updates(rules, updates);
    let result = if PART == 1 {
        get_nb_good_updates(rules, updates)
    } else {
        get_nb_corrected_updates(rules, updates)
    };
    println!("Result:\n{:?}", result);
}
