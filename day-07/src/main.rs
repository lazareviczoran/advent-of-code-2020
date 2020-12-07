use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    let rules = read("input.txt");

    println!(
        "part1 solution: {}",
        find_possibilities_count(&rules, "shiny gold")
    );
    println!(
        "part2 solution: {}",
        find_count_within(&rules, "shiny gold")
    );
}

fn find_possibilities_count(rules: &HashMap<String, Vec<(usize, String)>>, target: &str) -> usize {
    rules
        .iter()
        .filter(|rule| rule.0 != target && contains(rules, rule.0, target))
        .count()
}

fn contains(rules: &HashMap<String, Vec<(usize, String)>>, rule: &str, target_color: &str) -> bool {
    rule == target_color
        || rules.contains_key(rule)
            && rules
                .get(rule)
                .unwrap()
                .iter()
                .any(|(_, r)| contains(rules, r, target_color))
}

fn find_count_within(rules: &HashMap<String, Vec<(usize, String)>>, target: &str) -> usize {
    let msg = format!("No {} bag", target);
    let next = rules.get(target).expect(&msg);
    next.iter()
        .map(|(count, child)| count + count * find_count_within(rules, child))
        .sum()
}

fn read(filename: &str) -> HashMap<String, Vec<(usize, String)>> {
    let mut file = File::open(filename).expect("File not found");
    let mut content = String::new();
    file.read_to_string(&mut content)
        .expect("Failed to read file");
    let mut rules = HashMap::new();
    content.lines().for_each(|s| {
        let values: Vec<&str> = s.split_terminator(" bags contain ").collect();
        let color = values[0].into();
        let mut contains = Vec::new();
        if !values[1].starts_with("no other") {
            contains = values[1]
                .split_terminator(',')
                .map(|st| {
                    let required_vals: Vec<&str> = st.trim().splitn(2, ' ').collect();
                    (
                        required_vals[0].parse().unwrap(),
                        required_vals[1].rsplitn(2, ' ').nth(1).unwrap().into(),
                    )
                })
                .collect();
        }
        rules.insert(color, contains);
    });
    rules
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_test() {
        let rules = read("test-input.txt");
        assert_eq!(find_possibilities_count(&rules, "shiny gold"), 4);
    }

    #[test]
    fn part2_test1() {
        let rules = read("test-input.txt");
        assert_eq!(find_count_within(&rules, "shiny gold"), 32);
    }

    #[test]
    fn part2_test2() {
        let rules = read("test-input2.txt");
        assert_eq!(find_count_within(&rules, "shiny gold"), 126);
    }
}
