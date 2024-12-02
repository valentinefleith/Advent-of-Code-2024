pub fn lines_to_reports(lines: Vec<String>) -> Vec<Vec<i32>> {
    lines
        .iter()
        .map(|line| {
            line.split_whitespace()
                .map(String::from)
                .map(|n| n.parse().unwrap())
                .collect()
        })
        .collect()
}

fn is_safe(report: &Vec<i32>) -> bool {
    let nb_of_elems = report.len();
    if nb_of_elems <= 2 {
        return true;
    }
    let order: i32 = if report[1] - report[0] > 0 { 1 } else { -1 };
    for (i, _nb) in report.iter().enumerate() {
        if i == nb_of_elems - 1 {
            return true;
        }
        let abs_adj_difference: i32 = (report[i + 1] - report[i]) * order;
        if abs_adj_difference < 1 || abs_adj_difference > 3 {
            return false;
        }
    }
    true
}

pub fn count_safe_reports(reports: Vec<Vec<i32>>) -> usize {
    reports.iter().filter(|rep| is_safe(rep)).count()
}
