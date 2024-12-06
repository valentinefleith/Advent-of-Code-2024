use std::{
    collections::HashSet,
    fs,
    io::{prelude::*, BufReader},
};

use day06::map::Map;

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
    let mut map = Map::new(lines.len(), lines[0].len(), lines).unwrap();
    let guard_positions: HashSet<(i32, i32)> = map.find_route();
    let result = if PART == 1 {
        guard_positions.len()
    } else {
        map.count_loops(guard_positions.clone())
    };
    println!("{}", result);
}
