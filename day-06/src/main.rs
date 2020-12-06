use std::collections::HashSet;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    let data = read("input.txt");
    println!("part1 solution: {}", count_all_unique_yes_answers(&data));

    println!("part2 solution: {}", count_unique_yes_answers_by_all(&data));
}

fn count_all_unique_yes_answers(data: &[Vec<String>]) -> usize {
    data.iter()
        .map(|answers| {
            answers.iter().fold(HashSet::new(), |mut unique, ans| {
                ans.chars().for_each(|ch| {
                    unique.insert(ch);
                });
                unique
            })
        })
        .map(|answers| answers.len())
        .sum()
}

fn count_unique_yes_answers_by_all(data: &[Vec<String>]) -> usize {
    data.iter()
        .map(|answers| {
            let initial_set: HashSet<char> = answers[0].chars().collect();
            answers.iter().skip(1).fold(initial_set, |unique, ans| {
                let curr_set = ans.chars().collect();
                unique.intersection(&curr_set).cloned().collect()
            })
        })
        .map(|answers| answers.len())
        .sum()
}

fn read(filename: &str) -> Vec<Vec<String>> {
    let mut file = File::open(filename).expect("File not found");
    let mut content = String::new();
    file.read_to_string(&mut content)
        .expect("Failed to read file content");

    let mut acc = Vec::new();
    let mut answers = Vec::new();
    content.lines().for_each(|s| {
        if !s.is_empty() {
            answers.push(s.to_string());
        } else if !answers.is_empty() {
            acc.push(answers.clone());
            answers.clear();
        }
    });
    acc.push(answers);
    acc
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_test() {
        let data = read("test-input.txt");
        assert_eq!(count_all_unique_yes_answers(&data), 11);
    }

    #[test]
    fn part2_test() {
        let data = read("test-input.txt");
        assert_eq!(count_unique_yes_answers_by_all(&data), 6);
    }
}
