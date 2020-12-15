fn main() {
    let data = [10, 16, 6, 0, 1, 17];
    println!("part1 solution: {:?}", get_nth_value(&data, 2020));
    println!("part2 solution: {:?}", get_nth_value(&data, 30000000));
}

fn get_nth_value(data: &[usize], n: usize) -> usize {
    let mut values = vec![0; n];
    for (i, &val) in data.iter().enumerate() {
        values[val] = i + 1;
    }
    let mut prev = *data.last().expect("vec is empty");
    for i in data.len()..n {
        let v = std::mem::replace(&mut values[prev], i);
        prev = if v == 0 { 0 } else { i - v };
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
