pub struct Item {
    pub name: String,
    pub power: u32,
}

impl Item {
    pub fn new(name: &str, power: u32) -> Self {
        Item {
            name: String::from(name),
            power,
        }
    }
}

