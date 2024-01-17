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
    // Topic: Working with an "enum"
    //
    // Task 5a:
    // * Use Basic enum to demonstrate its use with "match"
    enum Direction {
        Up,
        Down,
        Left,
        Right
    }
    // Task 5b:
    // * Print the name of a color to the terminal
    //
    // * Use an enum with color names as variants
    // * Use a function to print the color name
    // * The function must use the enum as a parameter
    // * Use a match expression to determine which color
    //   name to print
    enum Colour {
        Red,
        Yellow,
        Blue,
        Green
    }

    fn which_colour(colour: Colour) {
        match colour {
            Colour::Red => println!("its RED"),
            Colour::Yellow => println!("its YELLOW"),
            Colour::Blue => println!("its BLUE"),
            Colour::Green => print!("its GREEN"),
        }
    }

// ---------------------------------------------------
    // Topic: Working with "struct"
    //
    // Task 6a: Basic use of "struct"
    struct GroceryItem {
        stock: i32,
        price: f64
    }

    //
    // Topic: Organizing similar data using structs
    // Task 6b:
    // * Print the flavor of a drink and it's fluid ounces
    // * Use an enum to create different flavors of drinks
    // * Use a struct to store drink flavor and fluid ounce information
    // * Use a function to print out the drink flavor and ounces
    // * Use a match expression to print the drink flavor

    enum Flavour {
        Spicy,
        Fruity,
        Sweet,
    }

    struct Drink {
        flavour: Flavour, // the "type" of the field is "enum"
        fluid_oz: f64,
    }

    fn print_drink(drink: Drink) {
        match drink.flavour { // here, we've to access one field of the "Drink" struct, so lets take "flavour"
            Flavour::Spicy => println!("its SPICY DRINK"),
            Flavour::Fruity => println!("its FRUITY DRINK"),
            Flavour::Sweet => println!("its SWEET DRINK")
        }

        println!("Ounces: {} oz", drink.fluid_oz);
    }

// ---------------------------------------------------
    // Topic: Working with "Tuples"
    //
    // Task 7: Basic use of Tuples
    enum Access {
        Full,
        Restricted,
    }

    fn one_two_three() -> (i32, i32, i32) {
        (1, 2, 3)
    }

// ---------------------------------------------------
    // Topic: Expressions
    //
    // Task 8: Basic use of Expression

fn main() {

    // Task 1
    println!("\n---------TASK1---------");
    first_name();
    last_name();

    // Task 2
    println!("\n---------TASK2---------");
    let a = 8;
    let b = 6;
    println!("Addition of {a} and {b} = {}", arithmetic_op(a, b, "add"));
    println!("Subtraction of {a} and {b} = {}", arithmetic_op(a, b, "subtract"));
    println!("Multiplication of {a} and {b} = {}", arithmetic_op(a, b, "multiply"));
    println!("Division(RoundOff) of {a} and {b} = {}", arithmetic_op(a, b, "divide"));
    println!("Remainder of {a}/{b} = {}", arithmetic_op(a, b, "remainder"));

    // Task 3
    println!("\n---------TASK3---------");
        let my_bool = false;

        if my_bool == true {
            println!("hello");
        } else {
            println!("goodbye");
        }

    // Task 4
    println!("\n---------TASK4---------");
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
    println!("Countdown Done");

    // Task 5a
    println!("\n---------TASK5---------");
    let go = Direction::Left;
    match go {
        Direction::Up => println!("Go UP"),
        Direction::Down => println!("Go DOWN"),
        Direction::Left => println!("Go LEFT"),
        Direction::Right => println!("Go RIGHT"),
    }

    // Task 5b
    which_colour(Colour::Red);

    // Task 6a
    println!("\n---------TASK6---------");
    let oatmeal = GroceryItem {
        stock: 25,
        price: 299.99,
    };
    println!("Stock: {:?}", oatmeal.stock);
    println!("Price: {:?}", oatmeal.price);

    // Task 6b
    let roohafza = Drink {
        flavour: Flavour::Sweet,
        fluid_oz: 32.50
    };
    print_drink(roohafza);
    
    let schezwan = Drink {
        flavour: Flavour::Spicy,
        fluid_oz: 20.64
    };
    print_drink(schezwan);

    // Task 7
    println!("\n---------TASK7---------");
    let numbers = one_two_three(); // direct entire access to tuple with "single variable name"
    let (x, y, z) = one_two_three(); // tuple destructuring
    println!("{:?}, {:?}", x, numbers.0); // 1
    println!("{:?}, {:?}", y, numbers.1); // 2
    println!("{:?}, {:?}", z, numbers.2); // 3

    // tuple with different type of data:
    let (employee, access) = ("Vansh", Access::Full);

    // Task 8
    println!("\n---------TASK8---------");

    let my_num = 3;

    // Example 1: if expression
    let is_lt_5 = if my_num < 5 { true } else { false }; 
        // OR a better way:
    let is_lt_5 = my_num < 5; // the "my_num<5" expression itself produces either "true" or "false" value
    println!("{is_lt_5}");

    // Example 2: match expression
    let message = match my_num {
        1 => "hello",
        _ => "goodbye",
    };
    println!("{message}");

    // Example 3: enum + match expression
    enum Menu {
        Burger,
        Drink,
        Fries,
    }
    let paid = true;
    let item = Menu::Drink;
    let drink_type = "water";
    let order_placed = match item {
        Menu::Drink => {drink_type == "water"},
        _ => true,
    };
    println!("Order Placed? `{order_placed}`");

}