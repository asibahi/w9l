use std::collections::HashSet;

use slotmap::new_key_type;
use ux::u6;

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
    edges: u6,
    corners: u6,
    merged_ids: HashSet<GroupId>,
}
impl Group {
    pub fn new(id: GroupId) -> Self {
        Self {
            edges: u6::new(0),
            corners: u6::new(0),
            merged_ids: HashSet::from([id]),
        }
    }

    pub fn merge(&mut self, other: &Self) {
        self.edges |= other.edges;
        self.corners |= other.corners;
        self.merged_ids.extend(&other.merged_ids);
    }

    pub fn merged_with(&self, id: &GroupId) -> bool {
        self.merged_ids.contains(id)
    }
}
