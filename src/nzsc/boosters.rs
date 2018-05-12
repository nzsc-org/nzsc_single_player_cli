use std::str::FromStr;
use std::fmt;
use super::helpers;
use super::moves::Move;

#[derive(Clone, Copy, PartialEq)]
pub enum Booster {
    Shadow,
    Speedy,
    Regenerative,
    ZombieCorps,
    Atlas,
    Strong,
    Backwards,
    Moustachio,
    None
}

impl Booster {
    pub fn get_moves(self) -> Vec<Move> {
        match self {
            Booster::Shadow => vec![
                Move::ShadowFireball,
                Move::ShadowSlip
            ],
            Booster::Speedy => vec![
                Move::RunInCircles,
                Move::LightningFastKarateChop
            ],
            Booster::Regenerative => vec![
                Move::Regenerate,
                Move::Gravedigger
            ],
            Booster::ZombieCorps => vec![
                Move::ZombieCorps,
                Move::Apocalypse
            ],
            Booster::Atlas => vec![
                Move::Lightning,
                Move::Earthquake
            ],
            Booster::Strong => vec![
                Move::Twist,
                Move::Bend
            ],
            Booster::Backwards => vec![
                Move::BackwardsMoustachio,
                Move::NoseOfTheTaunted
            ],
            Booster::Moustachio => vec![
                Move::MustacheMash,
                Move::BigHairyDeal
            ],
            Booster::None => Vec::new()
        }
    }
}

impl FromStr for Booster {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match &helpers::lowercase_no_whitespace(s)[..] {
            "shadow" => Ok(Booster::Shadow),
            "speedy" => Ok(Booster::Speedy),
            "regenerative" => Ok(Booster::Regenerative),
            "zombiecorps" => Ok(Booster::ZombieCorps),
            "atlas" => Ok(Booster::Atlas),
            "strong" => Ok(Booster::Strong),
            "backwards" => Ok(Booster::Backwards),
            "moustachio" => Ok(Booster::Moustachio),
            "none" => Ok(Booster::None),
            _ => Err(())
        }
    }
}

impl fmt::Display for Booster {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let string = match self {
            &Booster::Shadow => "Shadow",
            &Booster::Speedy => "Speedy",
            &Booster::Regenerative => "Regenerative",
            &Booster::ZombieCorps => "Zombie Corps",
            &Booster::Atlas => "Atlas",
            &Booster::Strong => "Strong",
            &Booster::Backwards => "Backwards",
            &Booster::Moustachio => "Moustachio",
            &Booster::None => "No Booster"
        };

        write!(f, "{}", string)
    }
}
