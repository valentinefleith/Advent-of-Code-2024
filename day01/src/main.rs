use std::{
    fs,
    io::{prelude::*, BufReader},
};

use day01::part1::compute_distance_between_lists;
use day01::part1::get_the_lists;
use day01::part2::get_similarity;

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
    if PART == 1 {
        let total_distance = compute_distance_between_lists(lines);
        println!("{}", total_distance);
    } else {
        let (list1, list2) = get_the_lists(lines);
        let similarity = get_similarity(list1, list2);
        println!("{}", similarity);
    }
}
