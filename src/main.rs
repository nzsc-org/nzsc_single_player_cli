mod nzsc;

use std::io;

fn main() {
    loop {
        nzsc::single_player_game::start();

        let mut response = String::new();
        println!("Play again? y/N");
        io::stdin().read_line(&mut response)
            .expect("Failed to read line.");

        if response.trim().to_lowercase().chars().next().unwrap() != 'y' {
            break;
        }
    }
}
