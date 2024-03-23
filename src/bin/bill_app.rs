use std::io;
use std::collections::HashMap; // NOTE: switching from Vec to HashMap, for better data storage & faster retrieval & deletion

#[derive(Debug, Clone)]
pub struct Bill {
    name: String,
    amount: f64,
}

#[derive(Debug, Clone)]
pub struct Bills {
    inner: HashMap<String, Bill> // We're using the `Bill name` to "key" (index/search) for specific bill, and storing the actual entire bill in "value" section
}

impl Bills {
    fn new() -> Self {
        Self {
            inner: HashMap::new()
        }
    }

    fn add(&mut self, bill: Bill) {
        self.inner.insert(bill.name.to_string(), bill);
    }

    fn get_all(&self) -> Vec<&Bill> {
        return self.inner.values().collect();
    }

    fn remove(&mut self, name: &str) -> bool { // returning "bool" to indicate if the deletion was successful or not
        return self.inner.remove(name).is_some();
    }

    // additional feature: helpful to "view single bill" as well as "edit bill"
    fn get_one(&self, name: &str) -> Result<&Bill, ()> {
        if let Some(bill) = self.inner.get(name) {
            return Ok(bill);
        } else {
            Err(println!("Bill Not Found"))
        }
    }

    fn update(&mut self, name: &str, new_name: &str, amount: f64) -> bool {

            // MAJOR NOTE: 
            // the below commented code works fine, but it doesn't really update the "name" key of the particular Bills HashMap
            // it just updates the `name field` of the `Bill struct` stored inside the `HashMap`
            // however, its not updating the "key" of the `bill` in the `HashMap` to match the new name

        // match self.inner.get_mut(name) {
        //     Some(bill) => {
        //         bill.name = new_name.to_owned();
        //         bill.amount = amount;
        //         true
        //     },

        //     None => false,
        // }

            // So, To fix this issue, 
            // you need to update both the "value" (the `Bill` struct) and the "key" (the bill's name) in the `HashMap` when editing a bill's name. 
            // Corrected code:
        if let Some(bill) = self.inner.remove(name) {
            // "remove()" function?
            // Removes a key from the map, returning the value at the key if the key was previously in the map.

            let updated_bill = Bill { name: new_name.to_owned(), amount };
            self.inner.insert(new_name.to_owned(), updated_bill);
            true

        } else {
            false
        }
    }
}

fn get_input() -> Option<String> { // to get any String input
    let mut buffer = String::new();

    while io::stdin().read_line(&mut buffer).is_err() {
        println!("Please Enter Your Data Again");
    }

    let input = buffer.trim().to_owned();
    if &input == "" {
        None
    } else {
        Some(input)
    }

}

fn get_bill_amount() -> Option<f64> { // to "parse" the String input into Number
    let input = match get_input() {
        Some(input) => input,
        None => return None,
    };
    if &input == "" {
        return None;
    }
    let parsed_input: Result<f64, _> = input.parse(); // IMP: Parses this string slice into another type "T" (which is mentioned in Result<T, _> block)
    if let Ok(amount) = parsed_input {
        return Some(amount);
    } else {
        println!("Error. please enter a valid number!");
        return None;
    }
    
}

mod menu {
    use crate::{ get_bill_amount, get_input, Bill, Bills };

    pub fn add_bill(bills: &mut Bills) {

        println!("-> Add Bill");
        println!("---> Bill Name: ");
        let name = match get_input() {
            Some(input) => input,
            None => return,
        };
        
        println!("---> Bill Amount: ");
        let amount = match get_bill_amount() {
            Some(input) => input,
            None => return
        };

        let bill = Bill { name, amount };
        bills.add(bill);
        println!("BILL ADDED!");

    }

    pub fn view_all_bills(bills: &Bills) {
        // println!("{:?}", bills);

        if bills.inner.is_empty() {
            println!("No Bills Available");

        } else {
            for bill in bills.get_all() {
                println!("{:?}", bill); // using the #[derive(Debug)] to print all properties of each bill
            }
        }
    }

    pub fn remove_bill(bills: &mut Bills) {

        println!("Available Bills: ");
        view_all_bills(bills); // to show the available bills in system

        println!("Enter Bill Name to Remove:");
        let name = match get_input() {
            Some(input) => input,
            None => return,
        };

        if bills.remove(&name) {
            println!("BILL REMOVED!");
        } else {
            println!("Bill Not Found");
        }
    }

    pub fn view_single_bill(bills: &mut Bills) {
        println!("Enter Bill Name to View:");
        let name = match get_input() {
            Some(input) => input,
            None => return,
        };

        if let Ok(bill) = bills.get_one(&name) {
            println!("Found Bill: {:?}", bill);
        }

    }

    pub fn edit_bill(bills: &mut Bills) {

        println!("Available Bills: ");
        view_all_bills(bills); // to show the available bills in system

        println!("Enter Bill Name to Edit:");
        let name = match get_input() {
            Some(input) => input,
            None => return,
        };

        println!("---> New bill Name: ");
        let new_name = match get_input() {
            Some(input) => input,
            None => return,
        };

        println!("---> New bill Amount: ");
        let amount = match get_bill_amount() {
            Some(input) => input,
            None => return
        };

        if bills.update(&name, &new_name, amount) {
            println!("BILL EDITED!");
        } else {
            println!("Bill Not Found");
        }
    }

}

enum MainMenu {
    AddBill,
    ViewAllBills,
    ViewOneBill,
    RemoveBill,
    EditBill
}

impl MainMenu {

    fn from_str(input: &str) -> Option<MainMenu> {
        match input {
            "1" => Some(Self::AddBill),
            "2" => Some(Self::ViewAllBills),
            "3" => Some(Self::ViewOneBill),
            "4" => Some(Self::RemoveBill),
            "5" => Some(Self::EditBill),
            _ => None

        }
    }

    fn display_menu() {
        println!("\n== Bill App Menu ==");
        println!("1) Add Bill");
        println!("2) View All Bills");
        println!("3) View One Bill");
        println!("4) Remove Bill");
        println!("5) Edit Bill");
        println!("\n Enter Selection: ");

    }

}

// creating a separate "run_program" fn, so that we can use "?" operator on input, to handle cases where empty input is recieved from user
fn run_program() -> Option<()> {
    
    let mut bills = Bills::new();
    
    loop {
        MainMenu::display_menu();
        let input = get_input()?;
        match MainMenu::from_str(input.as_str()) {
            Some(MainMenu::AddBill) => {
                menu::add_bill(&mut bills);
            },
            Some(MainMenu::ViewAllBills) => {
                menu::view_all_bills(&bills);
            },
            Some(MainMenu::ViewOneBill) => {
                menu::view_single_bill(&mut bills);
            },
            Some(MainMenu::RemoveBill) => {
                menu::remove_bill(&mut bills)
            },
            Some(MainMenu::EditBill) => {
                menu::edit_bill(&mut bills);
            }
            None => break,
        }
    }
    None
    
}
fn main() {
    run_program();
}