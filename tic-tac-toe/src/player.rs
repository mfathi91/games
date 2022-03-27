#[derive(Debug, PartialEq)]
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
}
