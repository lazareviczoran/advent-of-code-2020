use num::bigint::BigInt;
use std::fs::read_to_string;

fn main() {
    let (id, buses) = read("input.txt");

    println!("part1 solution {}", find_first_to_depart(id, &buses));
    println!("part2 solution {}", find_first_depart_to_all(&buses));
}

fn find_first_to_depart(id: usize, buses: &[usize]) -> usize {
    let best_bus = buses
        .iter()
        .filter(|&&bus_id| bus_id > 0)
        .map(|bus_id| {
            let remaining = id.rem_euclid(*bus_id);
            (*bus_id, *bus_id - remaining)
        })
        .min_by_key(|a| a.1)
        .unwrap();
    best_bus.0 * best_bus.1
}

// using the Chinese remainder theorem
fn find_first_depart_to_all(buses: &[usize]) -> BigInt {
    let values = buses
        .iter()
        .enumerate()
        .filter(|(_i, &id)| id > 0)
        .map(|v| (BigInt::from(v.0), BigInt::from(*v.1)))
        .collect::<Vec<(BigInt, BigInt)>>();

    let mod_multi = values.iter().map(|(_, val)| val).product::<BigInt>();

    let m_values = values
        .iter()
        .map(|(_, val)| &mod_multi / val)
        .collect::<Vec<BigInt>>();

    let sum: BigInt = values
        .iter()
        .zip(m_values.iter())
        .map(|((delay, time), m_val)| {
            let y = (0..)
                .find(|&curr| (m_val * BigInt::from(curr)) % time == BigInt::from(1))
                .unwrap();
            let rem = (time - delay) % time;
            rem * m_val * y
        })
        .sum();
    sum % mod_multi
}

fn read(filename: &str) -> (usize, Vec<usize>) {
    let content = read_to_string(filename).expect("Failed to read file");
    let lines = content.split('\n').collect::<Vec<&str>>();
    let ids = lines[1]
        .split_terminator(',')
        .map(|id| id.parse().unwrap_or(0))
        .collect::<Vec<usize>>();
    (lines[0].parse().unwrap(), ids)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let (id, buses) = read("test-input.txt");
        assert_eq!(find_first_to_depart(id, &buses), 295);
    }

    #[test]
    fn test2() {
        let (_, buses) = read("test-input.txt");
        assert_eq!(find_first_depart_to_all(&buses), BigInt::from(1068781));
    }

    #[test]
    fn test3() {
        let (_, buses) = read("test-input2.txt");
        assert_eq!(find_first_depart_to_all(&buses), BigInt::from(3417));
    }

    #[test]
    fn test4() {
        let (_, buses) = read("test-input3.txt");
        assert_eq!(find_first_depart_to_all(&buses), BigInt::from(754018));
    }

    #[test]
    fn test5() {
        let (_, buses) = read("test-input4.txt");
        assert_eq!(find_first_depart_to_all(&buses), BigInt::from(779210));
    }

    #[test]
    fn test6() {
        let (_, buses) = read("test-input5.txt");
        assert_eq!(find_first_depart_to_all(&buses), BigInt::from(1261476));
    }

    #[test]
    fn test7() {
        let (_, buses) = read("test-input6.txt");
        assert_eq!(find_first_depart_to_all(&buses), BigInt::from(1202161486));
    }
}
