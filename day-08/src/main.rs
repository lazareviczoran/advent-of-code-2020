use std::collections::HashSet;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    let mut console = GameConsole::new(read("input.txt"));

    console.run();
    println!("part1 solution {}", console.score);
    console.reset();

    console.run_til_termination();
    println!("part2 solution {}", console.score);
}

struct GameConsole {
    commands: Vec<Op>,
    score: i32,
    cmd_idx: i32,
    ran_cmds: HashSet<i32>,
}

impl GameConsole {
    pub fn new(commands: Vec<Op>) -> Self {
        Self {
            commands,
            score: 0,
            cmd_idx: 0,
            ran_cmds: HashSet::new(),
        }
    }

    pub fn run(&mut self) {
        while !self.ran_cmds.contains(&self.cmd_idx)
            && (self.cmd_idx as usize) < self.commands.len()
        {
            self.ran_cmds.insert(self.cmd_idx);
            match self.commands[self.cmd_idx as usize] {
                Op::Acc(val) => {
                    self.score += val;
                    self.cmd_idx += 1;
                }
                Op::Jmp(val) => self.cmd_idx += val,
                Op::Nop(_) => self.cmd_idx += 1,
            }
        }
    }

    pub fn run_til_termination(&mut self) {
        for i in 0..self.commands.len() {
            if self.switch(i) {
                self.run();
                if self.cmd_idx as usize == self.commands.len() {
                    return;
                }
                self.switch(i);
                self.reset();
            }
        }
    }

    pub fn reset(&mut self) {
        self.cmd_idx = 0;
        self.score = 0;
        self.ran_cmds.clear();
    }

    fn switch(&mut self, i: usize) -> bool {
        let mut changed = false;
        self.commands[i] = match self.commands[i] {
            Op::Jmp(val) => {
                changed = true;
                Op::Nop(val)
            }
            Op::Nop(val) => {
                changed = true;
                Op::Jmp(val)
            }
            Op::Acc(val) => Op::Acc(val),
        };
        changed
    }
}

enum Op {
    Acc(i32),
    Jmp(i32),
    Nop(i32),
}

fn read(filename: &str) -> Vec<Op> {
    let mut file = File::open(filename).expect("File not found");
    let mut content = String::new();
    file.read_to_string(&mut content)
        .expect("Failed to read file");

    content
        .lines()
        .map(|s| {
            let values = s.split(' ').collect::<Vec<_>>();
            let num = values[1].parse().unwrap();
            match values[0] {
                "acc" => Op::Acc(num),
                "jmp" => Op::Jmp(num),
                "nop" => Op::Nop(num),
                _ => panic!("unexpected value {}", values[0]),
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_test() {
        let mut console = GameConsole::new(read("test-input.txt"));
        console.run();
        assert_eq!(console.score, 5);
    }

    #[test]
    fn part2_test() {
        let mut console = GameConsole::new(read("test-input.txt"));
        console.run_til_termination();
        assert_eq!(console.score, 8);
    }
}
