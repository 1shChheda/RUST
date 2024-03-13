use crate::player::Player;
use crate::item::Item;

pub fn start() {
    println!("\n\nWelcome to the Rust Adventure Game!");

    let mut player1 = Player::new("Vansh");
    println!("Player1 `{}` created with health {}", player1.name, player1.health);

    let mut player2 = Player::new("Darshil");
    println!("Player2 `{}` created with health {}", player2.name, player2.health);

    let sword = Item::new("Sword", 20);
    player1.add_item(sword);
    println!("Player `{}` found a {} with power {}", player1.name, player1.item.name, player1.item.power);

    // println!("Player `` found a {} with power {}", sword.name, sword.power);


    // player1.take_damage(player2, player1.item.power);
    Player::take_damage(&mut player2, player1.item.power);
    println!("`{}` attacked `{}` with {}.", player1.name, player2.name, player1.item.name);
    println!("{}'s health: {}\t{}'s health: {}", player2.name, player2.health, player1.name, player1.health);

}
