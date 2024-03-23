use std::io;

#[derive(Debug, Clone)]
pub struct Bill {
    name: String,
    amount: f64,
}

#[derive(Debug, Clone)]
pub struct Bills {
    inner: Vec<Bill>
}

impl Bills {
    fn new() -> Self {
        Self {
            inner: vec![]
        }
    }

    fn add(&mut self, bill: Bill) {
        self.inner.push(bill);
    }

    fn get_all(&self) -> Vec<&Bill> {
        return self.inner.iter().collect();
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

    pub fn view_bills(bills: &Bills) {
        // println!("{:?}", bills);

        if bills.inner.is_empty() {
            println!("No Bills Available");

        } else {
            for bill in bills.get_all() {
                println!("{:?}", bill); // using the #[derive(Debug)] to print all properties of each bill
            }
        }
    }

}

enum MainMenu {
    AddBill,
    ViewBill,
}

impl MainMenu {

    fn from_str(input: &str) -> Option<MainMenu> {
        match input {
            "1" => Some(Self::AddBill),
            "2" => Some(Self::ViewBill),
            _ => None

        }
    }

    fn display_menu() {
        println!("\n== Bill App Menu ==");
        println!("1) Add Bill");
        println!("2) View Bill");
        println!("\n Enter Selection: ");

    }

}

fn main() {

    let mut bills = Bills::new();
    
    loop {
        MainMenu::display_menu();
        let input = get_input().expect("No data entered!");
        match MainMenu::from_str(input.as_str()) {
            Some(MainMenu::AddBill) => {
                menu::add_bill(&mut bills);
            },
            Some(MainMenu::ViewBill) => {
                menu::view_bills(&bills);
            },
            None => return,
        }
    }
}