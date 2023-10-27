#![allow(dead_code, unused)]
pub mod ascii;
pub mod player;

use hexx::{hex, Hex};
use player::*;
use std::collections::HashMap;
use std::collections::HashSet;

type Error = Box<dyn std::error::Error>;

#[derive(Debug)]
pub struct Board<const N: usize> {
    pub state: HashMap<Hex, Option<Player>>,
    to_move: Player,
    last_move: Option<Hex>,
}

impl<const N: usize> Board<N> {
    pub const SIZE: usize = N + 2;
    pub const RADIUS: i32 = Self::SIZE as i32 - 1;
    pub const CORNERS: [Hex; 6] = [
        hex(0, Self::RADIUS).rotate_cw(0),
        hex(0, Self::RADIUS).rotate_cw(1),
        hex(0, Self::RADIUS).rotate_cw(2),
        hex(0, Self::RADIUS).rotate_cw(3),
        hex(0, Self::RADIUS).rotate_cw(4),
        hex(0, Self::RADIUS).rotate_cw(5),
    ];

    pub const EDGES: [[Hex; N]; 6] = {
        // gimme const array::from_fn pls
        let mut outer = [[Hex::ZERO; N]; 6];
        let mut i = 0;
        while i < 6usize {
            let mut inner: [Hex; N] = [Hex::ZERO; N];
            let mut j = 0;
            while j < N {
                inner[j] = hex(-(1 + j as i32), Self::RADIUS).rotate_cw(i as u32);
                j += 1;
            }
            outer[i] = inner;
            i += 1;
        }
        outer
    };

    pub fn new() -> Self {
        let state = hexx::shapes::hexagon(Hex::ZERO, Self::RADIUS as u32)
            .map(|h| (h, None))
            .collect::<HashMap<_, _>>();

        Self {
            state,
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
