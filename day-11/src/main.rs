use std::fs::read_to_string;
use std::thread::sleep;
use std::time::Duration;

const ESC: &str = "\x1B[";
const RESET: &str = "\x1B[0m";

fn main() {
    let mut data = read("input.txt");
    println!(
        "part1 solution: {}",
        simulate_and_count(&mut data.clone(), false)
    );

    println!("part2 solution: {}", simulate_and_count(&mut data, true));
}

fn simulate_and_count(data: &mut Vec<Vec<char>>, look_further: bool) -> usize {
    simulate_changes(data, look_further, false);
    count_occupied(&data)
}

fn count_occupied(data: &[Vec<char>]) -> usize {
    data.iter()
        .map(|row| row.iter().filter(|&&f| f == '#').count())
        .sum()
}

fn simulate_changes(data: &mut Vec<Vec<char>>, look_further: bool, visualize: bool) {
    let min_occupied_required = if look_further { 5 } else { 4 };
    loop {
        let curr_state = data.clone();
        if visualize {
            sleep(Duration::from_millis(750));
            print_map(&curr_state);
        }
        let mut changed = 0;
        for (x, row) in data.iter_mut().enumerate() {
            for (y, field) in row.iter_mut().enumerate() {
                if *field == 'L'
                    && count_adjacent(&curr_state, x as i32, y as i32, look_further) == 0
                {
                    *field = '#';
                    changed += 1;
                } else if *field == '#'
                    && count_adjacent(&curr_state, x as i32, y as i32, look_further)
                        >= min_occupied_required
                {
                    *field = 'L';
                    changed += 1;
                }
            }
        }

        if changed == 0 {
            return;
        }
    }
}

fn count_adjacent(data: &[Vec<char>], x: i32, y: i32, look_further: bool) -> usize {
    let mut count = 0;
    for i in -1..=1 {
        for j in -1..=1 {
            let (mut step_x, mut step_y) = (i, j);
            if i == 0 && j == 0 {
                continue;
            }

            loop {
                let (next_x, next_y) = (x + step_x, y + step_y);
                if next_x < 0
                    || next_x as usize >= data.len()
                    || next_y < 0
                    || next_y as usize >= data[x as usize].len()
                {
                    break;
                }
                if data[(next_x as usize)][(next_y as usize)] == '#' {
                    count += 1;
                }
                if !look_further || data[(next_x as usize)][(next_y as usize)] != '.' {
                    break;
                }
                step_x += i;
                step_y += j;
            }
        }
    }
    count
}

fn print_map(data: &[Vec<char>]) {
    let black_background = 40;
    let mut s = format!("{}[2J", 27 as char);
    for row in data {
        for field in row {
            let color = match &field {
                '#' => 41,
                '.' => 40,
                'L' => 47,
                _ => panic!("unexpected char"),
            };
            s.push_str(&format!("{}{2}{1};1m", RESET, color, ESC));
            s.push(' ');
        }

        s.push_str(&format!("{}{2}{1};1m", RESET, black_background, ESC));
        s.push('\n');
    }
    s.push_str(&format!("{}{2}{1};1m", RESET, black_background, ESC));
    println!("{}", s);
}

fn read(filename: &str) -> Vec<Vec<char>> {
    read_to_string(filename)
        .expect("Failed to read file")
        .lines()
        .map(|s| s.chars().collect())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let mut data = read("test-input.txt");
        assert_eq!(simulate_and_count(&mut data, false), 37);
    }

    #[test]
    fn test2() {
        let mut data = read("test-input.txt");
        assert_eq!(simulate_and_count(&mut data, true), 26);
    }
}
