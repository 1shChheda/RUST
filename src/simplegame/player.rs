use crate::item::Item; // IMP: here "crate" refers to game.rs file, since we've mentioned the file hierarchy in Cargo.toml

pub struct Player {
    pub name: String,
    pub health: u32,
    pub item: Item,
}

impl Player {
    pub fn new(name: &str) -> Self {
        Player {
            name: String::from(name),
            health: 100,
            item: Item::new("None", 0),
        }
    }

    pub fn add_item(&mut self, item: Item) {
        self.item = item;
        // println!("Player found a {} with power {}", self.item.name, self.item.power);
    }

    pub fn take_damage(&mut self, damage: u32) {
        self.health -= damage;
        // println!("{} took {} damage!", self.name, damage);
    }
    
}