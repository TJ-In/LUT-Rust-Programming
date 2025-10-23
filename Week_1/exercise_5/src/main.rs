use rand::Rng;
use std::io;

fn main() {
    let mut player = 100.0;
    let mut enemy = 150.0;
    let mut potions = vec![25.0, 25.0, 25.0];

    while player > 0.0 && enemy > 0.0 {
        println!("| Your HP - {} | Boss HP - {} |", player, enemy);
        println!("| 1) Attack | 2) Defend | 3) Heal |");

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read user input");
        let input = input.trim();

        match input {
            "1" => receive_player_attack_dmg(&mut player, &mut enemy),
            "2" => receive_defence_multiplier(&mut player),
            "3" => consume_potion(&mut player, &mut potions),
            _ => println!("Unknown command."),
        }
    }
    if player < 0.0 && enemy < 0.0 {
        println!("Both fainted! Game over.");
    } else {
        match player > enemy {
            true => println!("Enemy boss defeated!"),
            false => println!("You have been defeated!"),
        }
    }
}

fn receive_player_attack_dmg(player: &mut f32, enemy: &mut f32) {
    let mut rng = rand::rng();
    let inflict_dmg = rng.random_range(12.5..=20.0);
    let receive_dmg = receive_boss_attack_dmg();
    *enemy -= inflict_dmg;
    *player -= receive_dmg;
    println!("Your attack deals {inflict_dmg} amount of damage.");
    println!("You take {receive_dmg} damage.");
}

fn receive_defence_multiplier(player: &mut f32) {
    let mut rng = rand::rng();
    let defense_multiplier = 1.0 / rng.random_range(2.0..=4.0);
    println!("Defense activated!");
    let receive_dmg = receive_boss_attack_dmg() * defense_multiplier;
    *player -= receive_dmg;
    println!("You take {receive_dmg} damage.");
}

fn receive_boss_attack_dmg() -> f32 {
    let mut rng = rand::rng();
    rng.random_range(5.0..=25.0)
}

fn consume_potion(player: &mut f32, potions: &mut Vec<f32>) {
    if potions.len() > 0 {
        *player += potions.pop().unwrap();
        println!("You consume a potion.");
    } else {
        println!("Out of potion!");
    }
}
