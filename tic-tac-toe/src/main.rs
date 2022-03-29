extern crate core;

mod board_element;

use crate::board_element::{BoardElement, GameStatus};
use std::collections::HashMap;
use std::io;

fn main() {
    // Create and initialize the tic-tac-toe board
    let mut board: HashMap<i32, BoardElement> = HashMap::new();
    for i in 1..=9 {
        board.insert(i, BoardElement::Empty);
    }

    let mut player = BoardElement::Player1;
    loop {
        draw(&board);
        // Read and validate the input number
        println!("{:?}, enter a number between 1 and 9:", player);
        let board_number = read_number(1, 9);
        if is_occupied(board_number, &board) {
            println!("The given board number is already occupied");
            continue;
        }

        board.insert(board_number, player);
        match get_game_status(&board) {
            GameStatus::OnGoing =>     player = player.opposite(),
            GameStatus::Player1Won => { draw(&board); println!("Player1 won!"); break; },
            GameStatus::Player2Won => { draw(&board); println!("Player2 won!"); break; },
            GameStatus::Tie        => { draw(&board); println!("Game tied!"); break; },
        };
    }
}

/// Draws the status of the game based on the given parameter. Prior to drawing the status
/// of the game, clears and resets the console position.
fn draw(board: &HashMap<i32, BoardElement>) {
    // Clear the terminal
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);

    println!(
        " {} | {} | {} ",
        board.get(&1).unwrap().symbol(),
        board.get(&2).unwrap().symbol(),
        board.get(&3).unwrap().symbol()
    );
    println!("---|---|---");
    println!(
        " {} | {} | {} ",
        board.get(&4).unwrap().symbol(),
        board.get(&5).unwrap().symbol(),
        board.get(&6).unwrap().symbol()
    );
    println!("---|---|---");
    println!(
        " {} | {} | {} ",
        board.get(&7).unwrap().symbol(),
        board.get(&8).unwrap().symbol(),
        board.get(&9).unwrap().symbol()
    );
}

/// Reads an input from user and returns the parsed number. If the attempt for parsing the input to
/// a number fails or if the number is out of the given boundaries, the user will be asked to
/// re-enter a valid input.
fn read_number(min: i32, max: i32) -> i32 {
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
        if number >= min && number <= max {
            return number;
        } else {
            println!("The given number must be between 1 and 9, try again");
        }
    }
}

fn is_occupied(board_number: i32, board: &HashMap<i32, BoardElement>) -> bool {
    *board.get(&board_number).unwrap() != BoardElement::Empty
}

/// Analyzes the given status of the game, and indicates if the game should continue, a player
/// has won, or the game is tie.
fn get_game_status(board: &HashMap<i32, BoardElement>) -> GameStatus {
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

    let mut empty_spot_found: bool = false;
    let mut player_map: HashMap<BoardElement, Vec<i32>> = HashMap::new();
    for (board_number, player) in board {
        if *player != BoardElement::Empty {
            let moves = player_map.entry(*player).or_insert(Vec::new());
            moves.push(*board_number);
        } else {
            empty_spot_found = true;
        }
    }

    if !empty_spot_found {
        return GameStatus::Tie;
    } else {
        for (player, moves) in player_map {
            for winner_move in &winner_moves {
                if winner_move.iter().all(|wm| moves.contains(wm)) {
                    if player == BoardElement::Player1 {
                        return GameStatus::Player1Won;
                    } else {
                        return GameStatus::Player2Won;
                    }
                }
            }
        }
        return GameStatus::OnGoing;
    }
}
