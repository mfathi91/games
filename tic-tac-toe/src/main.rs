extern crate core;

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
        if let Some(p) = get_winner(&status) {
            println!("{:?} won!", p);
            break;
        }
        turn = turn.opposite();
    }
}

fn draw(status: &HashMap<i32, String>) {
    // Clear the terminal
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);

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

fn get_winner(status: &HashMap<i32, String>) -> Option<Player> {
    let winner_moves = vec![
        vec![1, 2, 3],
        vec![4, 5, 6],
        vec![7, 8, 9],
        vec![1, 4, 7],
        vec![2, 5, 8],
        vec![3, 6, 9],
        vec![1, 5, 9],
        vec![3, 5, 7],
    ];

    let mut player_map: HashMap<Player, Vec<i32>> = HashMap::new();
    for (board_number, player_symbol) in status {
        if !player_symbol.eq(" ") {
            let player = Player::from_symbol(player_symbol);
            let moves = player_map.entry(player).or_insert(Vec::new());
            moves.push(*board_number);
        }
    }

    for (player, moves) in player_map {
        for winner_move in &winner_moves {
            if winner_move.iter().all(|wm| moves.contains(wm)) {
                return Option::Some(player);
            }
        }
    }
    Option::None
}
