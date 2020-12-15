use std::collections::HashMap;

fn main() {
    let data = [10, 16, 6, 0, 1, 17];
    println!("part1 solution: {:?}", get_nth_value(&data, 2020));
    println!("part2 solution: {:?}", get_nth_value(&data, 30000000));
}

fn get_nth_value(data: &[usize], n: usize) -> usize {
    let mut values = data
        .iter()
        .enumerate()
        .map(|(i, &val)| (val, i + 1))
        .collect::<HashMap<usize, usize>>();
    let length = data.len();
    let mut prev = data[length - 1];
    for i in length..n {
        prev = i - values.insert(prev, i).unwrap_or(i);
    }
    prev
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let data = [0, 3, 6];
        assert_eq!(get_nth_value(&data, 2020), 436);
    }
}
