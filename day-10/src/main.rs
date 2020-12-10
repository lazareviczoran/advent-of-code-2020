use std::collections::HashMap;
use std::fs::read_to_string;

fn main() {
    let data = read("input.txt");
    let diffs = find_adapter_distribution(&data);
    println!("part1 solution {}", diffs[1] * diffs[3]);
    println!("part2 solution {}", find_distinct_arrangements(&data));
}

fn find_adapter_distribution(data: &[usize]) -> [usize; 4] {
    data.windows(2).fold([0; 4], |mut acc, values| {
        acc[values[1] - values[0]] += 1;
        acc
    })
}

fn find_distinct_arrangements(data: &[usize]) -> usize {
    find_valid_arrangements(data, 0, &mut HashMap::new())
}

fn find_valid_arrangements(data: &[usize], i: usize, memo: &mut HashMap<usize, usize>) -> usize {
    if i == data.len() - 1 {
        return 1;
    }
    if let Some(val) = memo.get(&i) {
        return *val;
    }
    let mut count = 0;
    for (pos, &val) in data.iter().enumerate().skip(i + 1) {
        if data[i] + 3 < val {
            break;
        }
        count += find_valid_arrangements(data, pos, memo);
    }
    memo.insert(i, count);
    count
}

fn read(filename: &str) -> Vec<usize> {
    let content = read_to_string(filename).expect("Failed to read file");

    let mut adapters = content
        .lines()
        .map(|s| s.parse().unwrap())
        .collect::<Vec<_>>();
    adapters.sort_unstable();
    adapters.insert(0, 0);
    adapters.push(adapters.last().unwrap() + 3);
    adapters
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let data = read("test-input.txt");
        let diffs = find_adapter_distribution(&data);
        assert_eq!(diffs[1] * diffs[3], 35);
        assert_eq!(find_distinct_arrangements(&data), 8);
    }

    #[test]
    fn test2() {
        let data = read("test-input2.txt");
        let diffs = find_adapter_distribution(&data);
        assert_eq!(diffs[1] * diffs[3], 220);
        assert_eq!(find_distinct_arrangements(&data), 19208);
    }
}
