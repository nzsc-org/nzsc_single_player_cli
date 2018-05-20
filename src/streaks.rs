use super::moves::Move;
use super::characters::Character;

pub struct MoveStreak {
    pub repeated_move: Option<Move>,
    pub times: u8
}

pub struct CharacterStreak {
    pub repeated_character: Option<Character>,
    pub times: u8
}

impl MoveStreak {
    pub fn new() -> Self {
        Self {
            repeated_move: None,
            times: 0
        }
    }

    pub fn update(&mut self, new_move: Move) {
        let is_streak_continued = if let Some(repeated_move) = self.repeated_move {
            repeated_move == new_move
        } else {
            false
        };

        self.repeated_move = Some(new_move);
        self.times = if is_streak_continued {
            self.times + 1
        } else {
            1
        }
    }
}

impl CharacterStreak {
    pub fn new() -> Self {
        Self {
            repeated_character: None,
            times: 0
        }
    }

    pub fn update(&mut self, new_character: Character) {
        let is_streak_continued = if let Some(repeated_character) = self.repeated_character {
            repeated_character == new_character
        } else {
            false
        };

        self.repeated_character = Some(new_character);
        self.times = if is_streak_continued {
            self.times + 1
        } else {
            1
        }
    }
}
