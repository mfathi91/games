#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum BoardElement {
    Player1,
    Player2,
    Empty,
}

impl BoardElement {
    pub fn symbol(&self) -> String {
        match *self {
            BoardElement::Player1 => "X".to_string(),
            BoardElement::Player2 => "O".to_string(),
            BoardElement::Empty => " ".to_string(),
        }
    }

    pub fn opposite(&self) -> BoardElement {
        match *self {
            BoardElement::Player1 => BoardElement::Player2,
            BoardElement::Player2 => BoardElement::Player1,
            _ => panic!("Unable to find the opposite"),
        }
    }
}

pub enum GameStatus {
    OnGoing,
    Player1Won,
    Player2Won,
    Tie,
}
