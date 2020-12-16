use std::collections::{BTreeMap, HashSet, VecDeque};
use std::fs::read_to_string;
use std::ops::RangeInclusive;

fn main() {
    let data = read("input.txt");
    println!(
        "part1 solution: {:?}",
        count_invalid_values_in_other_tickets(&data)
    );
    println!("part2 solution: {:?}", calculate_departure_value(&data));
}

fn find_fields_order(data: &Data) -> BTreeMap<usize, usize> {
    let valid_tickets = filter_tickets(&data, true);
    let mut candidates = (0..data.rules.len())
        .map(|i| {
            (0..data.rules.len())
                .filter(|&j| {
                    valid_tickets
                        .iter()
                        .all(|ticket| data.rules[j].is_valid(&ticket[i]))
                })
                .collect::<HashSet<usize>>()
        })
        .enumerate()
        .collect::<VecDeque<(usize, HashSet<usize>)>>();

    let mut order = BTreeMap::new();
    while !candidates.is_empty() {
        candidates.rotate_left(
            candidates
                .iter()
                .position(|(_, set)| set.len() == 1)
                .unwrap(),
        );
        if let Some((new_pos, curr_set)) = candidates.pop_front() {
            let orig_pos = *curr_set.iter().next().unwrap();
            order.insert(new_pos, orig_pos);
            candidates.iter_mut().for_each(|(_, set)| {
                set.remove(&orig_pos);
            });
        }
    }
    order
}

fn calculate_departure_value(data: &Data) -> usize {
    find_fields_order(&data)
        .iter()
        .filter_map(|(&new_pos, &orig_pos)| {
            if data.rules[orig_pos].name.starts_with("departure") {
                Some(data.my_ticket[new_pos])
            } else {
                None
            }
        })
        .product()
}

fn count_invalid_values_in_other_tickets(data: &Data) -> usize {
    let invalid_tickets = filter_tickets(&data, false);
    invalid_tickets
        .iter()
        .map(|vals| {
            vals.iter()
                .filter(|v| !data.rules.iter().any(|rule| rule.is_valid(v)))
                .sum::<usize>()
        })
        .sum()
}

fn filter_tickets(data: &Data, is_valid: bool) -> Vec<Vec<usize>> {
    data.other_tickets
        .iter()
        .filter(|vals| {
            vals.iter()
                .any(|v| !data.rules.iter().any(|rule| rule.is_valid(v)))
                ^ is_valid
        })
        .cloned()
        .collect::<Vec<Vec<usize>>>()
}

struct Data {
    rules: Vec<Rule>,
    my_ticket: Vec<usize>,
    other_tickets: Vec<Vec<usize>>,
}

struct Rule {
    name: String,
    ranges: Vec<RangeInclusive<usize>>,
}
impl Rule {
    pub fn new(name: String, ranges: Vec<RangeInclusive<usize>>) -> Self {
        Self { name, ranges }
    }

    pub fn is_valid(&self, value: &usize) -> bool {
        self.ranges.iter().any(|r| r.contains(&value))
    }
}

fn read(filename: &str) -> Data {
    let content = read_to_string(filename).expect("Failed to read file");
    let parts = content.split_terminator("\n\n").collect::<Vec<&str>>();
    let rules = parts[0]
        .lines()
        .map(|s| {
            let s_parts = s.split_terminator(": ").collect::<Vec<&str>>();
            let name = s_parts[0].to_string();
            let ranges: Vec<RangeInclusive<usize>> = s_parts[1]
                .split_terminator(" or ")
                .map(|r| {
                    let range = r
                        .split_terminator('-')
                        .map(|val| val.parse().unwrap())
                        .collect::<Vec<usize>>();
                    range[0]..=range[1]
                })
                .collect();
            Rule::new(name, ranges)
        })
        .collect();

    let temp = parts[1].lines().nth(1).unwrap();
    let my_ticket = temp
        .split_terminator(',')
        .filter_map(|v| v.parse().ok())
        .collect::<Vec<usize>>();

    let temp = parts[2].lines().skip(1).collect::<Vec<&str>>();
    let other_tickets = temp
        .iter()
        .map(|p| {
            p.split_terminator(',')
                .filter_map(|v| v.parse().ok())
                .collect::<Vec<usize>>()
        })
        .collect();

    Data {
        rules,
        my_ticket,
        other_tickets,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let data = read("test-input.txt");
        assert_eq!(count_invalid_values_in_other_tickets(&data), 71);
        let valid_tickets = filter_tickets(&data, true);
        assert_eq!(valid_tickets, [[7, 3, 47]]);
        let invalid_tickets = filter_tickets(&data, false);
        assert_eq!(invalid_tickets, [[40, 4, 50], [55, 2, 20], [38, 6, 12]]);
    }

    #[test]
    fn test2() {
        let data = read("test-input2.txt");
        let order = find_fields_order(&data);
        assert_eq!(
            order
                .values()
                .map(|&i| data.rules[i].name.clone())
                .collect::<Vec<String>>(),
            ["departure row", "class", "departure seat"]
                .iter()
                .map(|s| s.to_string())
                .collect::<Vec<String>>()
        );
        assert_eq!(calculate_departure_value(&data), 11 * 13);
    }
}
