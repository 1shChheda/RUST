// ---------------------------------------------------
    // Topic: Functions
    //
    // Task 1:
    // * Displays your first and last name
    // * Use a function to display your first name
    // * Use a function to display your last name
    // * Use the println macro to display messages to the terminal
fn first_name() {
    println!("Vansh");
}

fn last_name() {
    println!("Chheda");
}
    //
    // Task 2:
    // * Perform Arithmetic Ops using functions
fn arithmetic_op(a: i32, b: i32, c: &str) -> i32 {
    if c == "add" {
        return a + b;
        
    } else if c == "subtract" {
        return a - b;

    } else if c == "multiply" {
        return a * b;

    } else if c == "divide" {
        return a / b;

    } else if c == "remainder" {
        return a % b;

    } else {
        return 0;
    }
}
// ---------------------------------------------------
    // Topic: Flow control using if..else
    //
    // Task 3:
    // * Displays a message based on the value of a boolean variable
    // * When the variable is set to true, display "hello"
    // * When the variable is set to false, display "goodbye"
    //
    // * Use a variable set to either true or false
    // * Use an if..else block to determine which message to display
    // * Use the println macro to display messages to the terminal

// ---------------------------------------------------
    // Topic: Decision making with "match"
    //
    // Task 4:
    // * Display "one", "two", "three", or "other" based on whether
    //   the value of a variable is 1, 2, 3, or some other number,
    //   respectively
    //
    // * Use a variable set to any integer
    // * Use a match expression to determine which message to display
    // * Use an underscore (_) to match on any value

// ---------------------------------------------------


fn main() {

    // Task 1
    first_name();
    last_name();

    // Task 2
    let a = 8;
    let b = 6;
    println!("Addition of {a} and {b} = {}", arithmetic_op(a, b, "add"));
    println!("Subtraction of {a} and {b} = {}", arithmetic_op(a, b, "subtract"));
    println!("Multiplication of {a} and {b} = {}", arithmetic_op(a, b, "multiply"));
    println!("Division(RoundOff) of {a} and {b} = {}", arithmetic_op(a, b, "divide"));
    println!("Remainder of {a}/{b} = {}", arithmetic_op(a, b, "remainder"));

    // Task 3
        let my_bool = false;

        if my_bool == true {
            println!("hello");
        } else {
            println!("goodbye");
        }

    // Task 4
    let some_int = 5;
    match some_int {
        1 => println!("its 1"),
        2 => println!("its 2"),
        3 => println!("its 3"),
        _ => println!("its something else"),
    }

    let mut i = 3;
    loop {
        println!("{:?}", i);
        i -= 1;
        if i == 0 {
            break;
        }
    }
    println!("Countdown Done")
    
}