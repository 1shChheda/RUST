// Hey! Vansh here. 
// This is a Single-Line Comment
/* And,
This is a Multi-Line Comment */
fn main() {

    // Basic Warm-Up Code
    let name = "vansh";
    let mut age = 42;
    println!("{} is my name", name);

    if age > 60 {
        println!("{name} is OLD!");
    } else if age > 40 {
        println!("{name} is ADULT!");
    } else {
        println!("{name} is YOUNG!");
    }

    'age_counter: loop {
        if age >= 50 {
            println!("Vansh is OLD now!");
            break 'age_counter;
        }

        println!("{name} turns age {age}!");
        age +=1;
    }

}
