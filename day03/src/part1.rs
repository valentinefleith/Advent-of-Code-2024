use regex::Regex;

pub fn get_only_mul(content: &String) -> Vec<(u32, u32)> {
    let re = Regex::new(r"mul\(([0-9]{1,3}),([0-9]{1,3})\)").unwrap();
    let mut nbs: Vec<(u32, u32)> = vec![];
    for (_, [first, second]) in re.captures_iter(content).map(|c| c.extract()) {
        nbs.push((
            first.trim().parse::<u32>().expect("ERROR"),
            second.trim().parse::<u32>().unwrap(),
        ));
    }
    nbs
}

pub fn multiply_and_sum(nbs: Vec<(u32, u32)>) -> u32 {
    nbs.iter().map(|t| t.0 * t.1).sum::<u32>()
}
