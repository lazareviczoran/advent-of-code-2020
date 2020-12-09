use std::cmp::Ordering;
use std::fs::read_to_string;

fn main() {
    let data = read("input.txt");
    let res = find_first_invalid(&data, 25);
    println!("part1 solution: {}", res);
    println!("part2 solution: {}", find_encryption_weakness(&data, res));
}

fn find_first_invalid(data: &[usize], length: usize) -> usize {
    let res = data
        .windows(length)
        .enumerate()
        .find(|(first_pos, chunk)| !two_sum_exists(chunk, data[first_pos + length]));
    data[res.unwrap().0 + length]
}

fn find_encryption_weakness(data: &[usize], target: usize) -> usize {
    let (mut low, mut high, mut curr_res) = (0, 0, 0);
    loop {
        match curr_res.cmp(&target) {
            Ordering::Less => {
                curr_res += data[high];
                high += 1;
            }
            Ordering::Greater => {
                curr_res -= data[low];
                low += 1;
            }
            Ordering::Equal => {
                let (mut min, mut max) = (usize::MAX, usize::MIN);
                (low..high).for_each(|i| {
                    min = min.min(data[i]);
                    max = max.max(data[i]);
                });
                return min + max;
            }
        }
    }
}

fn two_sum_exists(data: &[usize], target: usize) -> bool {
    for i in 0..data.len() {
        for j in i + 1..data.len() {
            if data[i] + data[j] == target {
                return true;
            }
        }
    }
    false
}

fn read(filename: &str) -> Vec<usize> {
    read_to_string(filename)
        .expect("Failed to read file")
        .lines()
        .map(|s| s.parse().unwrap())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_test() {
        let data = read("test-input.txt");
        assert_eq!(find_first_invalid(&data, 5), 127);
    }

    #[test]
    fn part2_test() {
        let data = read("test-input.txt");
        assert_eq!(find_encryption_weakness(&data, 127), 62);
    }
}
