#![allow(dead_code)]

use std::usize;
struct Game {
    rolls: Vec<u16>,
}

impl Game {
    fn new() -> Game {
        Game {
            rolls: Vec::<u16>::new(),
        }
    }

    fn roll(&mut self, pins_down: u16) {
        self.rolls.push(pins_down);
    }

    fn score(&self) -> u16 {
        let mut frame: usize = 0;
        let mut score = 0;
        for _ in 0..10 {
            if self.is_strike(frame) {
                score += 10 + self.strike_bonus(frame);
                frame += 1; // strike : next roll starts a new frame
            } else if self.is_spare(frame) {
                score += 10 + self.spare_bonus(frame);
                frame += 2;
            } else {
                score = score + self.first_roll(frame) + self.second_roll(frame);
                frame += 2;
            }
        }
        score
    }

    fn first_roll(&self, frame: usize) -> u16 {
        *self.rolls.get(frame).unwrap()
    }

    fn second_roll(&self, frame: usize) -> u16 {
        *self.rolls.get(frame + 1).unwrap()
    }

    fn is_strike(&self, frame: usize) -> bool {
        self.first_roll(frame) == 10
    }

    fn strike_bonus(&self, frame: usize) -> u16 {
        self.rolls.get(frame + 1).unwrap() + self.rolls.get(frame + 2).unwrap()
    }

    fn is_spare(&self, frame: usize) -> bool {
        !self.is_strike(frame) && self.first_roll(frame) + self.second_roll(frame) == 10
    }

    fn spare_bonus(&self, frame: usize) -> u16 {
        *self.rolls.get(frame + 2).unwrap()
    }
}

#[test]
fn all_gutter_game_scores_0() {
    let mut game = Game::new();
    roll_many_times(&mut game, 20, 0);
    assert_eq!(0, game.score());
}

#[test]
fn all_open_frame_game_scores_sum_of_rolls() {
    let mut game = Game::new();
    roll_many_times(&mut game, 10, 1);
    roll_many_times(&mut game, 10, 2);
    assert_eq!(30, game.score());
}

#[test]
fn one_spare_adds_next_roll_as_bonus_score() {
    let mut game = Game::new();
    roll_many_times(&mut game, 3, 5); //spare, first frame score is 15 not 10 !
    roll_many_times(&mut game, 17, 1);
    assert_eq!(37, game.score());
}

#[test]
fn one_strike_adds_next_2_rolls_as_bonus_score() {
    let mut game = Game::new();
    game.roll(10);
    roll_many_times(&mut game, 2, 3); //total 6 strike bonus on first roll !
    roll_many_times(&mut game, 16, 1);
    assert_eq!(38, game.score());
}

#[test]
fn all_strikes_game_scores_300() {
    let mut game = Game::new();
    roll_many_times(&mut game, 12, 10);
    assert_eq!(300, game.score());
}

// test helper function, throws many rolls at once
fn roll_many_times(game: &mut Game, nb: u8, pins_down: u16) {
    for _ in 0..nb {
        game.roll(pins_down);
    }
}
