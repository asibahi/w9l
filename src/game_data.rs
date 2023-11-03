use std::collections::HashSet;

use hexx::{Direction, Hex};
use slotmap::new_key_type;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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

#[derive(Clone, Copy, Debug)]
pub struct Stone {
    pub owner: Player,
    pub group_id: GroupId,
}

new_key_type! { pub struct GroupId; }

pub enum GameState {
    Win(Player, WinCon),
    Draw,
    Ongoing,
}

pub enum WinCon {
    Bridge, // connect two corners
    Fork,   // connect three edges
    Ring,   // encricle a cell
}
