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

fn run_and_sum_values(memory: &mut HashMap<usize, usize>, data: &[Data]) -> usize {
    for item in data {
        let Data { mask, instructions } = item;
        for &(addr, new_val) in instructions {
            let mut val = new_val;
            for (i, ch) in mask.iter().rev().enumerate() {
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

fn run_and_sum_values2(memory: &mut HashMap<usize, usize>, data: &[Data]) -> usize {
    for item in data {
        for &(addr, val) in &item.instructions {
            let (addr_mask, initial_val) = generate_addr_mask(addr, &item.mask);
            let mut addresses = Vec::new();
            generate_address_values(&addr_mask, initial_val, 0, &mut addresses);
            for actual_addr in addresses {
                memory.insert(actual_addr, val);
            }
        }
    }
    memory.values().sum()
}

fn generate_addr_mask(address: usize, mask: &[char]) -> ([char; 36], usize) {
    let mut res = ['0'; 36];
    let mut val = 0;
    for ((i, mask_ch), ch) in mask.iter().rev().enumerate().zip(res.iter_mut().rev()) {
        match mask_ch {
            '1' => {
                *ch = '1';
                val |= 1 << i;
            }
            'X' => {
                *ch = 'X';
                val &= !(1 << i);
            }
            _ => val |= address & (1 << i),
        }
    }
    (res, val)
}

fn generate_address_values(mask: &[char; 36], candidate: usize, i: usize, acc: &mut Vec<usize>) {
    if i == mask.len() {
        acc.push(candidate);
        return;
    }

    generate_address_values(mask, candidate, i + 1, acc);
    if mask[i] == 'X' {
        generate_address_values(mask, candidate | 1 << (mask.len() - 1 - i), i + 1, acc);
    }
}

struct Data {
    mask: Vec<char>,
    instructions: Vec<(usize, usize)>,
}

fn read(filename: &str) -> Vec<Data> {
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
            Data {
                mask: lines[0].chars().collect(),
                instructions,
            }
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
