use std::collections::HashSet;

use hexx::Hex;
use ux::u6;

#[derive(Clone, Copy, Debug)]
pub enum Player {
    Black,
    White,
}

impl Player {
    pub fn flip(&self) -> Self {
        match self {
            Player::Black => Player::White,
            Player::White => Player::Black,
        }
    }
}

pub struct Stone {
    owner: Player,
    group_id: u32,
}

pub struct Group {
    edges: u6,
    corners: u6,
    locations: HashSet<Hex>,
}
