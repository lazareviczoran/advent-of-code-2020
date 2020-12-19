use std::collections::HashMap;
use std::fs::read_to_string;

fn main() {
    let mut data = read("input.txt");
    println!("part1 solution: {}", count_matches(&data));
    println!("part2 solution: {}", count_matches2(&mut data));
}

fn count_matches(data: &(HashMap<usize, Vec<Value>>, Vec<String>)) -> usize {
    let mut memo = HashMap::new();
    data.1
        .iter()
        .filter(|msg| {
            let hits = matches_rule(&msg, &data.0, 0, &mut memo);
            !hits.is_empty() && hits.iter().any(|&i| i == msg.len())
        })
        .count()
}

fn count_matches2(data: &mut (HashMap<usize, Vec<Value>>, Vec<String>)) -> usize {
    data.0
        .insert(8, vec![Value::Rules(vec![42]), Value::Rules(vec![42, 8])]);
    data.0.insert(
        11,
        vec![Value::Rules(vec![42, 31]), Value::Rules(vec![42, 11, 31])],
    );
    count_matches(data)
}

fn matches_rule<'a>(
    message: &'a str,
    rules: &HashMap<usize, Vec<Value>>,
    rule: usize,
    memo: &mut HashMap<(&'a str, usize), Vec<usize>>,
) -> Vec<usize> {
    if let Some(found) = memo.get(&(message, rule)) {
        return found.clone();
    }
    let matched_positions = match rules.get(&rule) {
        Some(values) => values.iter().fold(Vec::new(), |mut acc, v| {
            match &v {
                Value::String(s) => {
                    if message.starts_with(s) {
                        acc.push(s.len());
                    }
                }
                Value::Rules(r_vec) => {
                    let mut candidates = vec![(message, 0)];
                    r_vec.iter().for_each(|r| {
                        let mut new_candidates = Vec::new();
                        for &(cand, cand_split_pos) in candidates.iter() {
                            let matches = matches_rule(cand, rules, *r, memo);
                            if matches.is_empty() {
                                continue;
                            }
                            new_candidates.extend(matches.iter().map(|&r| {
                                let (_, remaining) = cand.split_at(r);
                                (remaining, cand_split_pos + r)
                            }));
                        }
                        candidates = new_candidates
                    });
                    if !candidates.is_empty() {
                        acc.extend(candidates.iter().map(|&(_s, res)| res));
                    }
                }
            };
            acc
        }),
        None => Vec::new(),
    };
    memo.insert((message, rule), matched_positions.clone());
    matched_positions
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
                        v = Value::String(r.get(1..2).unwrap().to_string());
                    } else {
                        v = Value::Rules(
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
    let messages = parts[1].lines().map(|l| l.to_string()).collect();

    (rules, messages)
}

#[derive(Debug, Clone)]
enum Value {
    String(String),
    Rules(Vec<usize>),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let data = read("test-input.txt");
        assert_eq!(count_matches(&data), 2);
    }

    #[test]
    fn test2() {
        let mut data = read("test-input2.txt");
        assert_eq!(count_matches(&data), 3);
        assert_eq!(count_matches2(&mut data), 12);
    }
}
