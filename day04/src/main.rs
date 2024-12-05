use std::{
    fs,
    io::{prelude::*, BufReader},
};

// use day02::part1::count_safe_reports;
use day04::grid::Grid;

const INPUT_PATH: &str = "input.txt";
// const PART: u32 = 2;

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
    let count = grid.count_word(String::from("XMAS"));
    println!("{}", count);
}
