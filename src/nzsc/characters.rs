use std::str::FromStr;
use std::fmt;
use super::helpers;
use super::moves::Move;
use super::boosters::Booster;

#[derive(Clone, Copy, PartialEq)]
pub enum Character {
    Ninja = 0,
    Zombie = 1,
    Samurai = 2,
    Clown = 3
}

impl Character {
    pub fn to_u8(self) -> u8 {
        self as u8
    }

    pub fn get_moves(self) -> Vec<Move> {
        match self {
            Character::Ninja => vec![
                Move::Kick,
                Move::NinjaSword,
                Move::Nunchucks
            ],
            Character::Zombie => vec![
                Move::Rampage,
                Move::Muscle,
                Move::Zap
            ],
            Character::Samurai => vec![
                Move::SamuraiSword,
                Move::Helmet,
                Move::Smash
            ],
            Character::Clown => vec![
                Move::JugglingKnives,
                Move::AcidSpray,
                Move::Nose
            ]
        }
    }

    pub fn get_boosters(self) -> Vec<Booster> {
        match self {
            Character::Ninja => vec![
                Booster::Shadow,
                Booster::Speedy,
                Booster::None
            ],
            Character::Zombie => vec![
                Booster::Regenerative,
                Booster::ZombieCorps,
                Booster::None
            ],
            Character::Samurai => vec![
                Booster::Atlas,
                Booster::Strong,
                Booster::None
            ],
            Character::Clown => vec![
                Booster::Backwards,
                Booster::Moustachio,
                Booster::None
            ]
        }
    }
}

impl FromStr for Character {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match &helpers::lowercase_no_whitespace(s)[..] {
            "ninja" => Ok(Character::Ninja),
            "zombie" => Ok(Character::Zombie),
            "samurai" => Ok(Character::Samurai),
            "clown" => Ok(Character::Clown),
            _ => Err(())
        }
    }
}

impl fmt::Display for Character {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let string = match self {
            &Character::Ninja => "Ninja",
            &Character::Zombie => "Zombie",
            &Character::Samurai => "Samurai",
            &Character::Clown => "Clown"
        };

        write!(f, "{}", string)
    }
}
