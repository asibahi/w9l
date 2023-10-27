#![allow(dead_code, unused)]

use hexx::Hex;
use std::collections::HashMap;
use std::collections::HashSet;

pub mod ascii;

type Error = Box<dyn std::error::Error>;

#[derive(Clone, Copy, Debug)]
pub enum Player {
    Black,
    White,
}

impl Player {
    fn flip(&self) -> Self {
        match self {
            Player::Black => Player::White,
            Player::White => Player::Black,
        }
    }
}

#[derive(Debug)]
pub struct Board {
    pub state: HashMap<Hex, Option<Player>>,

    edges: HashSet<Hex>,
    corners: HashSet<Hex>,

    size: u32,
    to_move: Player,
    last_move: Option<Hex>,
}

impl Board {
    pub fn new(size: u32) -> Self {
        let radius = size - 1;
        let state = hexx::shapes::hexagon(Hex::ZERO, radius)
            .map(|h| (h, None))
            .collect::<HashMap<_, _>>();

        let edges = state
            .keys()
            .filter(|h| {
                (h.x.unsigned_abs() == radius)
                    ^ (h.y.unsigned_abs() == radius)
                    ^ (h.z().unsigned_abs() == radius)
            })
            .copied()
            .collect::<HashSet<_>>();

        let corners = state
            .keys()
            .filter(|h| {
                ((h.x.unsigned_abs() == radius || h.x == 0)
                    && (h.y.unsigned_abs() == radius || h.y == 0))
                    ^ (h.x == 0 && h.y == 0)
            })
            .copied()
            .collect::<HashSet<_>>();

        Self {
            state,
            edges,
            corners,
            size,
            to_move: Player::Black,
            last_move: None,
        }
    }

    pub fn move_at(&mut self, hex: Hex) -> Result<(), Error> {
        let Some(content) = self.state.get_mut(&hex) else {
            return Err("Illegal Move: Location out of bounds".into());
        };

        if content.is_some() {
            return Err("Illegal Move: Location already occupied".into());
        }

        _ = content.insert(self.to_move);
        self.last_move = Some(hex);
        self.to_move = self.to_move.flip();

        Ok(())
    }
}
