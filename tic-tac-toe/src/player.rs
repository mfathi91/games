use crate::Player::{Player1, Player2};

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum Player {
    Player1,
    Player2,
}

impl Player {
    pub fn symbol(&self) -> String {
        match *self {
            Player::Player1 => "X".to_string(),
            Player::Player2 => "O".to_string(),
        }
    }

    pub fn opposite(&self) -> Player {
        match *self {
            Player::Player1 => Player::Player2,
            Player::Player2 => Player::Player1,
        }
    }

    pub fn from_symbol(symbol: &str) -> Player {
        match symbol {
            "X" => Player1,
            "O" => Player2,
            _ => panic!("Unknown symbol"),
        }
    }
}
