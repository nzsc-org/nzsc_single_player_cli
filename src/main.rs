extern crate nzsc_single_player;
extern crate nzsc_single_player_text_interface;

use std::io;
use std::str::FromStr;

extern crate rand;
use rand::Rng;

// A note about print! vs println!:
//
// There are a lot of \n and \n\n in the following code.
// It is easy to get confused.
// For the sake of clarity, I intentionally avoided using println!
// This way, every \n is explicit,
//     so you won't have to think about mentally appending one after every println!

fn main() {
    loop {
        let random_seed = rand::thread_rng()::gen_range(1, 4294967296);
        let mut game = nzsc_single_player::single_player_game::SinglePlayerNZSCGame::new(random_seed);
        let mut response = String::new();
        let initial_output = game.initial_output();

        for notification in &initial_output.notifications {
            let notification_string = nzsc_single_player_text_interface::notification::to_string(notification);
            print!("{}\n", notification_string);
        }
        print!("\n");

        let question = initial_output.question.expect("Game's initial_output does not ask a question!");
        let question_string = nzsc_single_player_text_interface::question::to_string(&question);
        print!("{}\n\n", question_string);
        io::stdin().read_line(&mut response)
            .expect("Failed to read line.");
        print!("\n");

        loop {
            let trimmed_input = response.trim().to_string();
            let parsed_input: nzsc_single_player::io::Answer = match &game.phase {
                &nzsc_single_player::single_player_game::Phase::CharacterChoosing {
                    human: _,
                    computer: _,
                } => {
                    let character_selection = if let Ok(selected_human_character) = nzsc_single_player::characters::Character::from_str(&trimmed_input[..]) {
                        nzsc_single_player::io::CharacterSelection::Character(selected_human_character)
                    } else {
                        nzsc_single_player::io::CharacterSelection::Nonexistent(trimmed_input)
                    };
                    nzsc_single_player::io::Answer::CharacterSelection(character_selection)
                },

                &nzsc_single_player::single_player_game::Phase::BoosterChoosing {
                    human: _,
                    computer: _,
                } => {
                    let booster_selection = if let Ok(selected_human_booster) = nzsc_single_player::boosters::Booster::from_str(&trimmed_input[..]) {
                        nzsc_single_player::io::BoosterSelection::Booster(selected_human_booster)
                    } else {
                        nzsc_single_player::io::BoosterSelection::Nonexistent(trimmed_input)
                    };
                    nzsc_single_player::io::Answer::BoosterSelection(booster_selection)
                },

                &nzsc_single_player::single_player_game::Phase::MoveChoosing {
                    human: _,
                    computer: _,
                } => {
                    let move_selection = if let Ok(selected_human_move) = nzsc_single_player::moves::Move::from_str(&trimmed_input[..]) {
                        nzsc_single_player::io::MoveSelection::Move(selected_human_move)
                    } else {
                        nzsc_single_player::io::MoveSelection::Nonexistent(trimmed_input)
                    };
                    nzsc_single_player::io::Answer::MoveSelection(move_selection)
                },
                &nzsc_single_player::single_player_game::Phase::GameOver {
                    human_points: _,
                    computer_points: _,
                } => panic!("The app is trying to handle input after the game is over."),
            };

            let output = game.next(parsed_input).expect("Mismatched types!");
            response = String::new();

            for notification in &output.notifications {
                let notification_string = nzsc_single_player_text_interface::notification::to_string(notification);
                print!("{}\n", notification_string);
            }
            print!("\n");

            if let Some(question) = output.question {
                let question_string = nzsc_single_player_text_interface::question::to_string(&question);
                print!("{}\n\n", question_string);
                io::stdin().read_line(&mut response)
                    .expect("Failed to read line.");
                print!("\n");
            } else {
                break;
            }
        }

        let mut response = String::new();
        print!("Play again? y/N\n\n");
        io::stdin().read_line(&mut response)
            .expect("Failed to read line.");

        if response == ""
            || response.trim().to_lowercase().chars().next().unwrap() != 'y'
        {
            break;
        } else {
            print!("\n");
        }
    }
}
