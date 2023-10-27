#[derive(Clone, Copy, Debug)]
pub enum Player {
    Black,
    White,
}

impl Player {
 pub   fn flip(&self) -> Self {
        match self {
            Player::Black => Player::White,
            Player::White => Player::Black,
        }
    }
}
