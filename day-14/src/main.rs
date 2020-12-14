use std::collections::HashMap;
use std::fs::read_to_string;

fn main() {
    let data = read("input.txt");
    let mut memory = HashMap::new();
    println!("part1 solution: {}", run_and_sum_values(&mut memory, &data));

    memory.clear();
    println!(
        "part2 solution: {}",
        run_and_sum_values2(&mut memory, &data)
    );
}

fn run_and_sum_values(
    memory: &mut HashMap<u128, u128>,
    data: &[(String, Vec<(u128, u128)>)],
) -> u128 {
    for (mask, instructions) in data {
        for &(addr, new_val) in instructions {
            let mut val = new_val;
            for (i, ch) in mask.chars().rev().enumerate() {
                match ch {
                    '1' => val |= 1 << i,
                    '0' => val &= !(1 << i),
                    _ => {}
                }
            }
            memory.insert(addr, val);
        }
    }
    memory.values().sum()
}

fn run_and_sum_values2(
    memory: &mut HashMap<u128, u128>,
    data: &[(String, Vec<(u128, u128)>)],
) -> u128 {
    for (mask, instructions) in data {
        for &(addr, val) in instructions {
            let addr_mask = generate_addr_mask(addr, &mask);
            for actual_addr in generate_address_values(&addr_mask, addr_mask, 0) {
                memory.insert(actual_addr, val);
            }
        }
    }
    memory.values().sum()
}

fn generate_addr_mask(address: u128, mask: &str) -> [char; 36] {
    let mut res = ['0'; 36];
    for (addr_ch, ch) in format!("{:b}", address)
        .chars()
        .rev()
        .zip(res.iter_mut().rev())
    {
        *ch = addr_ch;
    }
    for (mask_ch, ch) in mask.chars().zip(res.iter_mut()) {
        match mask_ch {
            '1' => *ch = '1',
            'X' => *ch = 'X',
            _ => {}
        }
    }
    res
}

fn generate_address_values(mask: &[char; 36], candidate: [char; 36], i: usize) -> Vec<u128> {
    if i == mask.len() {
        return vec![candidate.iter().collect::<String>().parse().unwrap()];
    }
    if mask[i] != 'X' {
        return generate_address_values(mask, candidate, i + 1);
    }

    let mut results = Vec::new();
    let mut candidate_clone = candidate;
    candidate_clone[i] = '0';
    results.push(generate_address_values(mask, candidate_clone, i + 1));
    candidate_clone[i] = '1';
    results.push(generate_address_values(mask, candidate_clone, i + 1));
    results.concat()
}

fn read(filename: &str) -> Vec<(String, Vec<(u128, u128)>)> {
    read_to_string(filename)
        .expect("Failed to read file")
        .split_terminator("mask = ")
        .skip(1)
        .map(|s| {
            let lines = s.lines().collect::<Vec<&str>>();
            let instructions = lines
                .iter()
                .skip(1)
                .filter_map(|l| {
                    let parts: Vec<&str> = l.split_terminator(" = ").collect();
                    let mut address = parts[0].strip_prefix("mem[")?;
                    address = address.strip_suffix(']')?;
                    Some((address.parse().unwrap(), parts[1].parse().unwrap()))
                })
                .collect();
            (lines[0].into(), instructions)
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let data = read("test-input.txt");
        let mut memory = HashMap::new();
        assert_eq!(run_and_sum_values(&mut memory, &data), 165);
    }

    #[test]
    fn test2() {
        let data = read("test-input2.txt");
        let mut memory = HashMap::new();
        assert_eq!(run_and_sum_values2(&mut memory, &data), 208);
    }
}
