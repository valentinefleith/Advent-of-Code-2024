use cached::proc_macro::cached;
use std::fs;

const INPUT_PATH: &str = "input.txt";

fn read_input(filename: &str) -> String {
    fs::read_to_string(filename)
        .expect("File not found.")
        .to_owned()
}

fn parse_input(input: String) -> Vec<i32> {
    input
        .split_whitespace()
        .map(|n| n.parse().unwrap())
        .collect()
}

#[cached]
fn find_nb_stones_from_one(nb: i128, current_step: i32, total_blinks: i32) -> i64 {
    if current_step == total_blinks {
        return 1;
    }
    let mut nb_stones = 0;
    if nb == 0 {
        nb_stones += find_nb_stones_from_one(1, current_step + 1, total_blinks);
    } else if ((nb as f64).log10().floor() as u32 + 1) % 2 == 0 {
        let nb_digits = (nb as f64).log10().floor() as i128 + 1;
        let divisor = 10_i128.pow((nb_digits as u32) / 2);
        nb_stones += find_nb_stones_from_one(nb / divisor, current_step + 1, total_blinks);
        nb_stones += find_nb_stones_from_one(nb % divisor, current_step + 1, total_blinks);
    } else {
        nb_stones += find_nb_stones_from_one(nb * 2024, current_step + 1, total_blinks);
    }
    nb_stones
}

fn count_nb_stones(arrangement: Vec<i32>, total_blinks: i32) -> i64 {
    let mut count = 0;
    for nb in arrangement {
        println!("DEALING WITH {}", nb);
        count += find_nb_stones_from_one(nb as i128, 0, total_blinks);
    }
    count
}

fn main() {
    let input: String = read_input(INPUT_PATH);
    let vec_input = parse_input(input);
    let result = count_nb_stones(vec_input, 75);
    println!("{:?}", result);
}
