fn another_function_top(){
    println!("hi from the top_function");
}

fn main() {
    /* Functions in rust */
    //functions are defined wiht "fn" keyword. then followed by function_name
    //Since main() is the "main function" that rust calls, other functions needs to be included in the inside of the main(){ here }

    another_function_top();
    println!("Hello, world! from main function");
    another_function();
//----------------------------------------------------------------
    /* Part 1 - Parameters */
    function_with_parameters(6);

    //Note that, with parameters/arguments "data type" needs to be defined first.
    print_label_measurement(3, 'h');

    //side-note
    //With 'char type', single-quote. example: 'h'
//----------------------------------------------------------------
    /* Part 2 - Statements and Expressions */
    //Statements = instructions that perform some action and do not return a value
    //Expressions evaluate to a resulting value
    
    let t:i32=5;//This is a statement. This line does not return "5". It simply just stores it.
    
    let t ={//This is an expression. Since "yy+1" evalutes and result in "4". Returns "4" therefore an expression.
        let yy =3;
        yy+1
    }; //In and expression that is covered with {}, ;(semicolon) comes after the }. {};

    // A statement just exsits. An expression do something with exisitance?
    println!("{t}");
//----------------------------------------------------------------
    /* Part 3 - Functions with return values*/
    let f5=five();
    println!("The value of f5 is {f5} and the value is from five()");

    // In order to return a value from a function, use "->". Note that this form of function needs assignment of "datatype" beforehand. Also, in order to return a value, semicolon won't be placed after the closing curly braket. See five()

    let added_one:i32 = add_one(6);
    let original_value:i32 = {
        added_one-1
    };
    println!("A value returned from add_one is {added_one} and the original value is {original_value}");
}

fn five() -> i32{
    5
}

fn add_one(x:i32) -> i32{
    x+1
}

fn another_function(){
    println!("Hi from another_function");
}

fn function_with_parameters(x:i32){
    println!("the value of x from this function parameter is: {x}");
}

fn print_label_measurement(value:i32, unit_label:char){
    println!("the measurement is : {value}{unit_label}");
}