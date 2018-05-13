extern crate rand;

use super::moves::Move;
use super::boosters::Booster;
use super::characters::Character;
use super::outcomes;
use super::players;

use std::io;
use std::str::FromStr;

use self::rand::Rng;

const SINGLE_USE_MOVES: [Move; 3] = [
    Move::Zap,
    Move::Regenerate,
    Move::AcidSpray
];
const DESTRUCTIVE_MOVES: [Move; 2] = [
    Move::Zap,
    Move::AcidSpray
];

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

        println!();
        println!("Choose a character:");
        println!("\tNinja");
        println!("\tZombie");
        println!("\tSamurai");
        println!("\tClown");

        io::stdin().read_line(&mut character_str)
            .unwrap();
        println!();

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
                        println!("The score is now {}-{}.", human.points, computer.points);
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
                    println!("The score is now {}-{}.", human.points, computer.points);
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
        println!();
        println!("Choose your booster:");

        let human_boosters = human.character.get_boosters();

        for booster in &human_boosters {
            println!("\t{}", booster);
        }

        let mut booster_str = String::new();
        io::stdin().read_line(&mut booster_str)
            .unwrap();
        println!();

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
                println!("The score is now {}-{}.", human.points, computer.points);
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

    while human.points < 5 && computer.points < 5 {
        println!();
        println!("Choose a move:");
        let available_human_moves = human.get_available_moves();
        for available_move in &available_human_moves {
            println!("\t{}", available_move);
        }

        let mut move_str = String::new();
        io::stdin().read_line(&mut move_str)
            .unwrap();
        println!();

        let result = Move::from_str(&move_str[..]);

        if let Ok(human_move) = result {
            if !available_human_moves.contains(&human_move) {
                if human.exhausted_moves.contains(&human_move) {
                    println!("{} is single use. 4 wait penalty!", human_move);
                    let new_waits = (human.waits as i16) - 4;
                    if (new_waits >= 0) {
                        human.waits = new_waits as u8;
                    } else {
                        human.waits = 0;
                        computer.points += 1;
                        println!("The score is now {}-{}", human.points, computer.points);
                    }
                    continue;
                } else if human.move_streak.repeated_move == Some(human_move)
                    && human.move_streak.times >= 3
                {
                    println!("You have played {} for the last 3 turns. You cannot play it a 4th time in a row. 3 wait penalty!", human_move);
                    let new_waits = (human.waits as i16) - 3;
                    if (new_waits >= 0) {
                        human.waits = new_waits as u8;
                    } else {
                        human.waits = 0;
                        computer.points += 1;
                        println!("The score is now {}-{}", human.points, computer.points);
                    }
                    continue;
                } else {
                    let human_character_boosters = human.character.get_boosters();
                    let mut human_character_booster_moves = Vec::new();
                    for booster in &human_character_boosters {
                        for booster_move in booster.get_moves() {
                            human_character_booster_moves.push(booster_move);
                        }
                    }

                    if human_character_booster_moves.contains(&human_move) {
                        println!("The move {} is from another booster. 2 wait penalty!", human_move);
                        let new_waits = (human.waits as i16) - 2;
                        if (new_waits >= 0) {
                            human.waits = new_waits as u8;
                        } else {
                            human.waits = 0;
                            computer.points += 1;
                            println!("The score is now {}-{}.", human.points, computer.points);
                        }
                        continue;
                    } else {
                        println!("The move {} is from another character. 3 wait penalty!", human_move);
                        let new_waits = (human.waits as i16) - 3;
                        if (new_waits >= 0) {
                            human.waits = new_waits as u8;
                        } else {
                            human.waits = 0;
                            computer.points += 1;
                            println!("The score is now {}-{}.", human.points, computer.points);
                        }
                        continue;
                    }
                }
            }

            let available_computer_moves = computer.get_available_moves();
            let random_index = rand_int_incl(0, (available_computer_moves.len() - 1) as u8) as usize;
            let computer_move = available_computer_moves[random_index];

            if SINGLE_USE_MOVES.contains(&human_move) {
                if !human.exhausted_moves.contains(&human_move) {
                    human.exhausted_moves.push(human_move);
                }
            }
            if SINGLE_USE_MOVES.contains(&computer_move) {
                if !computer.exhausted_moves.contains(&computer_move) {
                    computer.exhausted_moves.push(computer_move);
                }
            }

            if DESTRUCTIVE_MOVES.contains(&human_move) {
                if !computer.exhausted_moves.contains(&computer_move) {
                    computer.exhausted_moves.push(computer_move);
                }
            }
            if DESTRUCTIVE_MOVES.contains(&computer_move) {
                if !human.exhausted_moves.contains(&human_move) {
                    human.exhausted_moves.push(human_move);
                }
            }

            human.move_streak.update(human_move);
            computer.move_streak.update(computer_move);

            let points = outcomes::get_points(vec![human_move, computer_move]);
            human.points += points[0];
            computer.points += points[1];

            println!("You chose {}.", human_move);
            println!("The computer chose {}.", computer_move);
            if points[0] == 0 && points[1] == 0 {
                println!("As a result, neither of you got a point.");
            } else if points[0] == 0 && points[1] == 1 {
                println!("As a result, the computer got a point.");
            } else if points[0] == 1 && points[1] == 0 {
                println!("As a result, you got a point.");
            } else if points[0] == 1 && points[2] == 1 {
                println!("As a result, each of you got a point.");
            } else {
                panic!("Unhandled outcome: ({}, {})", points[0], points[1]);
            }
            println!("The score is now {}-{}.", human.points, computer.points);

            if human.points == computer.points
                && human.points > 5
            {
                println!("Both of you have {} points. In order to break the tie, the score has been set to 4-4.", human.points);
                human.points = 4;
                computer.points = 4;
            }
        } else {
            println!("The move \"{}\" does not exist. 4 wait penalty!", move_str.trim());
            let new_waits = (human.waits as i16) - 4;
            if (new_waits >= 0) {
                human.waits = new_waits as u8;
            } else {
                human.waits = 0;
                computer.points += 1;
                println!("The score is now {}-{}.", human.points, computer.points);
            }
        }
    }

    println!();
    if computer.points > human.points {
        println!("The computer won {} - {}.", human.points, computer.points);
        return;
    } else {
        println!("You won {} - {}.", human.points, computer.points);
        return;
    }
}
