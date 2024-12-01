pub fn get_similarity(list1: Vec<i32>, list2: Vec<i32>) -> i32 {
    let mut counts: Vec<i32> = vec![];
    for nb in &list1 {
        let count: usize = list2.iter().filter(|n| *n == nb).count();
        counts.push(count.try_into().unwrap());
    }
    let scores: Vec<i32> = list1.iter().zip(counts).map(|(a, b)| a * b).collect();
    scores.iter().sum()
}
