extern crate nzsc_single_player_cli;
use nzsc_single_player_cli::command_line_app::CommandLineApp;

use std::io;

fn main() {
    loop {
        let mut game = nzsc_single_player_cli::single_player_game::SinglePlayerNZSCGame::new();
        let mut response = String::new();
        let initial_prompt = game.initial_prompt();
        println!("{}", initial_prompt);
        io::stdin().read_line(&mut response)
            .expect("Failed to read line.");

        loop {
            let prompt = game.next(response.trim().to_string());
            response = String::new();
            println!("{}", prompt.text);

            if prompt.is_final {
                break;
            }

            io::stdin().read_line(&mut response)
                .expect("Failed to read line.");
            println!();
        }

        let mut response = String::new();
        println!("Play again? y/N");
        io::stdin().read_line(&mut response)
            .expect("Failed to read line.");

        if response.trim().to_lowercase().chars().next().unwrap() != 'y' {
            break;
        }
    }
}
