mod player;

use crate::player::Player;
use std::collections::HashMap;
use std::io;

fn main() {
    let mut status: HashMap<i32, String> = HashMap::new();
    let mut turn = Player::Player1;
    loop {
        println!("{:?}, enter a number between 1 and 9:", turn);
        let input = read_number();
        status.insert(input, turn.symbol());
        draw(&status);

        turn = new_turn(turn);
    }
}

fn draw(status: &HashMap<i32, String>) {
    println!(
        " {} | {} | {} ",
        unwrap_or_empty(status.get(&1)),
        unwrap_or_empty(status.get(&2)),
        unwrap_or_empty(status.get(&3))
    );
    println!("---|---|---");
    println!(
        " {} | {} | {} ",
        unwrap_or_empty(status.get(&4)),
        unwrap_or_empty(status.get(&5)),
        unwrap_or_empty(status.get(&6))
    );
    println!("---|---|---");
    println!(
        " {} | {} | {} ",
        unwrap_or_empty(status.get(&7)),
        unwrap_or_empty(status.get(&8)),
        unwrap_or_empty(status.get(&9))
    );
}

fn unwrap_or_empty(position: Option<&String>) -> String {
    match position {
        Some(symbol) => symbol.to_string(),
        None => " ".to_string(),
    }
}

fn read_number() -> i32 {
    loop {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Unable to read from input");
        let number = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid number supplied, try again");
                continue;
            }
        };
        if number >= 1 && number <= 9 {
            return number;
        } else {
            println!("The given number must be between 1 and 9, try again");
        }
    }
}

fn new_turn(turn: Player) -> Player {
    if turn == Player::Player1 {
        Player::Player2
    } else {
        Player::Player1
    }
}
