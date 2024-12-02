fn is_regular_safe(report: &Vec<i32>) -> bool {
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

fn is_almost_safe(report: &Vec<i32>) -> bool {
    for (i, _nb) in report.iter().enumerate() {
        let mut temp = report.clone();
        temp.remove(i);
        if is_regular_safe(&temp) {
            return true;
        }
    }
    false
}

fn check_safety(report: &Vec<i32>) -> bool {
    is_regular_safe(report) || is_almost_safe(report)
}

pub fn count_newsafe_reports(reports: Vec<Vec<i32>>) -> usize {
    reports.iter().filter(|rep| check_safety(rep)).count()
}
