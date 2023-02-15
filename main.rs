//Loops Assignment - Random Number Guessing Game
//Max Edward | 2/15/23

//The std::io brings the in and out operations from the standard library.
use std::io;

//Function for the "pause()" functionality, prevent terminal window from immediately closing.
fn pause() {
    use std::process::Command;
    let _ = Command::new("cmd").arg("/C").arg("pause").status();
}

//Main Function.
fn main() {
    let mut _target: u32 = 0; //Number the computer choses.
    let mut _guess: u32 = 0; //Number that the user has input.
    let mut _tries: u32 = 0; //How many times the user has guessed during a game session.


    //Generate Random Number between 1 and 20.
    _target = (rand::random::<u32>() % 20) + 1;

    //Introducing the user to the game.
    println!("\nHello! Welcome to the Number Guessing Game!\n");
    println!("The number is between 1 and 20. Can you guess what it is?\n");

    //Loop for allowing the user to guess the correct number "x" number of times.
    loop {
        let mut _input = String::new(); //Variable for user input

        //Get user input
        io::stdin()
            .read_line(&mut _input)
            .expect("Failed to read input.");

        //Place user input into "guess" variable. If not a number, reject and ask again.
        _guess = match _input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a valid number.");
                continue;
            }
        };

        //Add +1 to "tries" variable each time the user guesses.
        _tries += 1;

        //If guess = target, reward user.
        if _guess == _target {
            println!(
                "Congradulations! You guessed the number in {} tries!\n",
                _tries
            );
            break;
        //If tries is greater or equal to 10, punish user.
        } else if _tries >= 10 {
            println!(
                "Sorry, you've run out of tries. The correct number was {}.\n",
                _target
            );
            break;
        //If user guesses too low, tell user.
        } else if _guess < _target {
            println!("That number is too low, try again!");
        //If user guesses too high, tell user.
        } else {
            println!("That number is too high, try again!");
        }
    }
    //Pause function for when breaking out of loop.
    pause();
}
