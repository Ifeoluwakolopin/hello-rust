use std::io;
use std::cmp::Ordering;
use rand::Rng;


fn main(){
    let header = "
============================================================
|                       GUESSING GAME                      |
============================================================
|                    Welcome Challenger!                   |
|                                                          |
|  The computer has generated a random number between      |
|               1 (one) and 100 (hundred).                 |
|                                                          |
|   Try to guess the number before your lives run out.     |
|                        Good luck!!                       |
============================================================\n
";
    println!("{}",header);
    // Start the game with a default number of rounds
    let mut trials: i32 = 6;
    // Available game modes
    let game_mode = ["normal", "hard"];
    let mut selected_mode = game_mode[0];


    // Allow player to choose game mode
    
    let secret_number = rand::thread_rng().gen_range(1..101);
    println!("The random number is {}", secret_number);
    

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
        // Print out the player's guess.
        println!("You guessed: {}", guess);

        if selected_mode == game_mode[0] {
            match guess.cmp(&secret_number) {
                Ordering::Less => println!("Too small"),
                Ordering::Equal =>{
                    println!("You guessed right!");
                    break;
                } 
                Ordering::Greater => println!("Too high")
            }

        } else {
            println!("You selected hard game mode!");
        }    
    }    
}

fn check_guess() -> 