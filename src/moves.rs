use std::str::FromStr;
use std::fmt;
use super::helpers;

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Move {
    Kick = 0,
    NinjaSword = 1,
    Nunchucks = 2,
    ShadowFireball = 3,
    ShadowSlip = 4,
    RunInCircles = 5,
    LightningFastKarateChop = 6,
    Rampage = 7,
    Muscle = 8,
    Zap = 9,
    Regenerate = 10,
    Gravedigger = 11,
    ZombieCorps = 12,
    Apocalypse = 13,
    SamuraiSword = 14,
    Helmet = 15,
    Smash = 16,
    Lightning = 17,
    Earthquake = 18,
    Twist = 19,
    Bend = 20,
    JugglingKnives = 21,
    AcidSpray = 22,
    Nose = 23,
    BackwardsMoustachio = 24,
    NoseOfTheTaunted = 25,
    MustacheMash = 26,
    BigHairyDeal = 27
}

impl Move {
    pub fn to_u8(self) -> u8 {
        self as u8
    }
}

impl FromStr for Move {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match &helpers::lowercase_no_whitespace(s)[..] {
            "kick" => Ok(Move::Kick),
            "ninjasword" => Ok(Move::NinjaSword),
            "nunchucks" => Ok(Move::Nunchucks),
            "shadowfireball" => Ok(Move::ShadowFireball),
            "shadowslip" => Ok(Move::ShadowSlip),
            "runincircles" => Ok(Move::RunInCircles),
            "lightningfastkaratechop" => Ok(Move::LightningFastKarateChop),
            "rampage" => Ok(Move::Rampage),
            "muscle" => Ok(Move::Muscle),
            "zap" => Ok(Move::Zap),
            "regenerate" => Ok(Move::Regenerate),
            "gravedigger" => Ok(Move::Gravedigger),
            "zombiecorps" => Ok(Move::ZombieCorps),
            "apocalypse" => Ok(Move::Apocalypse),
            "samuraisword" => Ok(Move::SamuraiSword),
            "helmet" => Ok(Move::Helmet),
            "smash" => Ok(Move::Smash),
            "lightning" => Ok(Move::Lightning),
            "earthquake" => Ok(Move::Earthquake),
            "twist" => Ok(Move::Twist),
            "bend" => Ok(Move::Bend),
            "jugglingknives" => Ok(Move::JugglingKnives),
            "acidspray" => Ok(Move::AcidSpray),
            "nose" => Ok(Move::Nose),
            "backwardsmoustachio" => Ok(Move::BackwardsMoustachio),
            "noseofthetaunted" => Ok(Move::NoseOfTheTaunted),
            "mustachemash" => Ok(Move::MustacheMash),
            "bighairydeal" => Ok(Move::BigHairyDeal),
            _ => Err(())
        }
    }
}

impl fmt::Display for Move {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let string = match self {
            &Move::Kick => "Kick",
            &Move::NinjaSword => "Ninja Sword",
            &Move::Nunchucks => "Nunchucks",
            &Move::ShadowFireball => "Shadow Fireball",
            &Move::ShadowSlip => "Shadow Slip",
            &Move::RunInCircles => "Run In Circles",
            &Move::LightningFastKarateChop => "Lightning Fast Karate Chop",
            &Move::Rampage => "Rampage",
            &Move::Muscle => "Muscle",
            &Move::Zap => "Zap",
            &Move::Regenerate => "Regenerate",
            &Move::Gravedigger => "Gravedigger",
            &Move::ZombieCorps => "Zombie Corps",
            &Move::Apocalypse => "Apocalypse",
            &Move::SamuraiSword => "Samurai Sword",
            &Move::Helmet => "Helmet",
            &Move::Smash => "Smash",
            &Move::Lightning => "Lightning",
            &Move::Earthquake => "Earthquake",
            &Move::Twist => "Twist",
            &Move::Bend => "Bend",
            &Move::JugglingKnives => "Juggling Knives",
            &Move::AcidSpray => "Acid Spray",
            &Move::Nose => "Nose",
            &Move::BackwardsMoustachio => "Backwards Moustachio",
            &Move::NoseOfTheTaunted => "Nose Of The Taunted",
            &Move::MustacheMash => "Mustache Mash",
            &Move::BigHairyDeal => "Big Hairy Deal"
        };

        write!(f, "{}", string)
    }
}
