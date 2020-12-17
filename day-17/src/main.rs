use std::fs::read_to_string;
use std::{collections::BTreeMap, ops::RangeInclusive};

fn main() {
    let data = read("input.txt");
    let mut cube = BTreeMap::new();
    cube.insert(0, data);
    println!("part1 solution: {}", count_active_after_n(&cube, 6));

    let mut tesseract = BTreeMap::new();
    tesseract.insert(0, cube);
    println!("part2 solution: {}", count_active_4d_after_n(&tesseract, 6));
}

type Map = BTreeMap<i32, BTreeMap<i32, char>>;
type Cube = BTreeMap<i32, Map>;
type Tesseract = BTreeMap<i32, Cube>;

fn count_active_after_n(data: &Cube, n: usize) -> usize {
    let mut state = data.clone();
    (0..n).for_each(|_| {
        let [x_range, y_range, z_range] = ranges(&state);
        let old = state.clone();

        z_range.for_each(|z| {
            x_range.clone().for_each(|x| {
                y_range.clone().for_each(|y| {
                    let count = count_active_neighbours(&old, (x, y, z), true);
                    let field = get_value(&mut state, (x, y, z));
                    update_value(field, count);
                })
            })
        })
    });
    count_active(&state)
}

fn count_active_4d_after_n(data: &BTreeMap<i32, Cube>, n: usize) -> usize {
    let mut state = data.clone();
    (0..n).for_each(|_| {
        let [x_range, y_range, z_range, r_range] = ranges_4d(&state);
        let old = state.clone();

        r_range.for_each(|r| {
            z_range.clone().for_each(|z| {
                x_range.clone().for_each(|x| {
                    y_range.clone().for_each(|y| {
                        let count = count_active_neighbours_4d(&old, (x, y, z, r));
                        let field = get_value_4d(&mut state, (x, y, z, r));
                        update_value(field, count)
                    })
                })
            })
        });
    });
    count_active_4d(&state)
}

fn count_active_neighbours(state: &Cube, pos: (i32, i32, i32), skip: bool) -> usize {
    let (curr_x, curr_y, curr_z) = pos;
    let mut count = 0;
    for z in curr_z - 1..=curr_z + 1 {
        for x in curr_x - 1..=curr_x + 1 {
            for y in curr_y - 1..=curr_y + 1 {
                if x == curr_x && y == curr_y && z == curr_z && skip {
                    continue;
                }
                count += state.get(&z).map_or(0, |m| {
                    m.get(&x)
                        .map_or(0, |r| (r.get(&y).unwrap_or(&'.') == &'#') as usize)
                });
            }
        }
    }
    count
}

fn count_active_neighbours_4d(state: &Tesseract, pos: (i32, i32, i32, i32)) -> usize {
    let (curr_x, curr_y, curr_z, curr_r) = pos;
    (curr_r - 1..=curr_r + 1)
        .map(|r| {
            state.get(&r).map_or(0, |c| {
                count_active_neighbours(c, (curr_x, curr_y, curr_z), r == curr_r)
            })
        })
        .sum()
}

fn count_active(state: &Cube) -> usize {
    state
        .values()
        .map(|m| {
            m.values()
                .map(|r| r.values().filter(|&&v| v == '#').count())
                .sum::<usize>()
        })
        .sum::<usize>()
}

fn count_active_4d(state: &Tesseract) -> usize {
    state.values().map(|c| count_active(c)).sum()
}

fn get_value(state: &mut Cube, pos: (i32, i32, i32)) -> &mut char {
    let (x, y, z) = pos;
    state
        .entry(z)
        .or_insert(BTreeMap::new())
        .entry(x)
        .or_insert(BTreeMap::new())
        .entry(y)
        .or_insert('.')
}

fn get_value_4d(state: &mut Tesseract, pos: (i32, i32, i32, i32)) -> &mut char {
    let (x, y, z, r) = pos;
    get_value(state.entry(r).or_insert(BTreeMap::new()), (x, y, z))
}

fn update_value(value: &mut char, neighbour_count: usize) {
    if *value == '#' && !(neighbour_count == 2 || neighbour_count == 3) {
        *value = '.';
    } else if *value == '.' && neighbour_count == 3 {
        *value = '#'
    }
}

fn ranges(state: &Cube) -> [RangeInclusive<i32>; 3] {
    let map = state.get(&0).unwrap();
    let row = map.get(&0).unwrap();
    [
        map.keys().min().unwrap() - 1..=map.keys().max().unwrap() + 1,
        row.keys().min().unwrap() - 1..=row.keys().max().unwrap() + 1,
        state.keys().min().unwrap() - 1..=state.keys().max().unwrap() + 1,
    ]
}

fn ranges_4d(state: &Tesseract) -> [RangeInclusive<i32>; 4] {
    let (&r_min, &r_max) = (state.keys().min().unwrap(), state.keys().max().unwrap());
    let [x_range, y_range, z_range] = ranges(state.get(&0).unwrap());
    [x_range, y_range, z_range, r_min - 1..=r_max + 1]
}

fn read(filename: &str) -> Map {
    let content = read_to_string(filename).expect("Failed to read file");
    let lines = content.lines().collect::<Vec<&str>>();
    let x_offset = lines.len() as i32 / 2;
    lines
        .iter()
        .map(|s| {
            let offset = s.len() as i32 / 2;
            s.char_indices()
                .map(|(i, ch)| (i as i32 - offset, ch))
                .collect::<BTreeMap<i32, char>>()
        })
        .enumerate()
        .map(|(i, map)| (i as i32 - x_offset, map))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let data = read("test-input.txt");
        let mut cube = BTreeMap::new();
        cube.insert(0, data);
        assert_eq!(count_active_after_n(&cube, 6), 112);
        let mut tesseract = BTreeMap::new();
        tesseract.insert(0, cube);
        assert_eq!(count_active_4d_after_n(&tesseract, 6), 848);
    }
}
