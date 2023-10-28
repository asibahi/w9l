use std::collections::HashSet;

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
    merged_ids: HashSet<GroupId>,
}
impl Group {
    pub fn new(id: GroupId) -> Self {
        Self {
            edges: 0,
            corners: 0,
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

    pub fn add_corner(&mut self, index: usize) {
        assert!(index < 6);
        self.corners |= (1 << index)
    }

    pub fn add_edge(&mut self, index: usize) {
        assert!(index < 6);
        self.edges |= (1 << index)
    }

    pub fn is_bridge(&self) -> bool {
        u8::count_ones(self.corners) >= 2
    }

    pub fn is_fork(&self) -> bool {
        u8::count_ones(self.edges) >= 3
    }
}
