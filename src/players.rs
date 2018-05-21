use super::moves::Move;
use super::boosters::Booster;
use super::characters::Character;
use super::streaks::{
    MoveStreak,
    CharacterStreak
};

pub struct Player {
    pub waits: u8,
    pub points: u8,
    pub character: Option<Character>,
    pub booster: Option<Booster>,
    pub character_streak: CharacterStreak,
    pub move_streak: MoveStreak,
    pub exhausted_moves: Vec<Move>,
}

impl Player {
    pub fn new() -> Player {
        Player {
            waits: 4,
            points: 0,
            character: None,
            booster: None,
            character_streak: CharacterStreak::new(),
            move_streak: MoveStreak::new(),
            exhausted_moves: vec![],
        }
    }

    pub fn penalize_waits(&mut self, waits: u8) -> u8 {
        if self.waits < waits {
            self.waits = 0;
            return 1;
        } else {
            self.waits -= waits;
            return 0;
        }
    }

    pub fn available_moves(&self) -> Vec<Move> {
        let character_moves = self.character.expect("Player.available_moves() has been called before Player has chosen a character!").get_moves();
        let booster_moves = self.booster.expect("Player.available_moves() has been called before Player has chosen a booster!").get_moves();

        let mut available_moves = character_moves;
        available_moves.extend(booster_moves);

        let exhausted_moves = &self.exhausted_moves;

        available_moves.retain(|&a| !exhausted_moves.contains(&a));

        if let Some(streak_move) = self.move_streak.repeated_move {
            if self.move_streak.times >= 3 {
                available_moves.retain(|&a| a != streak_move);
            }
        }

        available_moves
    }
}
