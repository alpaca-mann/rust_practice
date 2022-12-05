use std::io; 
use std::cmp::Ordering; //from standard library, import cmp module, then import Ordering enum type.
use rand::Rng;
fn main() {
    println!("Guess the number");

    /*Added - Part 2 Start */
    let secret_number = rand::thread_rng().gen_range(1..=100);
    /*Added - Part 2 End*/

    println!("The secret number is: {secret_number}");

    println!("Input your guess");

    let mut guess=String::new();
    
    io::stdin()
    .read_line(&mut guess)
    .expect("Failed to read line");

/*Added - Part 2.3 Start */
//Variable Shadowing. 
    let guess:u32 = guess.trim().parse().expect("Please type a number");
    //.trim() this method erases whitespace.
    /* .parse() this method chages type of a variable in this case "string" to "u32".
       .parse() can only change a "String" variable that can be changed into specified type. 
       If string contains a value such as %(thumbs_up emoji), it cannot convert the value.
    */
    //For such a reason, .expect() method is used to handle error.


/*Added - Part 2.3 End */

    println!("you guessed: {guess}");

    /*Added - Part 2.2 Start */
    match guess.cmp(&secret_number){ 
        Ordering::Less=>println!("Too smol"),
        Ordering::Greater=>println!("Too bic"),
        Ordering::Equal=>println!("yay!"),
    }
    /*Added - Part 2.2 End*/
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
