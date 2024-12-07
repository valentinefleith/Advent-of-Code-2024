//fn concatenate(uint1: u64, uint2: u64) -> u64 {
//    let uint1_string = uint1.to_string();
//    let uint2_string = uint2.to_string();
//    let concatenated_str = format!("{uint1_string}{uint2_string}");
//    concatenated_str.parse::<u64>().unwrap()
//}

fn concatenate(uint1: u64, uint2: u64) -> u64 {
    let digits = (uint2 as f64).log10().floor() as u32 + 1;
    uint1 * 10u64.pow(digits) + uint2
}

fn is_combination(test: &u64, remaining: &[u64], current_result: u64) -> bool {
    if remaining.is_empty() {
        return current_result == *test;
    }
    if current_result > *test {
        return false;
    }
    let next = &remaining[1..];
    is_combination(test, next, current_result + remaining[0])
        || is_combination(test, next, current_result * remaining[0])
        || is_combination(test, next, concatenate(current_result, remaining[0]))
}

pub fn get_right_combinations2(tested: Vec<u64>, remaining: Vec<Vec<u64>>) -> u64 {
    let mut result: u64 = 0;
    for (i, test) in tested.iter().enumerate() {
        if is_combination(test, &remaining[i][1..], remaining[i][0]) {
            result += test;
        }
    }
    result
}
