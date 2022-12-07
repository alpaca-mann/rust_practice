use std::io;

fn main() {
    /*
    - Overview -
     Rust is statically type language.
     Which means that every value in rust is of a certain data type.
     In the book, two data type subsets are introduced.

     1. Scalar
     2. compound
     */
//----------------------------------------------------------------

    /* Part 1 - Example from guessing game */
    let guess = "4";
    println!("value type of {guess} is String");

    //Variable Shadowing which creates another variable of "guess"
    let guess:u32 = guess.parse().expect("not a number");

    println!("value type of {guess} is now U32");

    // If ":u32" is not specified, the compiler gives an error sayng "consider giving 'guess' a type"
    

//----------------------------------------------------------------

    /* Scaler Types */
    //A Scaler type = single value type. Integers, floating-point numbers, Booleans, and characters

    // Integer Type
println!("Rust offers, signed and unsigned integers");
println!("And there are: 8 bit, 16 bit, 32 bit, 64,bit, 128 bit, arch.");
println!("arch is architecture specific meaning, value type will be 64 bits, if the machine is 64 bits architecture and so on.");

/* 
On How to hand Integer Overflow can be found at:
https://doc.rust-lang.org/stable/book/ch03-02-data-types.html#integer-overflow
*/
     /* Part 2 - End */

//----------------------------------------------------------------

/* Part 3 - Floating Point Type */
let x = 2.0;
let y:f32 = 3.0;

println!("x is : {x}, and y is : {y}");

/* Part 3 End */

//----------------------------------------------------------------

/*Part 4 - Numerical Operations */
// addition
let sum = 5 + 10;

// subtraction
let difference = 95.5 - 4.3;

// multiplication
let product = 4 * 30;

// division
let quotient = 56.7 / 32.2;
let floored = 2 / 3; // Results in 0

// remainder
let remainder = 43 % 5;
/* Part 4 End */

//----------------------------------------------------------------

/* Part 5 Boolean type  */
let t =true;
let f:bool = false; //explicit type annotation.
/* Part 5 End */

//----------------------------------------------------------------

/* Part 6 Character type*/
//Use '' (signle quote) for char type.
let c='z';
let z:char='X';
let cat='😻';
println!("{cat}");
/* Part 6 End  */

//----------------------------------------------------------------

/* Compound Type */
println!("Tuple and Array type cannot grow or shrink in size. Vec type can but covered in later chapter");
//Compound type is used for grouping multiple values into one type.

/* part 7 - Tuple type  */
let tup:(i32,f64,u8) =(500,6.4,2);

//In order to pull the values of "tup", "tup" must be destructured.
//There are two ways

let (x,y,z) =tup;
println!("the value of y is: {y}");

//Here we assign 500 to "x", 6.4 to "y", and 2 to "z"
//println!() macro prints out "y" which is 6.4

//Another way to destructure "tup" is to used .(period)
let five_hundred = tup.0;
let six_point_four=tup.1;
let two=tup.2;

//This way destructure "tup" in index position. "x" = 0,"y" = 1,"z" =2
//the index is starts from 0 in Rust. 

//Tuple as units: 
/*
The tuple without any values has a special name, unit. This value and its corresponding type are both written () and represent an empty value or an empty return type. Expressions implicitly return the unit value if they don’t return any other value.
 */
/* Part 7 End */
 
/* part 8 - Array Type*/
//Unlike tuple which can contain several types, array can only contain one type.

let array_example=[1,2,3,4,5];

let first = array_example[0];
let second = array_example[1];

println!("In array_example, first value is {first} and its index position is 0");
/* part 8 End */

array_example_function();
}

fn array_example_function(){
    println!("-----------------------------");
    let a =[1,2,3,4,5];
    println!("please enter an array index from 1 to 5.");
    println!("If input goes is larger than 5, the program result in error\n");
    println!("This behaviour is an example of Rust's memory safety principles in action.\n");
    println!("Read more on: https://doc.rust-lang.org/stable/book/ch03-02-data-types.html#invalid-array-element-access\n");
    println!("Now, enter a number. Try typing 10 :)");

    let mut index=String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize =index
        .trim()
        .parse()
        .expect("Index entered was not a number");
    let element = a[index];
 
    println!("the value of the element at index {index} is: {element}");
}