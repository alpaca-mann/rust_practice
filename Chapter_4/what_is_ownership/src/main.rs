fn main() {
    /* What is Ownership */
    //Memory Management rules which needs to be taken into account when writing with Rust 

    /* About Memory - Stack and Heap */
    /*1. Stack = Think of a stack of plates: when you add more plates, you put them on top of the pile, and when you need a plate, you take one off the top. Adding or removing plates from the middle or bottom wouldn’t work as well! Adding data is called pushing onto the stack, and removing data is called popping off the stack. All data stored on the stack must have a known, fixed size. */

    /*2. Heap = The heap is less organized: when you put data on the heap, you request a certain amount of space. The memory allocator finds an empty spot in the heap that is big enough, marks it as being in use, and returns a pointer, which is the address of that location. This process is called allocating on the heap and is sometimes abbreviated as just allocating (pushing values onto the stack is not considered allocating).  */

    /*3. Accesing the data stored on Heap needs Pointer on the Stacks to navigate between them*/

//---------------------------

    /* Ownership Rules */
    /*
     1. Each value in Rust has an owner.
     2. There can only be one owner at a time.
     3. When the owner goes out of scope, the value will be dropped. 

     - Personal note -
     1. Owner = A person
     2. Each persons are different
     3. When a person goes out of the group(scope) there are in, he/she left the group.
    */

    /* Variable Scope */
    /* Just like other programming languages */
    /* 
        fn main ()
        {
            let s = "hello";// variable "s" is present

        }// variable "s" is gone.
    */

//---------------------------

    /* The String Type */
    
    /* The double colon :: operator allows us to namespace this particular from function under the String type rather than using some sort of name like string_from. We’ll discuss this syntax more in the “Method Syntax” section of Chapter 5  */

    // "String" Type can be mutated.
    let mut s = String::from("hello");
    s.push_str(",World"); //push_str() method is from "String" type.
    println!("{}",s); //Print "hello,World"

    /* Why "String" type is mutable and "str" isn't*/
    //"str" = string literal with known size?
    // "String" = string literal with unknow size?

    /*
    -from the book-
    1. In the case of a string literal, we know the contents at compile time, so the text is hardcoded directly into the final executable. This is why string literals are fast and efficient. But these properties only come from the string literal’s immutability. Unfortunately, we can’t put a blob of memory into the binary for each piece of text whose size is unknown at compile time and whose size might change while running the program.
    */

//---------------------------

    /* Ways Variables and Data Interact: Move */

    let x = 5; // On the stack
    let y = x; // On the stack
    
    

    let s1 = String::from("hello");//on the Heap. Pointer on the Stack.
    let s2 = s1; // s1 is no longer exists. due to "s1" & "s2" pointing to the same data on Heap at the same time. Rust considers "s1" no longer exists after s2 is created.

    println!("{}, world",s2);

    // Read more on https://doc.rust-lang.org/stable/book/ch04-01-what-is-ownership.html#ways-variables-and-data-interact-move
//---------------------------
    /* Way Variables and Data Interact: Clone */
    //Read more on https://doc.rust-lang.org/stable/book/ch04-01-what-is-ownership.html#ways-variables-and-data-interact-clone
    let s3  = String::from("hello");
    let s4 = s3.clone();

    println!("s3 = {}, s4 = {}",s3,s4);

//---------------------------
    /* Stack-Only Data: Copy */
    let xx=5;
    let yy=xx;

    println!("xx = {},yy = {}",xx,yy);

    //Read more on https://doc.rust-lang.org/stable/book/ch04-01-what-is-ownership.html#stack-only-data-copy
    
//---------------------------
/* Ownership and Functions */
/*
let s = String::from("hello");  // s comes into scope

    takes_ownership(s);             // s's value moves into the function...
                                    // ... and so is no longer valid here

    let x = 5;                      // x comes into scope

    makes_copy(x);                  // x would move into the function,
                                    // but i32 is Copy, so it's okay to still
                                    // use x afterward

} // Here, x goes out of scope, then s. But because s's value was moved, nothing
  // special happens.

fn takes_ownership(some_string: String) { // some_string comes into scope
    println!("{}", some_string);
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.

fn makes_copy(some_integer: i32) { // some_integer comes into scope
    println!("{}", some_integer);
} // Here, some_integer goes out of scope. Nothing special happens.
*/
//------------------------------------------------

/* Return values and Scope */
/*
let s1 = gives_ownership();         // gives_ownership moves its return
// value into s1

let s2 = String::from("hello");     // s2 comes into scope

let s3 = takes_and_gives_back(s2);  // s2 is moved into
// takes_and_gives_back, which also
// moves its return value into s3

} // Here, s3 goes out of scope and is dropped. s2 was moved, so nothing
// happens. s1 goes out of scope and is dropped.

fn gives_ownership() -> String {             // gives_ownership will move its
    // return value into the function
    // that calls it

let some_string = String::from("yours"); // some_string comes into scope

some_string                              // some_string is returned and
    // moves out to the calling
    // function
}

// This function takes a String and returns one
fn takes_and_gives_back(a_string: String) -> String { // a_string comes into
             // scope

a_string  // a_string is returned and moves out to the calling function
}*/
//-------------------------------------------------------------

/*Returning multiple values using tuple */
let s1 = String::from("hello");

let (s2, len) = calculate_length(s1);

println!("The length of '{}' is {}.", s2, len);
}

fn calculate_length(s: String) -> (String, usize) {
let length = s.len(); // len() returns the length of a String

(s, length)
}