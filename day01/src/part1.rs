pub fn get_the_lists(input_content: Vec<String>) -> (Vec<i32>, Vec<i32>) {
    let split_lines: Vec<Vec<String>> = input_content
        .iter()
        .map(|line| line.split_whitespace().map(String::from).collect())
        .collect();
    let mut list1: Vec<i32> = split_lines
        .iter()
        .map(|l| l[0].parse::<i32>().unwrap())
        .collect();
    let mut list2: Vec<i32> = split_lines
        .iter()
        .map(|l| l[1].parse::<i32>().unwrap())
        .collect();
    list1.sort();
    list2.sort();
    (list1, list2)
}

fn get_total_distance(list1: Vec<i32>, list2: Vec<i32>) -> i32 {
    let total_distances: Vec<i32> = list1
        .iter()
        .zip(list2)
        .map(|(a, b)| (a - b).abs())
        .collect();
    total_distances.iter().sum()
}

pub fn compute_distance_between_lists(input_content: Vec<String>) -> i32 {
    let (list1, list2) = get_the_lists(input_content);
    let total_distance: i32 = get_total_distance(list1, list2);
    total_distance
}
