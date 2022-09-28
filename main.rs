use rand::Rng;
use std::io;

struct Entity {
    image: String,
    attack: u8,
    defence: u8,
    hp: u8,
}

fn main() {
    let enemy = Entity {
        image: String::from(r"
            #  #
               
             ##
            #  #
        "),
        attack: 3,
        defence: 2,
        hp: 12,
    };
    let player = Entity {
        image: String::from(r"
            3  3
            
            #  #
             ##
        "),
        attack: 2,
        defence: 3,
        hp: 11,
    }; 
    let mut option: u8 = 0;
    //stdin.read_line(&mut user_input).unwrap();
    println!("{}", enemy.image);
    while option == 0
    {
        option = select_option();
    }
}

fn select_option() -> u8 {
    let mut user_input = String::new();
    let stdin = io::stdin();
    println!(r"
    Choose:
    1. Attack
    2. Defened
    3. Talk
    4. Heal
    ");
    stdin.read_line(&mut user_input).unwrap();
    let option = match user_input.trim().parse::<u8>() {
        Ok(i) => i,
        Err(_e) => 0,
    };
    if 1 <= option && option <= 4
    {
        return option;    
    } 
    println!("Oops! not a vallid option! (man shut your-)");
    return 0;
}
