#![feature(min_const_generics)] // Not a stable feature yet => requires nightly
use itertools::Itertools;
use std::collections::{HashMap, HashSet};
use std::fs::read_to_string;

fn main() {
    let data = read::<3>("input.txt");
    println!("part1 solution: {}", count(data));

    let data = read::<4>("input.txt");
    println!("part2 solution: {}", count(data));
}

fn count<const N: usize>(mut active: HashSet<[i32; N]>) -> usize {
    let diffs = generate_diffs::<N>();
    (0..6).for_each(|_| {
        let mut counts = HashMap::new();
        active.iter().for_each(|pos| {
            diffs.iter().for_each(|diff| {
                let mut curr_pos = *pos;
                for (val, diff_val) in curr_pos.iter_mut().zip(diff.iter()) {
                    *val += diff_val;
                }
                *counts.entry(curr_pos).or_insert(0) += 1;
            })
        });
        active = counts
            .iter()
            .filter(|&(pos, &count)| count == 3 || active.contains(pos) && count == 2)
            .map(|(pos, _)| *pos)
            .collect();
    });

    active.len()
}

fn generate_diffs<const N: usize>() -> Vec<[i32; N]> {
    (0..N)
        .map(|_i| -1..=1)
        .multi_cartesian_product()
        .filter_map(convert)
        .collect::<Vec<_>>()
}

fn convert<const N: usize>(v: Vec<i32>) -> Option<[i32; N]> {
    let boxed = v.into_boxed_slice();
    let mut a: [i32; N] = [0; N];
    a.copy_from_slice(&boxed[0..N]);
    if a == [0; N] {
        return None;
    }
    Some(a)
}

fn read<const N: usize>(filename: &str) -> HashSet<[i32; N]> {
    read_to_string(filename)
        .expect("Failed to read file")
        .lines()
        .enumerate()
        .fold(HashSet::new(), |mut acc, (x, l)| {
            acc.extend(
                l.char_indices()
                    .filter(|&(_y, ch)| ch == '#')
                    .map(|(y, _)| {
                        let mut pos = [0; N];
                        pos[0] = x as i32;
                        pos[1] = y as i32;
                        pos
                    }),
            );
            acc
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let data = read::<3>("test-input.txt");
        assert_eq!(count(data), 112);
    }

    #[test]
    fn test2() {
        let data = read::<4>("test-input.txt");
        assert_eq!(count(data), 848);
    }
}
