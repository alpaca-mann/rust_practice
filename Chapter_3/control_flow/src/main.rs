fn main() {
    /*
    -Overview-
    If Expressions (expressions return values)
    Loops
    */
    //-------------------------------------
    /* If expressions */

    /*Part 1 - if expressions */
    let number: i8 = 3;
    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    let test_bool:bool=true;

    if test_bool {
    println!("{test_bool}");
    }

    let number_2: i8=6;

    if number_2 % 4 ==0{
        println!("number is divisible by 4");
    }
    else if number_2 % 3 ==0{
        println!("number is divisible by 3");
    }
    else if number_2 % 2 ==0{
        println!("number is divisible by 2");
    }
    else {
        println!("number is not divisible by 4, 3, or 2");
    }

    let condition_bool:bool = false;
    let number_3:i8 = if !condition_bool {5} else {6};

    println!("the value of number_3 is :{number_3}");

    //In rust, values that have the potential to be results from each arm(arm == inside of the {}) of the if must be the same type.

    
    /*Part 1 - End */
//-------------------------------------
    /* Loops */
    // 1. loop

    let mut counter=1;

    let result = loop{
        counter+=1;

        if counter==10 {
        break counter*2;
        }
    };
    println!("{result}");
//-------------------------------------
    //2. loop labels
    //loop labels can be added to loop() by using '(single quote). 

    /*
        If you have loops within loops, break and continue apply to the innermost loop at that point. You can optionally specify a loop label on a loop that we can then use with break or continue to specify that those keywords apply to the labeled loop instead of the innermost loop. Loop labels must begin with a single quote. Here’s an example with two nested loops:
    */
    let mut count:i8 = 0;
    'counting_up: loop {
        println!("count = {count}");
    let mut remaining = 10;

    loop {
            println!("remaining = {remaining}");
            if remaining==9{
                break;
            }
            if count==2 {
                break 'counting_up;
            }
            remaining -= 1;
        }
    count += 1;
    }
    println!("End count = {count}");

    /*
        The outer loop has the label 'counting_up, and it will count up from 0 to 2. The inner loop without a label counts down from 10 to 9. The first break that doesn’t specify a label will exit the inner loop only. The break 'counting_up; statement will exit the outer loop. 
    */
//-------------------------------------
/* Part 3 Conditional Loops with "while" */

let mut number = 3;

while number !=0 {
    println!("{number}!");

    number -= 1;
}
println!("Reached zero");
//-------------------------------------
/* Part 4 Looping Through a Collection with "for" */
let a:[i32; 5] = [10, 20, 30, 40, 50];
for data_set in a{
    println!("the value is: {data_set}");
}

/* Different approach can be done using "while" */
/*
    let a =[10, 20, 30, 40, 50]
    let mut index = 0;

    while index < 5{
        println!("the value is: {}", a[index]);
    index +=1;
    }

But, this approach can cause error. Suppose 50 was removed from the array "a". There are only 4 elements in "a". But, condition that was set for "while" was unchanged. There are no fifth element therefore it could throw array out range error.

By using "for" loop, the probablity of error can be eliminated.

・Excerpt from the book・
Using the for loop, you wouldn’t need to remember to change any other code if you changed the number of values in the array
*/

//Another example of "for" loop

for number in (1..4).rev(){
    println!("{number}!");
}
println!("Reached Zero.");
}
