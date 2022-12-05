use std::io; 
use std::cmp::Ordering; //from standard library, import cmp module, then import Ordering enum type.
use rand::Rng;
fn main() {
    println!("Guess the number");

    /*Added - Part 2 Start */
    let secret_number = rand::thread_rng().gen_range(1..=100);
    /*Added - Part 2 End*/

    println!("The secret number is: {secret_number}");

    /*Part 2.4 Start - creating a game loop */
    loop{//By typing loop{contents}, as long as a user or a program itself needs to exit out of the loop, the program keeps on running.
        
    println!("Input your guess");

    let mut guess=String::new();
    
    io::stdin()
    .read_line(&mut guess)
    .expect("Failed to read line");

/*Part 2.5 Start - Handling Invalid user input */
    let guess:u32 = match guess.trim().parse() {
        Ok(num) => num,
        Err(_) => continue,

        /*
        By using "match" it will check whether .parse() returns ok() or Err(_).
        .parse method returns Rusult Type (retuns ok or err) therefore, if the result of .parse() returns ok, it will return "num"(which contains the user input), if user types something that cannot be converted to specified type(which is u32 type), .parse() will return err variant, the game will "continue" to ask for another input.
        */
    };
    /* Part 2.5 End*/

    println!("you guessed: {guess}");

    /*Added - Part 2.2 Start */
    match guess.cmp(&secret_number){ 
        Ordering::Less=>println!("Too smol"),
        Ordering::Greater=>println!("Too bic"),
        /*Part 2.5 quiting the game after a user guessed the correct number */
        Ordering::Equal=>{println!("yay!");
        break; // break; ends this program because after exiting out of the loop, main function also ends.
        }

        /*Part 2.5 End */
    }
    /*Added - Part 2.2 End*/
    }
    /*Part 2.4 End - Adding game loop */
}

/*・Things done・*/
/*
1. Generating random secret number
import rand dependency in cargo.toml
after importing test build. (cargo build)

#note: if program hangs with "waiting for ....", use "cargo clean" to reset the project. code won't be affected.

2.Comparing the Guess to the Secret Number.

*/

/*・Study notes・*/
/*
In order to understand what traits and method to use from a crate, 
Check the documentations of them. Or stack overflow?

Use "cargo doc --open" to see documentations on each dependencies used on a project.
*/
