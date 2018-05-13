extern crate rand;

use super::moves::Move;
use super::boosters::Booster;
use super::characters::Character;
use super::outcomes;
use super::players;

use std::io;
use std::str::FromStr;

use self::rand::Rng;

fn rand_int_incl(min: u8, max_incl: u8) -> u8 {
    rand::thread_rng().gen_range(min, max_incl + 1)
}

pub fn start() {
    let mut human = players::CharacterlessPlayer::new();
    let mut computer = players::CharacterlessPlayer::new();

    let mut pending_human_character: Option<Character> = None;
    let mut pending_computer_character: Option<Character> = None;

    while human.points < 5  && computer.points < 5 {
        let mut character_str = String::new();

        println!("Choose a character:");
        println!("\tNinja");
        println!("\tZombie");
        println!("\tSamurai");
        println!("\tClown");

        io::stdin().read_line(&mut character_str)
            .unwrap();

        let result = Character::from_str(&character_str[..]);

        match result {
            Ok(human_character) => {
                println!("You chose {}.", human_character);

                if human.character_streak.times == 3
                     && human.character_streak.repeated_character == Some(human_character)
                {
                    println!("You chose {} more than 3 times in a row! 3 wait penalty!", human_character);
                    let new_waits = (human.waits as i16) - 3;
                    if (new_waits >= 0) {
                        human.waits = new_waits as u8;
                    } else {
                        human.waits = 0;
                        computer.points += 1;
                        println!("The score is now {}-{}", human.points, computer.points);
                    }
                }

                let computer_character = [
                    Character::Ninja,
                    Character::Zombie,
                    Character::Samurai,
                    Character::Clown
                ][rand_int_incl(0, 3) as usize];

                if computer_character == human_character {
                    println!("The computer also chose {}. You must repick.", computer_character);
                    human.character_streak.update(human_character);
                    computer.character_streak.update(computer_character);
                } else {
                    println!("The computer chose {}.", computer_character);

                    pending_human_character = Some(human_character);
                    pending_computer_character = Some(computer_character);

                    let headstart = outcomes::get_headstart(human_character, computer_character);
                    human.points += headstart.0;
                    computer.points += headstart.1;

                    println!("After applying the headstart, the score is {}-{}.", human.points, computer.points);
                    break;
                }
            },
            Err(()) => {
                println!("The character \"{}\" does not exist. 4 wait penalty!", character_str.trim());
                let new_waits = (human.waits as i16) - 4;
                if (new_waits >= 0) {
                    human.waits = new_waits as u8;
                } else {
                    human.waits = 0;
                    computer.points += 1;
                    println!("The score is now {}-{}", human.points, computer.points);
                }
            }
        }
    }

    if computer.points >= 5 {
        println!("The computer won {} - {}.", human.points, computer.points);
        return;
    } else if human.points >= 5{
        println!("You won {} - {}.", human.points, computer.points);
        return;
    }

    let human_character = pending_human_character.expect("No human character!");
    let computer_character = pending_computer_character.expect("No computer character!");

    let mut human = players::BoosterlessPlayer::new(human, human_character);
    let mut computer = players::BoosterlessPlayer::new(computer, computer_character);

    let mut pending_human_booster: Option<Booster> = None;
    let mut pending_computer_booster: Option<Booster> = None;
    while human.points < 5 && computer.points < 5 {
        println!("Choose your booster:");

        let human_boosters = human.character.get_boosters();

        for booster in &human_boosters {
            println!("\t{}", booster);
        }

        let mut booster_str = String::new();
        io::stdin().read_line(&mut booster_str)
            .unwrap();

        if let Ok(human_booster) = Booster::from_str(&booster_str[..]) {
            println!("You chose {}.", human_booster);

            let computer_boosters = computer.character.get_boosters();
            let computer_booster = computer_boosters[rand_int_incl(0, 1) as usize];

            println!("The computer chose {}.", computer_booster);

            pending_human_booster = Some(human_booster);
            pending_computer_booster = Some(computer_booster);

            break;
        } else {
            println!("The booster \"{}\" does not exist. 4 wait penalty!", booster_str.trim());
            let new_waits = (human.waits as i16) - 4;
            if (new_waits >= 0) {
                human.waits = new_waits as u8;
            } else {
                human.waits = 0;
                computer.points += 1;
                println!("The score is now {}-{}", human.points, computer.points);
            }
        }
    }

    if computer.points >= 5 {
        println!("The computer won {} - {}.", human.points, computer.points);
        return;
    } else if human.points >= 5{
        println!("You won {} - {}.", human.points, computer.points);
        return;
    }

    let human_booster = pending_human_booster.expect("No human booster!");
    let computer_booster = pending_computer_booster.expect("No computer booster!");

    let mut human = players::Player::new(human, human_booster);
    let mut computer = players::Player::new(computer, computer_booster);
}
