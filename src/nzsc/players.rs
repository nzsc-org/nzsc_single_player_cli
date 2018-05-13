use super::moves::Move;
use super::boosters::Booster;
use super::characters::Character;
use super::streaks::{
    MoveStreak,
    CharacterStreak
};

pub struct CharacterlessPlayer {
    pub points: u8,
    pub waits: u8,
    pub character_streak: CharacterStreak
}

pub struct BoosterlessPlayer {
    pub points: u8,
    pub waits: u8,
    pub character: Character
}

pub struct Player {
    pub points: u8,
    pub waits: u8,
    pub character: Character,
    pub booster: Booster,
    pub exhausted_moves: Vec<Move>,
    pub move_streak: MoveStreak
}

impl CharacterlessPlayer {
    pub fn new() -> Self {
        Self {
            points: 0,
            waits: 4,
            character_streak: CharacterStreak::new()
        }
    }
}

impl BoosterlessPlayer {
    pub fn new(characterless_player: CharacterlessPlayer, character: Character) -> Self {
        Self {
            points: characterless_player.points,
            waits: characterless_player.waits,
            character
        }
    }
}

impl Player {
    pub fn new(boosterless_player: BoosterlessPlayer, booster: Booster) -> Self {
        Self {
            points: boosterless_player.points,
            waits: boosterless_player.waits,
            character: boosterless_player.character,
            booster,
            exhausted_moves: Vec::new(),
            move_streak: MoveStreak::new()
        }
    }

    pub fn get_available_moves(&self) -> Vec<Move> {
        let character_moves = self.character.get_moves();
        let booster_moves = self.booster.get_moves();

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
