use regex::Regex;

pub fn get_abled_mul(content: &String) -> String {
    let re_able = Regex::new(r"(do\(\))|(don't\(\))").unwrap();
    let mut results: Vec<String> = Vec::new();
    let mut last_end = 0;
    let mut to_push: bool = true;
    for mat in re_able.find_iter(content) {
        if to_push {
            results.push(content[last_end..mat.start()].to_string());
        }
        to_push = &content[mat.start()..mat.end()] == "do()";
        last_end = mat.end();
    }
    results.join("")
}
