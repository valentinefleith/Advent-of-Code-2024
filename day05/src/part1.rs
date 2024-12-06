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

pub fn create_rule_map(rules: Vec<Vec<i32>>) -> HashMap<i32, Vec<i32>> {
    let mut ordered_rules: HashMap<i32, Vec<i32>> = HashMap::new();
    for rule in rules {
        let current_nb = rule[0];
        ordered_rules
            .entry(current_nb)
            .or_insert_with(Vec::new)
            .push(rule[1]);
    }
    for (k, v) in ordered_rules.iter_mut() {
        println!("key = {}, len_val = {}", k, v.len());
    }
    ordered_rules
}

fn is_correct(update: Vec<i32>, rule_map: &HashMap<i32, Vec<i32>>) -> bool {
    let mut clone = update.clone();
    clone.sort_by(|a, b| {
        let order_a = rule_map.get(a).map(|v| v.len());
        let order_b = rule_map.get(b).map(|v| v.len());
        //println!("{}", order_a.unwrap());
        //println!("{}", order_a.unwrap());
        order_b.cmp(&order_a)
    });
    //println!("Update = {:?}, sorted = {:?}", update, clone);
    update == clone
}

pub fn get_nb_good_updates(rule_map: HashMap<i32, Vec<i32>>, updates: Vec<Vec<i32>>) -> i32 {
    //println!("HASHMAP = {:?}", rule_map);
    //let correct_updates: Vec<Vec<i32>> = updates.iter().filter(|u|is_correct((*u).to_vec(), &rule_map)).cloned().collect();
    //println!("CORRECT UPDATES:\n{:?}\nlen={}", correct_updates, correct_updates.len());

    updates
        .iter()
        .filter(|u| is_correct((*u).to_vec(), &rule_map))
        .cloned()
        .map(|u| u[u.len() / 2])
        .sum()

    //map(u|u[u.len()/2]).sum()
}
