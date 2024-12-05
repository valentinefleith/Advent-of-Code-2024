use std::{
    fs,
    io::{prelude::*, BufReader},
};

use day04::grid::Grid;

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
    let grid = Grid::new(lines.len(), lines[0].len(), lines).unwrap();
    let count = if PART == 1 {
        grid.count_word(String::from("XMAS"))
    } else {
        grid.count_crosses(String::from("MAS"))
    };
    println!("{}", count);
}
