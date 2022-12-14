fn main() {
/* References and Borrowing */

/* Tuple ver */
/*The issue with the tuple code is that we have to return the String to the calling function so we can still use the String after the call to calculate_length, because the String was moved into calculate_length. */

let s1 = String::from("hello");

    let (s2, len) = calculate_length_tuple(s1);

    println!("The length of '{}' is {}.", s2, len);
//--------------------------------------------------
/* Reference ver */

let s1_r:String = String::from("hello");

let len_r = calculate_length_reference(&s1_r);

println!("The length of '{}' is {}.",s1_r,len_r);

/*
note that we pass &s1 into calculate_length and, in its definition, we take &String rather than String. These ampersands represent references, and they allow you to refer to some value without taking ownership of it.

-Personal note-
withoug taking ownership = without creating unnecessary variables. without allocating unnecessary space onto Stack/Heap with identical data.
references = asking whether data is stored on stack/heap. If there are, use it as read only. Use the value stored.
 */
//--------------------------------------------------
/* Mutable References */

let mut s = String::from("hello");

change(&mut s); 
// value stored in "s" will be used as writable read only. 
//change() will use writable read only value, concatnate with ", world". the result is stored onto "some_string".
//---------------------------------------------

/* Rules of mutable references */
// 1. only one single mutable reference is allowed. 

let mut s_mut=String::from("hello");

let r1 = &mut s_mut;
//let r2 = &mut s_mut; <- "r2" attempts to refer to "s_mut" at the same time as "r1". Compile error.

//--------------------------------------------------------

/* Alternative ways of referencing same mutable variable. */
let mut s_mut2=String::from("hello there");
{
let r1_2 = &mut s_mut2;
println!("{r1_2}");
}//r1_2 goes out of scope. reference to s_mut2 ends here. 

//Therefore can be referenced again.
let r2_2 = &mut s_mut2;
println!("{r2_2}");


let mut s_mut3:String = String::from("hi");

let r3_mut: &mut String = &mut s_mut3;

println!("{}, {}", r2_2, r3_mut);

//--------------------------------------------------------
/* Rule 2 - if immutable reference happens before mutable reference, comiple error */

/* Error ver */
let mut s_rule2 = String::from("hello");

let r1_rule2 = &s_rule2; // no problem
let r2_rule2 = &s_rule2; // no problem
//let r3_rule2 = &mut s_rule2; // BIG PROBLEM. violates Rule 2

println!("{}, {}, and {}", r1_rule2, r2_rule2, r2_rule2);
//Users of an immutable reference don’t expect the value to suddenly change out from under them!

//---------------------------------------------

/* Ok ver */
let mut s_rule2_ok = String::from("hello");

    let r1_rule2_ok = &s_rule2_ok; // no problem
    let r2_rule2_ok = &s_rule2_ok; // no problem
    println!("{} and {}", r1_rule2_ok, r2_rule2_ok);
    // variables r1 and r2 will not be used after this point

    let r3_rule2_ok = &mut s_rule2_ok; // no problem
    println!("{}", r3_rule2_ok);

    /*
    This is called Non-Lexical Lifetimes(NLL)
    More on: https://blog.rust-lang.org/2018/12/06/Rust-1.31-and-rust-2018.html#non-lexical-lifetimes
    */
//---------------------------------------------

/* Dangling References */ 
let reference_to_something = dangle_ok_ver();
println!("{reference_to_something}");
}

/*
fn dangle_err_ver() -> &String {
    let s = String::from("hello");

    &s
}//variable "s" is out of memory here. therefore cannot reference. cannot find read only data.
 */
fn dangle_ok_ver() -> String{
    let s = String::from("Hello again from dangle function");

    s

//---------------------------------------------

/* The Rules of References */
/*
Let’s recap what we’ve discussed about references:

1. At any given time, you can have either one mutable reference or any number of immutable references.
2. References must always be valid.( Must always can find read only data)

Next, we’ll look at a different kind of reference: slices.
*/
}

//--------------------------------------------------------
// Aux functions.
fn change(some_string: &mut String){
    some_string.push_str(", world");
    println!("{some_string}");
}

fn calculate_length_tuple(s: String) -> (String, usize) {
    let length = s.len(); // len() returns the length of a String

    (s, length)
}

fn calculate_length_reference(s_r:&String)-> usize{
    s_r.len()
}
