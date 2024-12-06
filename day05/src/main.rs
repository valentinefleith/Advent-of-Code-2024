use std::{
    fs,
    io::{prelude::*, BufReader},
};

use day05::part1::create_rule_map;
use day05::part1::get_nb_good_updates;
use day05::part1::parse_lines;

const INPUT_PATH: &str = "input.txt";
//const PART: u32 = 2;

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
    let rule_map = create_rule_map(rules);
    let result = get_nb_good_updates(rule_map, updates);
    println!("Result:\n{:?}", result);
    // println!("Hello, world!");
}
