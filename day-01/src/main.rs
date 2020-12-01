use std::collections::HashSet;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    let values = read("input.txt");
    println!("part1 solution: {}", two_sum(&values));
    println!("part2 solution: {}", three_sum(&values));
}

fn two_sum(values: &HashSet<i32>) -> i32 {
    for &val in values.iter() {
        let target = 2020 - val;
        if values.contains(&target) {
            return val * target;
        }
    }
    panic!("Shouldn't reach this");
}

fn three_sum(values: &HashSet<i32>) -> i32 {
    let values_vec = values.iter().cloned().collect::<Vec<i32>>();
    for i in 1..values_vec.len() {
        for j in 0..i {
            let target = 2020 - values_vec[i] - values_vec[j];
            if values.contains(&target) {
                return values_vec[i] * values_vec[j] * target;
            }
        }
    }
    panic!("Shouldn't reach this");
}

fn read(filename: &str) -> HashSet<i32> {
    let mut file = File::open(filename).expect("File not found");
    let mut content = String::new();
    file.read_to_string(&mut content)
        .expect("Error while reading file content");

    content
        .split_terminator("\n")
        .map(|s| s.parse().unwrap())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_test() {
        let values = read("test-input.txt");
        assert_eq!(two_sum(&values), 514579);
    }

    #[test]
    fn part2_test() {
        let values = read("test-input.txt");
        assert_eq!(three_sum(&values), 241861950);
    }
}
