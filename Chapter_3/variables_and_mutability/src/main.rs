fn main() {
    /*Part 1 - Changing a value with "mut". */
    let x=5;
    println!("Value of x is: {x}");

    /*
    x=6; // Changing value of "x" like this won't work in rust
    println!("Value of x is now : {x}")
    */

    /*In order to change the value of "x", "mut" is needed*/
    let mut y=6;
    println!("value of y is: {y}");
    y=7;
    println!("value of y is changed to: {y}");
    /*Part 1 End */

    /*Part 2 - Constants*/

    //Constants are always immutable. Variable name has to be in All UPPER_CASE with UNDER_SCORES in between. Be carful with local/global scope.

    const THREE_HORUS_IN_SECONDS: u32 =60*60*3;

    //For more info check this URL: https://doc.rust-lang.org/stable/reference/const_eval.html

    /*Part 2 End */

    /*Part 3 - Shadowing */

    let z=5;
    let z=z+1;

    {
        let z=z*2;
        println!("The value of z in the inner scope is: {z}");
    }

    print!("The value of z is: {z}");
    
    /*Part 3 End */

    /* Part 4 - difference between "mut" and shadowing  */
    
    let spaces="     ";//"String" type
    let spaces = spaces.len();//.len() method returns "usize" type

    /* Compiler error example */
    /*
    let mut spaces = "    ";
    spaces=spaces.len();
    */

    //The two lines above are trying to change "String" type to "usize" type. "mut" allows change of a "value" with "same type", but does not allow change of a "type" in value.
    
    /* Part 4 End */
}

/*
More onDiffernce between "mut" and "shadowing"

Excellent answer can be found on here: https://stackoverflow.com/questions/53235334/in-rust-whats-the-difference-between-shadowing-and-mutability

"mut" simply allows creaters to change a value.
"shadowing" creates another value.
Example of shadowing:

let x =6; 
let x =7;

first "x" is x_0
second "x" is x_1 (a new variable "x" created and stored in memory.)

Thefore, if a value needs to be changed during execution, use "mut"
If a value nneds to be copied and then used, use "shadowing"

See more in Part 4 above
*/