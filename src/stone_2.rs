use hexx::{Direction, Hex};
use itertools::Itertools;
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

#[derive(Debug)]
pub struct Group {
    edges: u8,
    corners: u8,
    stones: Vec<Hex>,
}
impl Group {
    pub fn new() -> Self {
        Self {
            edges: 0,
            corners: 0,
            stones: Vec::new(),
        }
    }

    pub fn merge(&mut self, other: &Self) {
        self.edges |= other.edges;
        self.corners |= other.corners;
        self.stones.extend(&other.stones);
    }

    pub fn add_hex_and_check_ring(&mut self, hex: Hex) -> bool {
        self.stones.push(hex);

        // algo from http://havannah.ewalds.ca/static/thesis.pdf
        // No ring for smaller groups
        self.stones.len() >= 6
        // new stone is connected to at least two stones in the same group.
            && self
                .stones
                .iter()
                .filter(|h| h.neighbor_direction(hex).is_some())
                .count()
                >= 2
        // Search for self. 
            && Direction::ALL_DIRECTIONS[..4]
                .into_iter()
                .any(|&dir| self.search_dir(hex, dir, hex))
    }

    fn search_dir(&self, current: Hex, dir: Direction, target: Hex) -> bool {
        let current = current + dir;
        current == target
            || (self.stones.contains(&current)
                && (self.search_dir(current, dir.clockwise(), target)
                    || self.search_dir(current, dir, target)
                    || self.search_dir(current, dir.counter_clockwise(), target)))
    }

    pub fn add_corner(&mut self, index: usize) {
        assert!(index < 6);
        self.corners |= (1 << index);
    }

    pub fn add_edge(&mut self, index: usize) {
        assert!(index < 6);
        self.edges |= (1 << index);
    }

    pub fn check_bridge(&self) -> bool {
        u8::count_ones(self.corners) >= 2
    }

    pub fn check_fork(&self) -> bool {
        u8::count_ones(self.edges) >= 3
    }
}