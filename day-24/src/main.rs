use std::collections::HashMap;
use std::fs::read_to_string;

const DIFFS: [(i32, i32, i32); 6] = [
    (-1, 1, 0),
    (0, 1, -1),
    (-1, 0, 1),
    (1, -1, 0),
    (1, 0, -1),
    (0, -1, 1),
];

fn main() {
    let data = read("input.txt");
    let map = flip_tiles(&data);

    println!(
        "part1 solution: {:?}",
        map.values().filter(|&&v| v == Color::Black).count()
    );
    println!("part2 solution: {}", run(&map));
}

fn flip_tiles(instructions: &[Vec<Direction>]) -> HashMap<Point, Color> {
    let mut map = HashMap::new();
    for instruction_set in instructions {
        let mut curr = Point::new(0, 0, 0);
        for ins in instruction_set {
            let diff = match ins {
                Direction::East => DIFFS[0],
                Direction::NorthEast => DIFFS[1],
                Direction::SouthEast => DIFFS[2],
                Direction::West => DIFFS[3],
                Direction::NorthWest => DIFFS[4],
                Direction::SouthWest => DIFFS[5],
            };
            curr = curr.add(diff);
        }
        let tile = map.entry(curr).or_insert(Color::White);
        *tile = match tile {
            Color::White => Color::Black,
            Color::Black => Color::White,
        }
    }
    map
}

fn run(tiles: &HashMap<Point, Color>) -> usize {
    let mut current = tiles.clone();
    for _i in 0..100 {
        let mut counts = HashMap::new();
        current.iter().for_each(|(pos, color)| {
            if color == &Color::Black {
                for p in get_neighbour_positions(*pos) {
                    *counts.entry(p).or_insert(0) += 1;
                }
            }
        });
        current = counts
            .iter()
            .filter(|&(tile_pos, &count)| {
                let tile_color = current.get(tile_pos).unwrap_or(&Color::White);
                tile_color == &Color::Black && count == 1 || count == 2
            })
            .map(|tile| (*tile.0, Color::Black))
            .collect();
    }
    current.len()
}

fn get_neighbour_positions(pos: Point) -> Vec<Point> {
    DIFFS.iter().map(|&diff| pos.add(diff)).collect()
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct Point {
    x: i32,
    y: i32,
    z: i32,
}
impl Point {
    pub fn new(x: i32, y: i32, z: i32) -> Self {
        Self { x, y, z }
    }

    pub fn add(&self, diff: (i32, i32, i32)) -> Self {
        Self::new(self.x + diff.0, self.y + diff.1, self.z + diff.2)
    }
}

#[derive(Debug, Clone, Copy)]
enum Direction {
    East,
    NorthEast,
    SouthEast,
    West,
    NorthWest,
    SouthWest,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Color {
    White,
    Black,
}

fn read(filename: &str) -> Vec<Vec<Direction>> {
    let content = read_to_string(filename).expect("Failed to read file");
    content
        .lines()
        .map(|l| {
            let mut chars = l.chars();
            let mut dirs = Vec::new();
            while let Some(ch) = chars.next() {
                let dir = match ch {
                    'e' => Direction::East,
                    'w' => Direction::West,
                    'n' => match chars.next() {
                        Some('e') => Direction::NorthEast,
                        Some('w') => Direction::NorthWest,
                        _ => unreachable!(),
                    },
                    's' => match chars.next() {
                        Some('e') => Direction::SouthEast,
                        Some('w') => Direction::SouthWest,
                        _ => unreachable!(),
                    },
                    _ => panic!("unexpected {}", ch),
                };
                dirs.push(dir);
            }
            dirs
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let data = read("test-input.txt");

        let map = flip_tiles(&data);

        assert_eq!(map.values().filter(|&&v| v == Color::Black).count(), 10);
        assert_eq!(run(&map), 2208);
    }
}
