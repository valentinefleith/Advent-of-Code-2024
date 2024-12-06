use std::collections::HashMap;

fn get_size_to_cut(lines: &Vec<String>) -> Result<usize, String> {
    let mut count: usize = 0;
    for line in lines {
        if line == "" {
            return Ok(count);
        }
        count += 1;
    }
    Err(format!("Delimiter not found."))
}

pub fn parse_lines(lines: Vec<String>) -> (Vec<Vec<i32>>, Vec<Vec<i32>>) {
    let size = get_size_to_cut(&lines).unwrap();
    let (slice1, slice2) = lines.split_at(size);
    let rules: Vec<Vec<i32>> = slice1
        .to_vec()
        .iter()
        .map(|l| {
            l.split("|")
                .map(String::from)
                .map(|n| n.parse().unwrap())
                .collect()
        })
        .collect();
    let updates: Vec<Vec<i32>> = slice2[1..]
        .to_vec()
        .iter()
        .map(|l| {
            l.split(",")
                .map(String::from)
                .map(|n| n.parse().unwrap())
                .collect()
        })
        .collect();
    (rules, updates)
}

fn create_rule_map(rules: &Vec<Vec<i32>>, update: &Vec<i32>) -> HashMap<i32, Vec<i32>> {
    let mut ordered_rules: HashMap<i32, Vec<i32>> = HashMap::new();
    for rule in rules {
        if update.contains(&rule[0]) && update.contains(&rule[1]) {
            let current_nb = rule[0];
            ordered_rules
                .entry(current_nb)
                .or_insert_with(Vec::new)
                .push(rule[1]);
        }
    }
    ordered_rules
}

fn is_correct(update: Vec<i32>, rules: &Vec<Vec<i32>>) -> bool {
    let rule_map = create_rule_map(rules, &update);
    let mut clone = update.clone();
    clone.sort_by(|a, b| {
        let order_a = rule_map.get(a).map(|v| v.len());
        let order_b = rule_map.get(b).map(|v| v.len());
        order_b.cmp(&order_a)
    });
    update == clone
}
fn get_correct_update_middle(update: Vec<i32>, rules: &Vec<Vec<i32>>) -> i32 {
    let rule_map = create_rule_map(rules, &update);
    let mut clone = update.clone();
    clone.sort_by(|a, b| {
        let order_a = rule_map.get(a).map(|v| v.len());
        let order_b = rule_map.get(b).map(|v| v.len());
        order_b.cmp(&order_a)
    });
    if update == clone {
        return 0;
    }
    clone[clone.len() / 2]
}

pub fn get_nb_good_updates(rules: Vec<Vec<i32>>, updates: Vec<Vec<i32>>) -> i32 {
    updates
        .iter()
        .filter(|u| is_correct((*u).to_vec(), &rules))
        .cloned()
        .map(|u| u[u.len() / 2])
        .sum()
}

pub fn get_nb_corrected_updates(rules: Vec<Vec<i32>>, updates: Vec<Vec<i32>>) -> i32 {
    updates
        .iter()
        .filter(|u| !is_correct((*u).to_vec(), &rules))
        .cloned()
        .map(|u| get_correct_update_middle(u, &rules))
        .sum()
}
