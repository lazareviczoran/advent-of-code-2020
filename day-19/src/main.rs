use itertools::Itertools;
use std::collections::HashMap;
use std::fs::read_to_string;

fn main() {
    let data = read("input.txt");
    println!(
        "part1 solution: {}",
        count_matches(&data, &|&(count42, count31)| count42 == 2 && count31 == 1)
    );

    println!(
        "part2 solution: {}",
        count_matches(&data, &|&(count42, count31)| count31 > 0
            && count42 > count31)
    );
}

fn count_matches(
    data: &(HashMap<usize, Vec<Value>>, Vec<String>),
    filter_fn: &dyn Fn(&(usize, usize)) -> bool,
) -> usize {
    let (rules, messages) = data;
    let mut memo = HashMap::new();
    let possibilities_42 = generate_possibilities(&rules, &mut memo, 42);
    let possibilities_31 = generate_possibilities(&rules, &mut memo, 31);

    messages
        .iter()
        .filter_map(|message| {
            let mut s = message.get(..).unwrap();
            let mut count42 = 0;
            while let Some(pos_position) = possibilities_42.iter().position(|r| s.starts_with(r)) {
                s = s.strip_prefix(&possibilities_42[pos_position]).unwrap();
                count42 += 1;
            }
            if count42 == 0 {
                return None;
            }
            let mut count31 = 0;
            while let Some(pos_position) = possibilities_31.iter().position(|r| s.starts_with(r)) {
                s = s.strip_prefix(&possibilities_31[pos_position]).unwrap();
                count31 += 1;
            }

            if count31 == 0 || !s.is_empty() {
                return None;
            }
            Some((count42, count31))
        })
        .filter(filter_fn)
        .count()
}

#[allow(dead_code)]
fn count_matches_generated(data: &(HashMap<usize, Vec<Value>>, Vec<String>)) -> usize {
    let (rules, messages) = data;
    let mut memo = HashMap::new();
    let possibilities = generate_possibilities(&rules, &mut memo, 0);

    messages
        .iter()
        .filter(|message| possibilities.contains(message))
        .count()
}

fn generate_possibilities(
    rules_map: &HashMap<usize, Vec<Value>>,
    memo: &mut HashMap<usize, Vec<String>>,
    curr_rule: usize,
) -> Vec<String> {
    if let Some(values) = memo.get(&curr_rule) {
        return values.clone();
    }
    let mut res: Vec<String> = Vec::new();
    if let Some(rules) = rules_map.get(&curr_rule) {
        match &rules[0] {
            Value::Char(ch) => res.push(String::from(*ch)),
            Value::Other(subrule) => {
                let values = subrule
                    .iter()
                    .map(|r| generate_possibilities(rules_map, memo, *r))
                    .collect::<Vec<_>>();
                res.extend(values.iter().multi_cartesian_product().map(|product| {
                    product.iter().fold(String::new(), |mut acc, curr| {
                        acc.push_str(&curr);
                        acc
                    })
                }));
                if rules.len() > 1 {
                    match &rules[1] {
                        Value::Other(subrule2) => {
                            let values = subrule2
                                .iter()
                                .map(|r| generate_possibilities(rules_map, memo, *r))
                                .collect::<Vec<_>>();
                            res.extend(values.iter().multi_cartesian_product().map(|product| {
                                product.iter().fold(String::new(), |mut acc, curr| {
                                    acc.push_str(&curr);
                                    acc
                                })
                            }));
                        }
                        _ => {
                            println!("char appeared when multiple rules");
                        }
                    }
                }
            }
        }
    }
    memo.insert(curr_rule, res.clone());
    res
}

fn read(filename: &str) -> (HashMap<usize, Vec<Value>>, Vec<String>) {
    let content = read_to_string(filename).expect("Failed to read file");
    let parts = content.split_terminator("\n\n").collect::<Vec<&str>>();
    let rules = parts[0]
        .lines()
        .map(|l| {
            let items = l.split_terminator(": ").collect::<Vec<&str>>();
            let rules = items[1]
                .split_terminator(" | ")
                .map(|r| {
                    let v: Value;
                    if r.starts_with('"') {
                        v = Value::Char(r.chars().nth(1).unwrap());
                    } else {
                        v = Value::Other(
                            r.split_whitespace()
                                .filter_map(|val| val.parse().ok())
                                .collect(),
                        );
                    }
                    v
                })
                .collect();
            (items[0].parse().unwrap(), rules)
        })
        .collect();
    let messages = parts[1].lines().map(|l| l.chars().collect()).collect();

    (rules, messages)
}

#[derive(Debug, Clone)]
enum Value {
    Char(char),
    Other(Vec<usize>),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let data = read("test-input.txt");
        assert_eq!(count_matches_generated(&data), 2);
    }

    #[test]
    fn test2() {
        let data = read("test-input2.txt");
        assert_eq!(
            count_matches(&data, &|&(count42, count31)| count42 == 2 && count31 == 1),
            3
        );

        assert_eq!(
            count_matches(&data, &|&(count42, count31)| count31 > 0
                && count42 > count31),
            12
        );
    }
}
