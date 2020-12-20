use std::{
    cmp::Ordering,
    collections::{BTreeMap, HashSet, VecDeque},
    fs::read_to_string,
};

fn main() {
    let data = read("input.txt");
    let image = find_big_picture(&data);
    println!("part1 solution: {}", calculate_edges_product(&image));

    let main_tile = merge_into_single_tile(image);
    println!("part2 solution: {}", count_monsters(main_tile));
}

fn count_monsters(tile: Tile) -> usize {
    let part1 = "                  # ";
    let part2 = "#    ##    ##    ###";
    let part3 = " #  #  #  #  #  #   ";
    let monster_pattern = format!("{}\n{}\n{}", part1, part2, part3)
        .lines()
        .map(|l| l.chars().collect())
        .collect::<Vec<Vec<char>>>();

    let count_hashes = tile
        .content
        .iter()
        .map(|r| r.iter().filter(|&&f| f == '#').count())
        .sum::<usize>();
    let monster_hash_count = monster_pattern
        .iter()
        .map(|r| r.iter().filter(|&&f| f == '#').count())
        .sum::<usize>();

    let monsters_count = generate_states(&tile)
        .iter()
        .map(|t| t.count_matched_monsters(&monster_pattern))
        .sum::<usize>();
    count_hashes - monsters_count * monster_hash_count
}

fn calculate_edges_product(image: &BTreeMap<Point, Tile>) -> usize {
    let (min_point, max_point) = match (image.keys().next(), image.keys().last()) {
        (Some(&min), Some(&max)) => (min, max),
        x => panic!("{:?}", x),
    };
    image.get(&min_point).unwrap().id
        * image.get(&Point::new(min_point.x, max_point.y)).unwrap().id
        * image.get(&Point::new(max_point.x, min_point.y)).unwrap().id
        * image.get(&max_point).unwrap().id
}

fn find_big_picture(tiles: &BTreeMap<usize, Tile>) -> BTreeMap<Point, Tile> {
    let mut image = BTreeMap::new();
    let mut used = HashSet::new();
    let initial_tile = tiles.iter().next().unwrap().1;
    used.insert(initial_tile.id);
    image.insert(Point::new(0, 0), initial_tile.clone());
    solve(tiles, &mut used, &mut image);

    image
}

fn generate_states(tile: &Tile) -> Vec<Tile> {
    let mut q = VecDeque::new();
    q.push_back(tile.clone());
    let mut visited = HashSet::new();
    while !q.is_empty() {
        let curr_state = q.pop_front().unwrap();
        if visited.contains(&curr_state) {
            continue;
        }
        visited.insert(curr_state.clone());

        let mut rotated = curr_state.clone();
        rotated.rotate();
        q.push_back(rotated);
        let mut flipped = curr_state.clone();
        flipped.flip();
        q.push_back(flipped);
    }
    visited.iter().cloned().collect()
}

fn solve(
    tiles: &BTreeMap<usize, Tile>,
    used: &mut HashSet<usize>,
    image: &mut BTreeMap<Point, Tile>,
) -> bool {
    if used.len() == tiles.len() {
        return true;
    }

    let curr_used = used.clone();
    for id in tiles.keys().filter(|k| !curr_used.contains(k)) {
        for (cand_point, cand_tile) in get_candidates(tiles, id, image) {
            let curr_id = cand_tile.id;
            used.insert(curr_id);
            image.insert(cand_point, cand_tile);
            if solve(tiles, used, image) {
                return true;
            }
            image.remove(&cand_point);
            used.remove(&curr_id);
        }
    }

    false
}

fn merge_into_single_tile(mut image: BTreeMap<Point, Tile>) -> Tile {
    remove_borders(&mut image);
    let mut main_tile = Tile::new(0, VecDeque::new());
    let dim = (image.len() as f32).sqrt() as usize;
    for (idx, (_, tile)) in image.iter().enumerate() {
        for (i, row) in tile.content.iter().enumerate() {
            let pos = idx / dim * row.len() + i;
            if main_tile.content.len() < pos + 1 {
                main_tile.content.push_back(VecDeque::new());
            }
            for &ch in row.iter().rev() {
                main_tile.content[pos].push_back(ch);
            }
        }
    }
    main_tile
}

fn remove_borders(image: &mut BTreeMap<Point, Tile>) {
    image.iter_mut().for_each(|(_p, tile)| {
        tile.content.pop_front();
        tile.content.pop_back();
        tile.content.iter_mut().for_each(|row| {
            row.pop_front();
            row.pop_back();
        });
    });
}

fn get_candidates(
    tiles: &BTreeMap<usize, Tile>,
    curr_tile_id: &usize,
    image: &BTreeMap<Point, Tile>,
) -> Vec<(Point, Tile)> {
    let mut tile = tiles.get(&curr_tile_id).unwrap().clone();
    image
        .iter()
        .filter_map(|(p, _id)| {
            let diff = [(1, 0), (-1, 0), (0, 1), (0, -1)]
                .iter()
                .find(|&&(diff_x, diff_y)| {
                    let candidate = p.add(diff_x, diff_y);
                    image.get(&candidate).is_none()
                        && matches_all_neighbours(&mut tile, image, candidate)
                })?;
            Some((p.add(diff.0, diff.1), tile.clone()))
        })
        .collect()
}

fn matches_all_neighbours(tile: &mut Tile, image: &BTreeMap<Point, Tile>, pos: Point) -> bool {
    [
        (Point::new(1, 0), Side::Right),
        (Point::new(-1, 0), Side::Left),
        (Point::new(0, 1), Side::Top),
        (Point::new(0, -1), Side::Bottom),
    ]
    .iter()
    .all(|&(diff, side)| {
        image
            .get(&pos.add(diff.x, diff.y))
            .map_or(true, |cmp_tile| tile.can_match(cmp_tile, side))
    })
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Point {
    x: i32,
    y: i32,
}
impl Point {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
    pub fn add(&self, diff_x: i32, diff_y: i32) -> Self {
        Self::new(self.x + diff_x, self.y + diff_y)
    }
}
impl Ord for Point {
    fn cmp(&self, other: &Self) -> Ordering {
        self.y.cmp(&other.y).reverse().then(self.x.cmp(&other.x))
    }
}
impl PartialOrd for Point {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(&other))
    }
}

#[derive(Debug, Clone, Copy)]
enum Side {
    Top,
    Bottom,
    Left,
    Right,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Tile {
    id: usize,
    content: VecDeque<VecDeque<char>>,
}
impl Tile {
    fn new(id: usize, content: VecDeque<VecDeque<char>>) -> Self {
        Self { id, content }
    }
    fn rotate(&mut self) {
        let n = self.content.len();
        for i in 0..n / 2 {
            for j in i..n - 1 - i {
                swap(&mut self.content, (i, j), (j, n - 1 - i));
                swap(&mut self.content, (i, j), (n - 1 - i, n - 1 - j));
                swap(&mut self.content, (i, j), (n - 1 - j, i));
            }
        }
    }

    fn flip(&mut self) {
        let n = self.content[0].len();
        self.content.iter_mut().for_each(|row| {
            for i in 0..n / 2 {
                row.swap(i, n - 1 - i);
            }
        })
    }

    pub fn can_match(&mut self, other: &Tile, side: Side) -> bool {
        (0..4).any(|_i| {
            if self.check_match_single(other, side) {
                return true;
            }
            self.flip();
            if self.check_match_single(other, side) {
                return true;
            }
            self.flip();
            self.rotate();
            false
        })
    }

    pub fn check_match_single(&mut self, other: &Tile, side: Side) -> bool {
        match side {
            Side::Top => self.content.front() == other.content.back(),
            Side::Bottom => self.content.back() == other.content.front(),
            Side::Left => self
                .content
                .iter()
                .zip(other.content.iter())
                .all(|(p, r)| p.back() == r.front()),
            Side::Right => self
                .content
                .iter()
                .zip(other.content.iter())
                .all(|(p, r)| p.front() == r.back()),
        }
    }

    pub fn count_matched_monsters(&self, monster_pattern: &[Vec<char>]) -> usize {
        let tile_dim = (self.content.len(), self.content[0].len());
        let pattern_dim = (monster_pattern.len(), monster_pattern[0].len());

        (0..=tile_dim.0 - pattern_dim.0)
            .map(|x| {
                (0..=tile_dim.1 - pattern_dim.1)
                    .filter(|y| {
                        monster_pattern.iter().enumerate().all(|(offset_x, row)| {
                            row.iter().enumerate().all(|(offset_y, &f)| {
                                f != '#' || self.content[x + offset_x][y + offset_y] == '#'
                            })
                        })
                    })
                    .count()
            })
            .sum()
    }
}

fn swap(content: &mut VecDeque<VecDeque<char>>, pos1: (usize, usize), pos2: (usize, usize)) {
    let temp = content[pos1.0][pos1.1];
    content[pos1.0][pos1.1] = content[pos2.0][pos2.1];
    content[pos2.0][pos2.1] = temp;
}

fn read(filename: &str) -> BTreeMap<usize, Tile> {
    read_to_string(filename)
        .expect("Failed to read file")
        .split_terminator("\n\n")
        .filter_map(|s| {
            let mut lines_iter = s.lines();
            let id = lines_iter
                .next()?
                .strip_prefix("Tile ")?
                .strip_suffix(":")?
                .parse()
                .unwrap();
            let content = lines_iter.map(|l| l.chars().collect()).collect();
            Some((id, Tile { id, content }))
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let data = read("test-input.txt");
        let image = find_big_picture(&data);
        assert_eq!(calculate_edges_product(&image), 20899048083289);

        let m = merge_into_single_tile(image);
        assert_eq!(
            m.content[0],
            ".#.#..#.##...#.##..#####".chars().collect::<VecDeque<_>>()
        );
        assert_eq!(
            m.content[12],
            "##..##.#...#...#.#.#.#..".chars().collect::<VecDeque<_>>()
        );
        assert_eq!(
            m.content[19],
            "#..####...#.#.#.###.###.".chars().collect::<VecDeque<_>>()
        );
        assert_eq!(
            m.content[23],
            "...###...##...#...#..###".chars().collect::<VecDeque<_>>()
        );
        assert_eq!(count_monsters(m), 273);
    }

    #[test]
    fn test_rotate() {
        let mut tile = Tile {
            id: 123,
            content: VecDeque::from(vec![
                VecDeque::from(vec!['.', '#', '#', '.', '.']),
                VecDeque::from(vec!['.', '#', '#', '.', '.']),
                VecDeque::from(vec!['.', '#', '#', '.', '.']),
                VecDeque::from(vec!['.', '#', '#', '.', '.']),
                VecDeque::from(vec!['.', '#', '#', '.', '.']),
            ]),
        };

        tile.rotate();
        assert_eq!(
            tile.content,
            VecDeque::from(vec![
                VecDeque::from(vec!['.', '.', '.', '.', '.']),
                VecDeque::from(vec!['#', '#', '#', '#', '#']),
                VecDeque::from(vec!['#', '#', '#', '#', '#']),
                VecDeque::from(vec!['.', '.', '.', '.', '.']),
                VecDeque::from(vec!['.', '.', '.', '.', '.']),
            ])
        );
    }
}
