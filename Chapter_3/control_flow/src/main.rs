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

    //2. loop labels
    let mut count:i8 = 0;
    'counting_up: loop {
        //println!()
    }
    println!("End count = {count}");
}
