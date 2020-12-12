use std::fs::read_to_string;
use std::ops::AddAssign;

fn main() {
    let data = read("input.txt");
    let mut ship = Ship::new(Point::new(1, 0));
    ship.move_ship(&data, true);
    println!(
        "part1 solution: {}",
        ship.position.x.abs() + ship.position.y.abs()
    );
    ship = Ship::new(Point::new(10, -1));
    ship.move_ship(&data, false);
    println!(
        "part2 solution: {}",
        ship.position.x.abs() + ship.position.y.abs()
    );
}

struct Ship {
    position: Point,
    waypoint: Point,
}

impl Ship {
    pub fn new(waypoint: Point) -> Self {
        Self {
            position: Point::new(0, 0),
            waypoint,
        }
    }

    pub fn move_ship(&mut self, instructions: &[Dir], move_ship_pos: bool) {
        for instruction in instructions.iter() {
            match &instruction {
                Dir::East(val) => self.move_in_dir(Point::new(*val, 0), move_ship_pos),
                Dir::West(val) => self.move_in_dir(Point::new(-*val, 0), move_ship_pos),
                Dir::North(val) => self.move_in_dir(Point::new(0, -*val), move_ship_pos),
                Dir::South(val) => self.move_in_dir(Point::new(0, *val), move_ship_pos),
                Dir::Forward(val) => {
                    self.position.x += val * self.waypoint.x;
                    self.position.y += val * self.waypoint.y;
                }
                Dir::Right(val) => self.rotate(*val),
                Dir::Left(val) => self.rotate(-*val),
            }
        }
    }

    fn move_in_dir(&mut self, dir: Point, move_ship_pos: bool) {
        if move_ship_pos {
            self.position += dir;
        } else {
            self.waypoint += dir;
        }
    }

    fn rotate(&mut self, val: i32) {
        let (sin, cos) = (val as f32).to_radians().sin_cos();
        let Point { x, y } = self.waypoint;
        self.waypoint.x = (x as f32 * cos - y as f32 * sin).round() as i32;
        self.waypoint.y = (x as f32 * sin + y as f32 * cos).round() as i32;
    }
}

#[derive(Debug, Copy, Clone)]
struct Point {
    x: i32,
    y: i32,
}
impl Point {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}
impl AddAssign for Point {
    fn add_assign(&mut self, other: Self) {
        *self = Point::new(self.x + other.x, self.y + other.y);
    }
}

#[derive(Clone)]
enum Dir {
    North(i32),
    South(i32),
    East(i32),
    West(i32),
    Forward(i32),
    Right(i32),
    Left(i32),
}

fn read(filename: &str) -> Vec<Dir> {
    read_to_string(filename)
        .expect("Failed to read file")
        .lines()
        .map(|s| {
            let (cmd, value_str) = s.split_at(1);
            let value = value_str.parse().unwrap();
            match cmd {
                "E" => Dir::East(value),
                "W" => Dir::West(value),
                "N" => Dir::North(value),
                "S" => Dir::South(value),
                "F" => Dir::Forward(value),
                "R" => Dir::Right(value),
                "L" => Dir::Left(value),
                _ => unreachable!(),
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
        let mut ship = Ship::new(Point::new(1, 0));
        ship.move_ship(&data, true);
        assert_eq!(ship.position.x.abs() + ship.position.y.abs(), 25);
    }

    #[test]
    fn test2() {
        let data = read("test-input.txt");
        let mut ship = Ship::new(Point::new(10, -1));
        ship.move_ship(&data, false);
        assert_eq!(ship.position.x.abs() + ship.position.y.abs(), 286);
    }
}
