use std::io::{self};
use std::cmp::Ordering;
use rand::Rng;


fn main(){
    let header = "
============================================================
|                       GUESSING GAME                      |
============================================================
|                    Welcome Challenger!                   |
|                                                          |
|       The computer has generated a random number.        |
|   Try to guess the number before your chances run out.   |
|                        Good luck!!                       |
============================================================";
    println!("{}",header);

    // Allow player to choose game mode
    println!(
"Please select a game mode:
Normal - You get 6 guesses for a number between 0 and 100
Advanced - You get 5 guesses for a number between 0 and 150

Type 0 for Normal mode, and 1 for Advanced.
Hit 'Enter' to skip selection. Default game mode will be Normal.
    ");
    // Store the number of player wins
    let mut wins: i32 = 0;
    let mut playing: bool = true;

    // Create string to store user input for game mode
    let mut player_mode = String::new();
        
    // read player input
    io::stdin().read_line(&mut player_mode).expect("Failed to read line");
    // parse input from string to integer32
    let player_mode: u32 = match player_mode.trim().parse(){
        Ok(num) => num,
        Err(_) => 0,
    };

    while playing {

        // Start the game with a default number of guesse based on player input
        let mut guesses: u32 = match player_mode {
            0 => {
                println!("selected NORMAL mode...Starting game...");
                6
            },
            1 => {
                println!("selected ADVANCED mode...Starting game...");
                5
            },
            _ => {
                println!("...Starting game...");
                6
            },
        };

        let end_range = match guesses {
            5 => 151,
            6 => 101,
            _ => 101
        };

        let secret_number = rand::thread_rng().gen_range(1..end_range);
        println!("The Secret number is between 0 and {}", end_range-1);
        
        // Print the number of tries the player has
        println!("You have {} guesses for this game", guesses);
        loop {
            // Take guess as input
            
            println!("Please input your guess. ");
            let mut guess = String::new();

            io::stdin().read_line(&mut guess)
            .expect("Failed to read line");

            // check if the guess is a positive integer.
            let guess: u32 = match guess.trim().parse(){
                Ok(num) => num,
                Err(_) => continue,
            };
            println!(
    "-------------------------"
            );
            
            match guess.cmp(&secret_number) {
                Ordering::Less => println!("Too small"),
                Ordering::Equal =>{
                    println!("YOU GUESSED RIGHT!");
                    wins += 1;
                    break;
                }
                Ordering::Greater => println!("Too high")
            }

            guesses -= 1;
            if guesses == 0 {
                let s: String = secret_number.to_string();
                println!("
=========================================
|   Sorry, You have run out of guesses. |
|                                       |
|   The hidden number was {}.           |
|   Try again next time!                |
=========================================", s
            );
                break;
            };
            println!(
    "You have {} guesses left.
    -------------------------
            ", guesses);
        }
        println!("You have won {} times.", &wins);
        println!("Press any key to play again or hit X to exit the game.");
        let mut plays_again = String::new();
        io::stdin().read_line(&mut plays_again).expect(
            "Unable to readline, please press a valid key"
        );
        
        match plays_again.chars().next().unwrap() {
            'x' | 'X' => {
                playing = false;
            },
            _ => {
                println!("Starting new round...");
            }
        };
    }

}