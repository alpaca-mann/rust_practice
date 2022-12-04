use std::io; // std = Standard Libary. io=Input/Output libray in std.
//Check the documentation for Standard library for what is capable.
fn main() {
    println!("Guess the number");
    println!("Input your guess");

    let mut guess=String::new();

    io::stdin()
    .read_line(&mut guess)
    .expect("Failed to read line");

    println!("you guessed: {guess}");
}
/*
1. Creating a variable in rust.

let apples=5;

Variables in rust are not changeable(unmutable) by default.

In order to create a changeable(mutable) variables use:

let mut apples=5;
--------------------------------
In this example, 

let mut guess=String::new();

"let mut guess" means create a variable called "guess" and make it mutable.

"String::new();" means inside a guess variable, store empty string.
------------------------
2. what does :: means in this example
example) String::new();

:: is used for calling(aka using) an "assosiated function".
assosiated function is a function for a type(such as int, string, float/double).

In this game,
String:: new(); is equal to create an "empty string" using new() function on String type.

personal note:
something like import <bbb> from <aaa>? import new_func from string_type

std::io::Stdin = From library called "std" import a module called "io" then call the function called "stdio"

Check at https://doc.rust-lang.org/reference/paths.html#path-qualifiers
--------------------------
3. Recieving user input
example) io::stdin().read_line(&mut guess);

・Excerpt from The book・
the line .read_line(&mut guess) calls the read_line method on the standard input handle to get input from the user. We’re also passing &mut guess as the argument to read_line to tell it what string to store the user input in. The full job of read_line is to take whatever the user types into standard input and append that into a string (without overwriting its contents), so we therefore pass that string as an argument. The string argument needs to be mutable so the method can change the string’s content.

The & indicates that this argument is a reference, which gives you a way to let multiple parts of your code access one piece of data without needing to copy that data into memory multiple times. References are a complex feature, and one of Rust’s major advantages is how safe and easy it is to use references. You don’t need to know a lot of those details to finish this program. For now, all you need to know is that like variables, references are immutable by default. Hence, you need to write &mut guess rather than &guess to make it mutable. 

.read_line is a method of stdin() function.

personal note:
a method is a function call of an object.
----------------------------------------------
4. Handling potential Failure with the Result Type
Example) io::stdin().read_line(&mut guess).expect("Failed to read line");

.expect() method is used for potential error handling.

・Excerpt from The book・
As mentioned earlier, read_line puts whatever the user enters into the string we pass to it, but it also returns a Result value. Result is an enumeration, often called an enum, which is a type that can be in one of multiple possible states. We call each possible state a variant.

Chapter 6 will cover enums in more detail. The purpose of these Result types is to encode error-handling information.

Result's variants are Ok and Err. The Ok variant indicates the operation was successful, and inside Ok is the successfully generated value. The Err variant means the operation failed, and Err contains information about how or why the operation failed.

Values of the Result type, like values of any type, have methods defined on them. An instance of Result has an expect method that you can call. If this instance of Result is an Err value, expect will cause the program to crash and display the message that you passed as an argument to expect. If the read_line method returns an Err, it would likely be the result of an error coming from the underlying operating system. If this instance of Result is an Ok value, expect will take the return value that Ok is holding and return just that value to you so you can use it. In this case, that value is the number of bytes in the user’s input.

If you don’t call expect, the program will compile, but you’ll get a warning:

$ cargo build
   Compiling guessing_game v0.1.0 (file:///projects/guessing_game)
warning: unused `Result` that must be used
  --> src/main.rs:10:5
   |
10 |     io::stdin().read_line(&mut guess);
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
   = note: `#[warn(unused_must_use)]` on by default
   = note: this `Result` may be an `Err` variant, which should be handled

warning: `guessing_game` (bin "guessing_game") generated 1 warning
    Finished dev [unoptimized + debuginfo] target(s) in 0.59s
Rust warns that you haven’t used the Result value returned from read_line, indicating that the program hasn’t handled a possible error.

The right way to suppress the warning is to actually write error handling, but in our case we just want to crash this program when a problem occurs, so we can use expect. You’ll learn about recovering from errors in Chapter 9.
---------------------------------------------------
5.Printing Values with println! Placeholders
Aside from the closing curly bracket, there’s only one more line to discuss in the code so far:

    println!("You guessed: {guess}");
This line prints the string that now contains the user’s input. The {} set of curly brackets is a placeholder: think of {} as little crab pincers that hold a value in place. You can print more than one value using curly brackets: the first set of curly brackets holds the first value listed after the format string, the second set holds the second value, and so on. Printing multiple values in one call to println! would look like this:

let x = 5;
let y = 10;

println!("x = {} and y = {}", x, y);
This code would print x = 5 and y = 10.
*/