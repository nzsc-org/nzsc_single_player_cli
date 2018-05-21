extern crate rand;
use self::rand::Rng;

use super::moves::Move;
use super::boosters::Booster;
use super::characters::Character;
use super::outcomes;
use super::players::Player;

use super::command_line_app;

use std::str::FromStr;

const SINGLE_USE_MOVES: [Move; 3] = [
    Move::Zap,
    Move::Regenerate,
    Move::AcidSpray
];
const DESTRUCTIVE_MOVES: [Move; 2] = [
    Move::Zap,
    Move::AcidSpray
];

fn rand_index_incl(max_incl: usize) -> usize {
    rand::thread_rng().gen_range(0, max_incl + 1) as usize
}

fn get_victory_term_by_margin(margin: u8) -> String {
    match margin {
        1 => "Clinch".to_string(),
        2 => "Hypnotization".to_string(),
        3 => "Obliteration".to_string(),
        4 => "Annihilation".to_string(),
        5 => "Wipeout".to_string(),
        _ => {
            panic!("Impossible victory margin: {}", margin);
        }
    }
}

pub struct SinglePlayerNZSCGame {
    human: Player,
    computer: Player,
}

impl SinglePlayerNZSCGame {
    pub fn new() -> SinglePlayerNZSCGame {
        SinglePlayerNZSCGame {
            human: Player::new(),
            computer: Player::new(),
        }
    }
}

impl command_line_app::CommandLineApp for SinglePlayerNZSCGame {
    fn initial_prompt(&self) -> String {
        "Choose a character:\n\tNinja\n\tZombie\n\tSamurai\n\tClown".to_string()
    }

    fn next(&mut self, response: String) -> command_line_app::Prompt {
        let mut output = String::new();

        if let Some(human_booster) = self.human.booster {
            let computer_booster = self.computer.booster.expect("Impossible state: Human has booster but not computer.");
            if let Ok(selected_human_move) = Move::from_str(&response[..]) {
                if self.human.available_moves().contains(&selected_human_move) {
                    let available_computer_moves = self.computer.available_moves();
                    let selected_computer_move = available_computer_moves[
                        rand_index_incl(available_computer_moves.len() - 1)
                    ];

                    output = format!("You chose {}. Computer chose {}.\n", selected_human_move, selected_computer_move);

                    self.human.move_streak.update(selected_human_move);
                    self.computer.move_streak.update(selected_computer_move);

                    if SINGLE_USE_MOVES.contains(&selected_human_move)
                        || DESTRUCTIVE_MOVES.contains(&selected_computer_move)
                    {
                        self.human.exhausted_moves.push(selected_human_move);
                    }
                    if SINGLE_USE_MOVES.contains(&selected_computer_move)
                        || DESTRUCTIVE_MOVES.contains(&selected_human_move)
                    {
                        self.computer.exhausted_moves.push(selected_computer_move);
                    }

                    let mut points = outcomes::get_points(vec![selected_human_move, selected_computer_move]);

                    if selected_human_move == Move::ShadowFireball && selected_computer_move == Move::Smash {
                        if computer_booster == Booster::Strong {
                            points[0] = 0;
                            points[1] = 1;
                        } else {
                            points[0] = 1;
                            points[1] = 0;
                        }
                    } else if selected_human_move == Move::Smash && selected_computer_move == Move::ShadowFireball {
                        if human_booster == Booster::Strong {
                            points[0] = 1;
                            points[1] = 0;
                        } else {
                            points[0] = 0;
                            points[1] = 1;
                        }
                    }

                    self.human.points += points[0];
                    self.computer.points += points[1];

                    if points[0] == 0 && points[1] == 0 {
                        output.push_str("As a result, neither of you gets a point.\n");
                    } else if points[0] == 0 && points[1] == 1 {
                        output.push_str("As a result, the computer gets a point.\n");
                    } else if points[0] == 1 && points[1] == 0 {
                        output.push_str("As a result, you get a point.\n");
                    } else if points[0] == 1 && points[1] == 1 {
                        output.push_str("As a result, both of you get a point.\n");
                    }

                    output.push_str(
                        &format!("The score is now {}-{}.\n", self.human.points, self.computer.points)[..]
                    );

                    if self.human.points == self.computer.points
                        && self.human.points >= 5
                    {
                        self.human.points = 4;
                        self.computer.points = 4;
                        output.push_str("Since there is a tie, the score will be reset to 4-4.\n\n");
                    }

                    if self.human.points < 5 && self.computer.points < 5 {
                        output.push_str("Choose a move:\n");
                        for available_move in &self.human.available_moves() {
                            output.push_str(
                                &format!("\t{}\n", available_move)[..]
                            );
                        }
                    } else {
                        let game_over_message = if self.human.points > self.computer.points {
                            format!("You won {}-{} ({}).\n", self.human.points, self.computer.points, get_victory_term_by_margin(self.human.points - self.computer.points))
                        } else {
                            format!("You lost {}-{} ({}).\n", self.human.points, self.computer.points, get_victory_term_by_margin(self.computer.points - self.human.points))
                        };
                        output.push_str(&game_over_message[..]);
                        return command_line_app::Prompt {
                            text: output,
                            is_final: true,
                        };
                    }
                } else {
                    let mut human_booster_moves: Vec<Move> = vec![];
                    for booster in &self.human.character.expect("Impossible state: Human has booster but no character").get_boosters() {
                        human_booster_moves.extend(booster.get_moves());
                    }

                    // NZSC Rule 3.4.1
                    if self.human.exhausted_moves.contains(&selected_human_move) {
                        self.computer.points += self.human.penalize_waits(4);
                        output = format!("{} is single-use. You cannot use it again. 4 wait penalty!\nThe score is now {}-{}.\n", selected_human_move, self.human.points, self.computer.points);
                    }
                    // NZSC Rule 3.4.2
                    else if self.human.move_streak.repeated_move == Some(selected_human_move)
                        && self.human.move_streak.times == 3
                    {
                        self.computer.points += self.human.penalize_waits(3);
                        output = format!("You have already chosen {} 3 times in a row. You must choose something else before choosing it again. 3 wait penalty!\nThe score is now {}-{}.\n", selected_human_move, self.human.points, self.computer.points);
                    }
                    // NZSC Rule 3.4.3
                    else if human_booster_moves.contains(&selected_human_move) {
                        self.computer.points += self.human.penalize_waits(2);
                        output = format!("{} is from the wrong booster. 2 wait penalty!\nThe score is now {}-{}.\n", selected_human_move, self.human.points, self.computer.points);
                    }
                    // NZSC Rule 3.4.4
                    else {
                        self.computer.points += self.human.penalize_waits(3);
                        output = format!("{} is from the wrong character. 3 wait penalty!\nThe score is now {}-{}.\n\n", selected_human_move, self.human.points, self.computer.points);
                    }

                    if self.computer.points < 5 {
                        output.push_str("Choose a move:\n");
                        for available_move in &self.human.available_moves() {
                            output.push_str(
                                &format!("\t{}\n", available_move)[..]
                            );
                        }
                    } else {
                        output.push_str(
                            &format!("You lost {}-{} ({}).\n", self.human.points, self.computer.points, get_victory_term_by_margin(self.computer.points - self.human.points))[..]
                        );

                        return command_line_app::Prompt {
                            text: output,
                            is_final: true,
                        };
                    }
                }
            } else {
                self.computer.points += self.human.penalize_waits(4);
                output = format!("\"{}\" is not a move. 4 wait penalty!\nThe score is now {}-{}.\n\n", response, self.human.points, self.computer.points);

                if self.computer.points < 5 {
                    output.push_str("Choose a move:\n");
                    for available_move in &self.human.available_moves() {
                        output.push_str(
                            &format!("\t{}\n", available_move)[..]
                        );
                    }
                } else {
                    output.push_str(
                        &format!("You lost {}-{} ({}).\n", self.human.points, self.computer.points, get_victory_term_by_margin(self.computer.points - self.human.points))[..]
                    );

                    return command_line_app::Prompt {
                        text: output,
                        is_final: true,
                    };
                }
            }
        } else if let Some(human_character) = self.human.character {
            let computer_character = self.computer.character.expect("Impossible state: Human has character but not computer.");
            if let Ok(selected_human_booster) = Booster::from_str(&response[..]) {
                let selected_computer_booster = computer_character.get_boosters()[rand_index_incl(1)];
                self.human.booster = Some(selected_human_booster);
                self.computer.booster = Some(selected_computer_booster);

                output = format!("You chose {}.\nComputer chose {}.\nLet the battle begin!\n\nChoose a move:\n", selected_human_booster, selected_computer_booster);
                for available_move in &self.human.available_moves() {
                    output.push_str(
                        &format!("\t{}\n", available_move)[..]
                    );
                }
            } else {
                self.computer.points += self.human.penalize_waits(3);
                output.push_str(
                    &format!("\"{}\" is not a booster. 3 wait penalty!\nThe score is now {}-{}.\n\n", response, self.human.points, self.computer.points)[..]
                );

                if self.computer.points < 5 {
                    output.push_str("Choose a booster:\n");
                    for booster in &human_character.get_boosters() {
                        output.push_str(
                            &format!("\t{}\n", booster)[..]
                        );
                    }
                } else {
                    output.push_str(
                        &format!("You lost {}-{} ({}).\n", self.human.points, self.computer.points, get_victory_term_by_margin(self.computer.points - self.human.points))[..]
                    );

                    return command_line_app::Prompt {
                        text: output,
                        is_final: true,
                    };
                }
            }
        } else {
            if let Ok(selected_human_character) = Character::from_str(&response[..]) {
                let selected_computer_character = [
                    Character::Ninja,
                    Character::Zombie,
                    Character::Samurai,
                    Character::Clown
                ][rand_index_incl(3)];
                if selected_human_character == selected_computer_character {
                    self.human.character_streak.update(selected_human_character);
                    self.computer.character_streak.update(selected_computer_character);
                    output.push_str(
                        &format!("\nBoth of you chose {0}, so you must repick.\nYou have picked {0} {1} times.\nComputer has picked {0} {2} times.\n", selected_human_character, self.human.character_streak.times, self.computer.character_streak.times)[..]
                    );
                } else {
                    self.human.character = Some(selected_human_character);
                    self.computer.character = Some(selected_computer_character);
                    output.push_str(
                        &format!("\nYou chose {}.\nComputer chose {}.\n\n", selected_human_character, selected_computer_character)[..]
                    );
                    output.push_str("Choose a booster:\n");
                    for booster in &selected_human_character.get_boosters() {
                        output.push_str(
                            &format!("\t{}\n", booster)[..]
                        );
                    }
                }
            } else {
                self.computer.points += self.human.penalize_waits(3);
                output = format!("\"{}\" is not a character. 3 wait penalty!\nThe score is now {}-{}.\n\n", response, self.human.points, self.computer.points);

                if self.computer.points  < 5 {
                    output.push_str("Choose a character:\n\tNinja\n\tZombie\n\tSamurai\n\tClown")
                } else {
                    output.push_str(
                        &format!("You lost {}-{} ({}).\n", self.human.points, self.computer.points, get_victory_term_by_margin(self.computer.points - self.human.points))[..]
                    );

                    return command_line_app::Prompt {
                        text: output,
                        is_final: true,
                    };
                }
            }
        }

        command_line_app::Prompt {
            text: output,
            is_final: false
        }
    }
}
