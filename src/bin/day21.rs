use std::{collections::HashMap, mem};

#[derive(Clone, Copy, Debug)]
struct Die {
    rolls: usize,
    next_val: usize,
}

impl Default for Die {
    fn default() -> Self {
        Die {
            rolls: 0,
            next_val: 1,
        }
    }
}

impl Die {
    fn roll(&mut self) -> usize {
        let result = self.next_val;
        self.rolls += 1;
        self.next_val = if self.next_val == 100 {
            1
        } else {
            self.next_val + 1
        };
        result
    }

    fn roll3(&mut self) -> usize {
        self.roll() + self.roll() + self.roll()
    }
}

#[derive(Clone, Copy, Debug)]
struct Game {
    die: Die,
    positions: [usize; 2],
    scores: [usize; 2],
}

impl Game {
    fn new(position1: usize, position2: usize) -> Self {
        Self {
            die: Die::default(),
            positions: [position1 - 1, position2 - 1],
            scores: [0, 0],
        }
    }

    fn play(&mut self) {
        let mut player = 0;
        while self.scores[0] < 1000 && self.scores[1] < 1000 {
            let new_position = (self.positions[player] + self.die.roll3()) % 10;
            self.positions[player] = new_position;
            self.scores[player] += new_position + 1;
            player = 1 - player;
        }
    }

    fn result(&self) -> usize {
        assert!(self.scores[0] >= 1000 || self.scores[1] >= 1000);
        let score = *self.scores.iter().min().unwrap();
        score * self.die.rolls
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
struct DiracGameState {
    positions: [usize; 2],
    scores: [usize; 2],
}

impl DiracGameState {
    fn new(position1: usize, position2: usize) -> Self {
        Self {
            positions: [position1 - 1, position2 - 1],
            scores: [0, 0],
        }
    }

    fn after_roll(&self, player: usize, roll: usize, apply_score: bool) -> Self {
        let mut positions = self.positions;
        let mut scores = self.scores;
        let new_position = (positions[player] + roll) % 10;
        positions[player] = new_position;
        if apply_score {
            scores[player] += new_position + 1;
        }
        Self { positions, scores }
    }

    fn get_winner(&self) -> Option<usize> {
        if self.scores[0] >= 21 {
            Some(0)
        } else if self.scores[1] >= 21 {
            Some(1)
        } else {
            None
        }
    }
}

fn dirac_game_wins(position1: usize, position2: usize) -> [usize; 2] {
    let mut wins = [0, 0];
    let mut round: HashMap<DiracGameState, usize> = Default::default();
    let mut new_round: HashMap<DiracGameState, usize> = Default::default();
    round.insert(DiracGameState::new(position1, position2), 1);
    let mut player = 0;
    while !round.is_empty() {
        for roll_id in 0..3 {
            new_round.clear();
            for roll in 1..=3 {
                for (state, universes) in &round {
                    let new_state = state.after_roll(player, roll, roll_id == 2);
                    if let Some(winner) = new_state.get_winner() {
                        wins[winner] += universes;
                    } else {
                        *new_round.entry(new_state).or_default() += universes;
                    }
                }
            }
            mem::swap(&mut round, &mut new_round);
        }
        player = 1 - player;
    }
    wins
}

fn main() {
    let mut game = Game::new(3, 5);
    game.play();
    println!("Part 1: {}", game.result());
    println!(
        "Part 2: {}",
        dirac_game_wins(3, 5).into_iter().max().unwrap()
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample() {
        let mut game = Game::new(4, 8);
        game.play();
        assert_eq!(game.result(), 739785);
        assert_eq!(dirac_game_wins(4, 8), [444356092776315, 341960390180808]);
    }
}
