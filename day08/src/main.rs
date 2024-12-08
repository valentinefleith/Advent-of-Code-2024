use std::{
    fs,
    io::{prelude::*, BufReader},
};

use day08::map::Map;

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
    let map = Map::new(lines.len(), lines[0].len(), lines).unwrap();
    let result = if PART == 1 { map.count_antinodes() } else { 0 };
    println!("{:?}", result);
}
