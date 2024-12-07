pub fn parse_lines(lines: Vec<String>) -> (Vec<u64>, Vec<Vec<u64>>) {
    let mut tested: Vec<u64> = vec![];
    let mut remaining: Vec<Vec<u64>> = vec![];
    for line in lines {
        let parts: Vec<&str> = line.split(":").collect();
        tested.push(parts[0].parse::<u64>().unwrap());
        remaining.push(
            parts[1]
                .split_whitespace()
                .map(|n| n.parse::<u64>().unwrap())
                .collect(),
        )
    }
    (tested, remaining)
}

fn is_combination(test: &u64, remaining: Vec<u64>, current_result: u64) -> bool {
    if remaining.is_empty() {
        return current_result == *test;
    }
    is_combination(test, remaining[1..].to_vec(), current_result + remaining[0])
        || is_combination(test, remaining[1..].to_vec(), current_result * remaining[0])
}

pub fn get_right_combinations(tested: Vec<u64>, remaining: Vec<Vec<u64>>) -> u64 {
    let mut result: u64 = 0;
    for (i, test) in tested.iter().enumerate() {
        if is_combination(test, remaining[i][1..].to_vec(), remaining[i][0]) {
            result += test;
        }
    }
    result
}
