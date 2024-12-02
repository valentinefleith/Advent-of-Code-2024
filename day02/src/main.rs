use std::{
    fs,
    io::{prelude::*, BufReader},
};

use day02::part1::count_safe_reports;
use day02::part1::lines_to_reports;

const INPUT_PATH: &str = "input.txt";
// const PART: u32 = 1;

fn read_input(filename: &str) -> Vec<String> {
    let file = fs::File::open(filename).expect("No such file.");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|line| line.expect("Could not parse line."))
        .collect()
}

fn main() {
    let lines = read_input(INPUT_PATH);
    let reports: Vec<Vec<i32>> = lines_to_reports(lines);
    let nb_safe = count_safe_reports(reports);
    println!("{}", nb_safe);
}
