use std::collections::HashMap;

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

// ---------------------------------------------------
    // Topic: Memory/Ownership Concept in RUST
    //
    // Task 9: 
    // * Print out the page_count and rating of a book
    // * Use a struct for the book
    // * Use two i32 fields for the page_count and rating
    // * Create a function to display the page_count, with the struct as a parameter
    // * Create a function to display the rating, with the struct as a parameter

    struct Book {
        pages: i32,
        rating: i32,
    }

    fn display_page_count(book: &Book) {
        println!("pages: {}", book.pages);
    }

    fn display_rating(book: &Book) {
        println!("rating: {}", book.rating);
    }

// ---------------------------------------------------
    // Topic: "impl" keyword
    //
    struct Temperature {
        degrees_f: f64,
    }

        // Task 10a: Associated Function with Instance
    // impl Temperature {
    //     fn show_temp(temp: Temperature) {
    //         println!("{} degrees F", temp.degrees_f)
    //     }
    // }

        // Task 10b: Method Implementation
    // impl Temperature {
    //     fn show_temp(&self) {
    //         println!("{} degrees F", self.degrees_f)
    //     }
    // }

        // Task 10c: Associated Function (as Constructor) and Method
    impl Temperature {
        fn freezing() -> Self {
            Self { degrees_f: 32.05 }
        }

        fn show_temp(&self) {
            println!("{} degrees F", self.degrees_f)
        }
    }

        // Task 10d: IMPORTANT
        // * Print the characteristics of a shipping box
        // * Must include dimensions, weight, and color
        //
        // * Use a struct to encapsulate the box characteristics
        // * Use an enum for the box color
        // * Implement functionality on the box struct to create a new box
        // * Implement functionality on the box struct to print the characteristics

    // enum Colour defined already in "Task 5b"
    // enum Colour {
    //     Red,
    //     Yellow,
    //     Blue,
    //     Green
    // }
    impl Colour {
        fn print(&self) {
            match self {
                Colour::Red => println!("Box Colour: Red"),
                Colour::Blue => println!("Box Colour: Blue"),
                Colour::Yellow => println!("Box Colour: Yellow"),
                Colour::Green => println!("Box Colour: Green"),
            }
        }
    }
    struct Dimensions {
        length: i32,
        width: i32,
        height: i32,
    }

    impl Dimensions {
        fn print(&self) {
            println!("Box Dimensions (L): {}", self.length);
            println!("Box Dimensions (H): {}", self.height);
            println!("Box Dimensions (W): {}", self.width);
        }
    }
    struct Box {
        dimensions: Dimensions,
        weight: f64,
        colour: Colour,
    }

    impl Box {
        fn new(dimensions: Dimensions, weight: f64, colour: Colour) -> Self {
            Self { 
                dimensions, 
                weight, 
                colour
            }
        }
        fn print(&self) {
            self.dimensions.print();
            println!("Box Weight: {}", self.weight);
            self.colour.print();
            
        }
    }

// ---------------------------------------------------
    // Topic: Data Structures | Vector
    //
    // Task 11a: Vector Basics
    struct Test {
        score: i32,
    }

    // Task 11b:
    // * Print 10, 20, "thirty", and 40 in a loop
    // * Print the total number of elements in a vector
    //
    // * Use a vector to store 4 numbers
    // * Iterate through the vector using a for..in loop
    // * Determine whether to print the number or print "thirty" inside the loop
    // * Use the .len() function to print the number of elements in a vector
// ---------------------------------------------------
    // Topic: Data Types | Strings
    //
    // Task 12a: String Basics
    struct Employee {
        emp_name: String,
    }

    // Task 12b: Use of String with 
    struct LineItem {
        name: String,
        count: i32,
    }

// ---------------------------------------------------
    // Topic: "derive" functionality
    //
    // Task 13: Use #derive - Basics
    #[derive (Debug, Clone, Copy)]
    enum Position {
        Manager,
        Supervisor,
        Developer,
    }

    #[derive (Debug, Clone, Copy)]
    struct Worker {
        position: Position,
        work_hours: i64,
    }

    fn print_worker(worker: Worker) {
        println!("{:?}", worker);
    }

// ---------------------------------------------------
    // Topic: Advanced Match
    //
    // Task 14a: Use case of Advanced Match
    enum Discount {
        Percent(i32),
        Flat(i32),
    }

    struct Ticket {
        event: String,
        price: f64,
    }

    // Task 14b:
    // * Print out a list of tickets and their information for an event
    // * Tickets can be Backstage, Vip, and Standard
    // * Backstage and Vip tickets include the ticket holder's name
    // * All tickets include the price
    //
    // * Use an enum for the tickets with data associated with each variant
    // * Create one of each ticket and place into a vector
    // * Use a match expression while iterating the vector to print the ticket info

    // #[derive(Debug)]
    enum EventTicket {
        Backstage(f64, String),
        Vip(f64, String),
        Standard(f64),
    }

// ---------------------------------------------------
    // Topic: "Option" Type
    //
    // Task 15a: Basic Use Case

    struct Customer {
        age: Option<i32>,
        email: String,
    }

    // Task 15b: Using "Option" in function returns
    struct ShoppingItem {
        name: String,
        qty: i32,
    }

    fn find_shoppingitem_qty(name: &str) -> Option<i32> {
        let all_shop_items = vec![
            ShoppingItem { name: String::from("bread"), qty: 5 },
            ShoppingItem { name: String::from("milk"), qty: 1 },
            ShoppingItem { name: String::from("cake"), qty: 3 },
        ];

        for item in all_shop_items {
            if item.name == name {
                return  Some(item.qty);
            }
        }

        None
    }


// ---------------------------------------------------
    // Topic: Browsing standard library documentation
    //
    // Task 16:
    // * Browse through various String methods

// ---------------------------------------------------
    // EXTRAS:
    // Topic: "assert" macro
    //
    // Task 17: Basic Use Case

// ---------------------------------------------------
    // Topic: "Result" Data Type
    //
    // Task 18a: Basic Usage

    #[derive(Debug)]
    enum MenuChoice {
        MainMenu,
        Start,
        Quit,
    }

    fn get_choice(input: &str) -> Result<MenuChoice, String> {
        match input {
            "mainmenu" => Ok(MenuChoice::MainMenu),
            "start" => Ok(MenuChoice::Start),
            "quit" => Ok(MenuChoice::Quit),
            _ => Err(String::from("menu choice not found")),
        }
    }

    fn print_choice (choice: &MenuChoice) {
        println!("choice = {:?}", choice);
    }

    // Task 18b:
    //
    // * Create an structure named 'Adult' that represents a person aged 21 or older:
    // * The structure must contain the person's name and age
    // * Implement Debug print functionality using 'derive`
    // * Implement a new function for the 'Adult structure that returns a Result:
    // * The Ok variant should contain the initialized structure, but only
    // if the person is aged 21 or older
    // * The Err variant should contain a String (or &str) that explains why the structure could not be created
    // * Instantiate two 'Adult` structures:
    // * One should be aged under 21
    // * One should be 21 or over
    // * Use `match to print out a message for each 'Adult':
    // * For the Ok variant, print any message you want
    // * For the Err variant, print out the error message
    #[derive(Debug)]
    struct Adult {
        name: String,
        age: i32
    }

    impl Adult {
        fn new(name: String, age: i32) -> Result<Self, String> {
            if age >= 21 {
                Ok(Self{ name, age })
            } else {
                Err(String::from("NOT An Adult!"))
            }
        }
    }

// ---------------------------------------------------
    // Topic: Data Structures | HashMap
    //
    // Task 19a: Basic Use Of HashMaps

    // Task 19b: Depict a Student Locker system
    #[derive(Debug)]
    struct Contents {
        stud_id: u32,
        stud_name: String,
    }


// ---------------------------------------------------
    // Topic: Map Combinators & Option Combinators
    //
    // Task 20a: Map Combinator

    // Task 20b: Option Combinator


// ---------------------------------------------------
    // Topic: Iterator
    //
    // Task 21:
    // a) Basic use case of iterator
    // b) Perform the following:
    // * Triple the value of each item in a vector.
    // * Filter the data to only include values > 10.
    // * Print out each element using a for loop.
    // NOTE: Use an iterator chain to accomplish the task.

// ---------------------------------------------------
    // Topic: Ranges
    //
    // Task 22: Basic Use of Ranges

// ---------------------------------------------------
    // Topic: "if let", "while let"
    //
    // Task 23a: use of "if let"
    enum IntOption {
        Some(i32),
        None,
    }

    // Task 23b: use of "while let"


// ---------------------------------------------------



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

    // Task 9a:
    println!("\n---------TASK9---------");
    let book = Book {
        pages: 32,
        rating: 9
    };

    display_page_count(&book);
    display_rating(&book);
    
    // Task 10a:
    println!("\n---------TASK10---------");
    // let hot = Temperature { degrees_f: 80.93 };
    // Temperature::show_temp(hot);

    // Task 10b:
    let hot = Temperature { degrees_f: 80.93 };
    hot.show_temp();

    // Task 10c:
    let cold = Temperature::freezing();
    cold.show_temp();

    // Task 10d:
    let small_dimensions = Dimensions {
        length: 10,
        width: 20,
        height: 30,
    };

    let small_box = Box::new(small_dimensions, 35.5, Colour::Red);

    small_box.print();

    // Task 11a:
    println!("\n---------TASK11---------");

    let my_scores = vec![
        Test { score: 90 },
        Test { score: 82 },
        Test { score: 75 },
        Test { score: 53 },
    ];

    for test in my_scores {
        println!("score = {}", test.score);
    }

    // Task 11b:
    let my_numbers = vec![10, 20, 30, 40];

    for num in &my_numbers {
        match num {
            30 => println!("thirty"),
            _ => println!("{}", num)
        }
    }

    println!("length of vector: {}", my_numbers.len());

    // Task 12a:
    println!("\n---------TASK12---------");
    let emp = Employee {
        emp_name: String::from("Vansh")
    };

    println!("{}", emp.emp_name);

    // Task 12b:
    let receipt = vec![
        LineItem {
            name: "Arduino".to_owned(),
            count: 3,
        },
        LineItem {
            name: String::from("Adafruit"),
            count: 5,
        }
    ];

    for item in receipt {
        println!("name: {}, count: {}", item.name, item.count);
    }


    // Task 13
    println!("\n---------TASK13---------");

    let me = Worker {
        position: Position::Developer,
        work_hours: 40,
    };

    // Using derived Debug trait (on ENUM) for printing
    println!("{:?}", me.position);

    // Using derived Debug trait (on STRUCT) for printing
    println! ("{:?}", me);

    // Using derived Copy trait (on STRUCT) for copying the data into the function, instead of passing the ownership of the OG data
    print_worker(me);
    print_worker(me);
        // NOTE: DO NOT USE "Clone, Copy" traits on data collections with larger number of fields. UNECESSARY Duplication of Data, memory usage redundancy

    // Task 14a:
    println!("\n---------TASK14---------");

    let flat = Discount::Flat(4);
    match flat {

        // We can use _ to ignore any value that happens to be there 
        // OR 
        // We can also match it to a particular value (ex: Flat(2))
        Discount::Flat(2) => println!("Base Flat Disc. of $2"),
        Discount::Flat(amount) => println!("Sp. Offer Discount of ${}", amount),
        _ => (), // We use () to return NOTHING
    }

    let concert = Ticket {
        event: String::from("OneDirection Concert"),
        price: 49.99
    };

    match concert {

        // If we're concerned with only the "price", then use ".." to ignore other fields
        Ticket{ price: 49.99, event } => println!("Event @ 49.99: {event}"),
        Ticket { price,.. } => println!("Event Price: {price}"),
    }

    // Task 14b:
    let event_tickets = vec![
        EventTicket::Backstage(89.99, String::from("Vansh")),
        EventTicket::Vip(69.99, String::from("Darshil")),
        EventTicket::Standard(69.99),
    ];

    for ticket in event_tickets {
        // println!("{ticket:?}"); // IF WE USE "#[derive]" on EventTicket

        match ticket {
            EventTicket::Backstage(price, holder) => println!("Backstage Ticket Holder: {holder}, Price: {price}"),

            EventTicket::Vip(price, holder) => println!("VIP Ticket Holder: {holder}, Price: {price}"),

            EventTicket::Standard(price) => println!("Standard Ticket Price: {price}"),
        }
    }


    // Task 15a:
    println!("\n---------TASK15---------");

    let mike_rodick = Customer {
        age: Some(22), 
        email: String::from("mikerodick22@example.com"),
    };

    let hugh_jass = Customer {
        age: None, 
        email: String::from("hughjass91@example.com"),
    };

    match mike_rodick.age {
        Some(age) => println!("Customer is {age} yrs old"),
        None => println!("Customer age Not provided"),
    }

    // Task 15b:
    let shopping_item_qty = find_shoppingitem_qty("bread");
    match shopping_item_qty {
        Some(qty) => println!("Item Found! Qty: {qty:?}"),
        None => println!("Item Not Found")
    }

    // Task 16:
    println!("\n---------TASK16---------");

        // "ends_with()" method
    let sample_str1 = "vanshchheda";
    println!("{}", sample_str1.ends_with("eda")); // returns true or false

        // "find()" method
    let sample_str2 = "vansh scores well in test";
    let str2_index = sample_str2.find("s");
    match str2_index {
        Some(n) => println!("Index: {n}"),
        None => println!("Index Not Found")
    }
    
        // "get" method (to get the substring out of a string)
    let sub_str2 = sample_str2.get(0..4);
    match sub_str2 {
        Some(result) => println!("SubString: {}", result),
        None => println!("Nothing")
    }

        // "split & collect" method (split method as used in Javascript)
        // returns a Vector of elements
        // NOTE: you'll have to mention the type (T) here ( Vec<T> )
            // example: Vec<&str>
    let split_str2: Vec<&str> = sample_str2.split(" ").collect();
    for elem in split_str2 {
        println!("{}", elem);
    }


    // Task 17:
    println!("\n---------TASK17---------");

    let mut x = 3;
    let mut y = 7;

    // assert!(x < y); // normal assert macro use.
    assert!(x < y, "x({}) is not GT y({})", x, y); // using assert with a custom panic message

    println!("Hey! the assertion was true, so Program proceeded.");


    // Task 18:
    println!("\n---------TASK18---------");

        // Task 18a:

    fn pick_choice(input: &str) -> Result <(), String> {
        let choice = get_choice(input)?;
        print_choice(&choice);
        Ok(())
    }

    // What is happening in above code?
        // the 'get_choice()' fn gives a "Result" type response ("Ok" or "Err")
        // We still can't access the "inner data" of this Return type
        // So, we have used "?" operator
        // What does "?" operator do?
            // It will check if the response is "Ok" or "Err"
            // if "Ok", then it will store the inner data in the variable
            // if "Err", then it will return the Err value up the call stack (meaning: first to "pick_choice()", representing the String, and then to )
    
    // pick_choice("start"); // This will only handle Successful cases

    // To also handle Err situation (IMPORTANT)
    if let Err(err) = pick_choice("start") {
        println!("Error: {}", err); // Print the error message
    }

    
        // Task 18b:
    fn adult_or_not(name: String, age: i32) -> Result<(), String> {
        let person = Adult::new(name, age)?;
        println!("{} is an Adult! age {}", person.name, person.age);
        Ok(())
    }

    if let Err(err) = adult_or_not("Vansh".to_owned(), 43) {
        println!("Nope! {}", err);
    }

    // Task 19
    println!("\n---------TASK19---------");

    // Task 19a
    let mut people = HashMap::new();
        // Insertion
    people.insert("Vansh", 29);
    people.insert("Arijit", 45);
    people.insert("Shawn", 32);

    match people.get("Vansh") {
        Some(age) => println!("age: {:?}", age),
        None => println!("Not Found"),
    }
        // Deletion (using key)
    people.remove("Shawn");

        // Traversal
        // 1) Both Key & Value
    for (key, val) in people.iter() {
        println!("person: {}, age: {}", key, val);
    }
    
        // 2) Only Keys
    for person in people.keys() {
        println!("person = {}", person);
    }

        // 3) Only Values
    for age in people.values() {
        println!("age = {}", age);
    }

    // Task 19b
    let mut lockers = HashMap::new();
    lockers.insert(1, Contents {
        stud_id: 500032,
        stud_name: "Vansh Chheda".to_owned()
    });
    lockers.insert(2, Contents {
        stud_id: 500102,
        stud_name: "Tanmay Thakkar".to_owned()
    });
    lockers.insert(3, Contents {
        stud_id: 500085,
        stud_name: "Vansh Chheda".to_owned()
    });

    for (locker_num, content) in lockers.iter() {
        println!("LockerNo.: {:?}, StudentId: {:?}", locker_num, content);
    }

    // Task 20:
        println!("\n---------TASK20---------");

        // Task 20a:
        fn maybe_num() -> Option<i32> { Some(32) }

        fn maybe_word() -> Option<String> { Some("Success".to_owned()) } 

            // let plus_one = match maybe_num {
            //     Some(num) => Some(num + 1),
            //     None => None,
            // };

            // Instead of doing the above, where you're getting a "Some(num)" and returning another "Some(num + 1)", do the following:

            let plus_one = maybe_num().map(|num| num + 1);
            // This will return the same thing if we get "Some()", else it wont store anything if we get "None"

            // Chained Map
            let word_length = maybe_word()
                .map(|word| word.len())
                .map(|len| len * 2)
            // println!("{:?}", word_length); // THIS WILL STILL PRINT "Some(14)", and not just "14"
            // Use Of "unwrap_or_default()"
                .unwrap_or_default();
                // With "unwrap_or_default()", if the value is "Some", it will be unwrapped and printed. If it's "None", it will return the default value for the type, which is 0 for integers
                // Now print:
                println!("Final value: {:?}", word_length);


        // Task 20b:
        let a: Option<i32> = Some(1);
        let a_is_some = a.is_some(); // Returns "true" if the option is a 'Some value'.
        let a_is_none = a.is_none(); // Returns "true" if the option is a 'None value'.
        let a_mapped = a.map(|num| num + 1);
        
        let a_filtered = a.filter(|num| num == &1 ); // FILTER: if the Return value is True, we keep the Data, else throw it away (then the final Return value = "None").
        // NOTE: in FILTER, it borrows the number, THUS, use "&" during comparison
        println!("{:?}", a_filtered);

        // Task 21:
        println!("\n---------TASK21---------");
        // a)
        let mut nums = vec![1,2,3,4,5];
        let plus_one: Vec<_> = nums
                .iter()
                .map(|num| num + 1)
                .collect();
        println!("{:?}", plus_one);

        // b)
        let nums = vec![1,2,3,4,5];
        let trip_num: Vec<_> = nums
                .iter()
                .map(|num| num * 3)
                .filter(|num| num > &10)
                .collect();

        println!("{:?}", trip_num);

        // Task 22:
        println!("\n---------TASK22---------");
        let range = 1..=3;
        let range = 1..3;

        for num in range {
            println!("{num:?}");
        }

        for ch in 'a'..='f' {
            println!("{ch:?}");
        }

        // Task 23:
        println!("\n---------TASK23---------");

        // Task 23a:
        let some_value = IntOption::Some(54);

        if let IntOption::Some(value) = some_value {
            println!("Value is: {value}");
        }

        let some_color = Colour::Red;

        if let Colour::Red = some_color {
            println!("its RED!");
        } else {
            println!("its NOT red!")
        }

        // Task 23b:
        let mut some_data = Some(3);

        while let Some(value) = some_data {
            println!("loop");
            some_data = None;
        }
        println!("done 01");

        let some_nums = vec![1,2,3];
        let mut num_iterator = some_nums.iter();

        while let Some(value) = num_iterator.next() {
            println!("num: {:?}", value);
        }
        println!("done 02");


}