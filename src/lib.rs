#![allow(dead_code, unused)]
pub mod ascii;
pub mod player;

use hexx::{hex, Hex};
use player::*;
use std::collections::HashMap;
use std::collections::HashSet;

type Error = Box<dyn std::error::Error>;

#[derive(Debug)]
pub struct Board {
    pub state: HashMap<Hex, Option<Player>>,

    edges: [HashSet<Hex>; 6],
    corners: [Hex; 6],

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

        let corners = std::array::from_fn(|i| hex(0, radius as i32).rotate_cw(i as u32));

        let edges = std::array::from_fn(|i| {
            let radius = radius as i32;
            (1..radius)
                .map(|j| hex(-j, radius).rotate_cw(i as u32))
                .collect()
        });

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
