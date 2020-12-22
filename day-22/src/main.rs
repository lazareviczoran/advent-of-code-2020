use std::collections::{HashSet, VecDeque};
use std::fs::read_to_string;

fn main() {
    let mut game = read("input.txt");
    game.play_simple();
    println!("part1 solution: {:?}", game.calculate_score());

    game.reset();
    game.play_recursive();
    println!("part2 solution: {}", game.calculate_score());
}

#[derive(Debug, Clone)]
struct Game {
    decks: [VecDeque<usize>; 2],
    prev_states: HashSet<[VecDeque<usize>; 2]>,
    initial_decks: [VecDeque<usize>; 2],
    winner: usize,
}
impl Game {
    pub fn new(decks: [VecDeque<usize>; 2]) -> Self {
        let initial_decks = decks.clone();
        Self {
            decks,
            initial_decks,
            prev_states: HashSet::new(),
            winner: usize::MAX,
        }
    }

    pub fn reset(&mut self) {
        self.decks = self.initial_decks.clone();
        self.prev_states.clear();
        self.winner = usize::MAX;
    }

    pub fn play_simple(&mut self) {
        while !self.decks[0].is_empty() && !self.decks[1].is_empty() {
            let (curr1, curr2) = self.draw_top_cards();
            self.append_to_winner((curr1 < curr2) as usize, curr1, curr2);
        }
        self.winner = self.decks[0].is_empty() as usize;
    }

    pub fn play_recursive(&mut self) {
        while !self.decks[0].is_empty() && !self.decks[1].is_empty() {
            if self.prev_states.contains(&self.decks) {
                break;
            }
            self.save_state();
            let (curr1, curr2) = self.draw_top_cards();
            if curr1 <= self.decks[0].len() && curr2 <= self.decks[1].len() {
                let mut new_game = self.create_subgame(curr1, curr2);
                new_game.play_recursive();
                self.append_to_winner(new_game.winner, curr1, curr2);
            } else {
                self.append_to_winner((curr1 < curr2) as usize, curr1, curr2);
            }
        }
        self.winner = self.decks[0].is_empty() as usize;
    }

    pub fn calculate_score(&self) -> usize {
        self.decks[self.winner]
            .iter()
            .rev()
            .enumerate()
            .fold(0, |acc, (i, v)| acc + (i + 1) * *v)
    }

    pub fn create_subgame(&mut self, take1: usize, take2: usize) -> Self {
        let new_deck1 = self.decks[0].iter().take(take1).copied().collect();
        let new_deck2 = self.decks[1].iter().take(take2).copied().collect();
        Game::new([new_deck1, new_deck2])
    }

    pub fn draw_top_cards(&mut self) -> (usize, usize) {
        let (curr1, curr2) = (
            self.decks[0].pop_front().unwrap(),
            self.decks[1].pop_front().unwrap(),
        );
        (curr1, curr2)
    }

    pub fn append_to_winner(&mut self, winner: usize, val1: usize, val2: usize) {
        self.decks[winner].push_back(if winner == 0 { val1 } else { val2 });
        self.decks[winner].push_back(if winner == 0 { val2 } else { val1 });
    }

    pub fn save_state(&mut self) {
        self.prev_states.insert(self.decks.clone());
    }
}

fn read(filename: &str) -> Game {
    let content = read_to_string(filename).expect("Failed to read file");
    let mut cards = content
        .split_terminator("\n\n")
        .map(|s| s.lines().skip(1).filter_map(|l| l.parse().ok()).collect());
    let decks = [cards.next().unwrap(), cards.next().unwrap()];
    Game::new(decks)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let mut game = read("test-input.txt");
        game.play_simple();
        assert_eq!(game.calculate_score(), 306);

        game.reset();
        game.play_recursive();
        assert_eq!(game.calculate_score(), 291);
    }

    #[test]
    fn test_infinite() {
        let mut game = Game::new([
            VecDeque::from(vec![43, 19]),
            VecDeque::from(vec![2, 29, 14]),
        ]);
        game.play_recursive();
        assert_eq!(game.winner, 0);
    }
}
