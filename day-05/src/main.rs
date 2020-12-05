use std::fs::File;
use std::io::prelude::*;

fn main() {
    let mut seat_ids = read("input.txt");

    println!(
        "part1 solution: {}",
        seat_ids.iter().max().expect("There are no seats")
    );

    seat_ids.sort_unstable();
    let my_seat_id = seat_ids
        .windows(2)
        .find(|&seats| seats[1] - seats[0] > 1)
        .map(|seats| seats[0] + 1)
        .expect("Couldn't find my seat");

    println!("part2 solution: {}", my_seat_id);
}

fn find_seat_value(pass: &str) -> usize {
    let mut row_l = 0;
    let mut row_h = 127;
    let mut seat_l = 0;
    let mut seat_h = 7;
    for ch in pass.chars() {
        match ch {
            'F' => row_h = (row_l + row_h) / 2,
            'B' => row_l = (row_l + row_h + 1) / 2,
            'L' => seat_h = (seat_l + seat_h) / 2,
            'R' => seat_l = (seat_l + seat_h + 1) / 2,
            _ => panic!("unexpected input"),
        }
    }
    row_l * 8 + seat_l
}

fn read(filename: &str) -> Vec<usize> {
    let mut file = File::open(filename).expect("File not found");
    let mut content = String::new();
    file.read_to_string(&mut content)
        .expect("Failed to load file content");

    content
        .split_terminator('\n')
        .map(find_seat_value)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_test() {
        let seat_ids = read("test-input.txt");
        assert_eq!(*seat_ids.iter().max().unwrap(), 820);
    }
}
