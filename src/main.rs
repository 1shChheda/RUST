//! Hey! Vansh here.
//! 
//! This is module-level documentation.
//!
//! It describes the purpose and functionality of the module.
//! 
// This is a Single-Line Comment
/* And,
This is a Multi-Line Comment */

// Documentation Generation
/// This is a Documentation Comment
/// Education Qualification List
enum Education {
    School(i32),
    College(i32),
    Undergrad(i32),
    Postgrad(i32),
}

/// Student Details
struct Student {
    name: String,
    yrs_exp: i32,
    qualification: Education,
}

/// This calculates total yrs of experience
impl Student {
    fn total_exp(&self) -> i32 {
        match self.qualification {
            Education::School(yrs) => return self.yrs_exp + yrs,
            Education::College(yrs) => return self.yrs_exp + yrs,
            Education::Postgrad(yrs) => return self.yrs_exp + yrs,
            Education::Undergrad(yrs) => return self.yrs_exp + yrs,
        }
    }
}

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

    /// Main function call for Student details
    let myself = Student { name: "Vansh".to_owned(), yrs_exp: 2, qualification: Education::Undergrad(4) };

    println!("Total Exp : {}", myself.total_exp());

}