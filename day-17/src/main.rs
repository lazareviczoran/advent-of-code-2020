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

fn count<const N: usize>(mut active: HashSet<Point<N>>) -> usize {
    let diffs = generate_diffs::<N>();
    (0..6).for_each(|_| {
        let counts = active.iter().fold(HashMap::new(), |mut acc, pos| {
            diffs.iter().for_each(|diff| {
                *acc.entry(add(pos, diff)).or_insert(0) += 1;
            });
            acc
        });
        active = counts
            .iter()
            .filter(|&(pos, &count)| count == 3 || active.contains(pos) && count == 2)
            .map(|(pos, _)| *pos)
            .collect();
    });

    active.len()
}

fn generate_diffs<const N: usize>() -> Vec<Point<N>> {
    (0..N)
        .map(|_i| -1..=1)
        .multi_cartesian_product()
        .filter_map(convert)
        .collect()
}

fn convert<const N: usize>(v: Vec<i32>) -> Option<Point<N>> {
    let boxed = v.into_boxed_slice();
    let mut a = [0; N];
    a.copy_from_slice(&boxed[0..N]);
    if a == [0; N] {
        return None;
    }
    Some(a)
}

fn read<const N: usize>(filename: &str) -> HashSet<Point<N>> {
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

type Point<const N: usize> = [i32; N];
fn add<const N: usize>(pos: &Point<N>, diff: &Point<N>) -> Point<N> {
    let mut res = *pos;
    res.iter_mut().zip(diff.iter()).for_each(|(a, &b)| *a += b);
    res
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
