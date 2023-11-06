use crate::game_data::*;
use colored::Colorize;
use hexx::{Direction, Hex};
use itertools::Itertools;
use slotmap::SlotMap;
use std::{
    cell::OnceCell,
    collections::{HashMap, HashSet},
    error::Error,
    fmt::{Display, Write},
};

#[derive(Debug, Clone)]
pub struct Board<const RADIUS: usize> {
    pub state: HashMap<Hex, Option<Stone>>,
    to_move: Player,
    game_state: GameState,
    groups: SlotMap<GroupId, Group>,
    turn: usize,
}

impl<const RADIUS: usize> Board<RADIUS> {
    pub const SIZE: usize = RADIUS + 1;
    const CELL_COUNT: usize = 1 + 3 * RADIUS * (RADIUS + 1); // RedBlob

    pub fn new() -> Self {
        let state = hexx::shapes::hexagon(Hex::ZERO, RADIUS as u32)
            .map(|h| (h, None))
            .collect::<HashMap<_, _>>();

        Self {
            state,
            to_move: Player::Black,
            game_state: GameState::Ongoing,
            groups: SlotMap::with_key(),
            turn: 0,
        }
    }

    pub fn get_game_state(&self) -> GameState {
        self.game_state
    }

    pub fn move_at(&mut self, input_hex: Hex) -> Result<(), Box<dyn Error>> {
        match self.state.get(&input_hex) {
            None => return Err("Illegal Move: Hex out of bounds".into()),
            Some(Some(_)) => return Err("Illegal Move: Hex already occupied".into()),
            _ => {}
        }

        // Get surrounding groups.
        let neighbor_groups = input_hex
            .all_neighbors()
            .iter()
            .filter_map(|h| {
                self.state
                    .get(h)
                    .and_then(|c| c.as_ref())
                    .and_then(|s| (s.owner == self.to_move).then_some((*h, s.group_id)))
            })
            .unique_by(|(_, gid)| *gid) // very important
            .collect::<Vec<_>>();

        // determine group membership
        let chosen_id = {
            if neighbor_groups.is_empty() {
                // no surrounding stones.
                self.groups.insert_with_key(Group::new)
            } else {
                // one or more surrounding stones. Just pick one.
                neighbor_groups[0].1
            }
        };

        // if the surrounding stones belong to different groups (have different group ID's)
        if neighbor_groups.len() >= 2 {
            // take out the groups to be merged and remove them from the slotmap
            let grps_to_be_merged = (1..neighbor_groups.len())
                .map(|i| {
                    let (_, working_id) = neighbor_groups[i];
                    self.groups.remove(working_id).unwrap_or_else(|| {
                        let (actual_id, _) = self
                            .groups
                            .iter()
                            .find(|(_, g)| g.merged_with(&working_id))
                            .expect("GroupId not found for any Group.");

                        self.groups.remove(actual_id).unwrap()
                    })
                })
                .collect::<Vec<_>>();

            // determine the placed stone's group as the chosen one
            let final_group = self.get_mut_group(chosen_id);

            for grp in grps_to_be_merged {
                final_group.merge(&grp);
            }

            // semi-clean up : all neighboring stones are set to the final group.
            for (h, _) in neighbor_groups {
                self.state.get_mut(&h).unwrap().as_mut().unwrap().group_id = chosen_id;
            }
        }

        // place the stone
        _ = self.state.get_mut(&input_hex).unwrap().insert(Stone {
            owner: self.to_move,
            group_id: chosen_id,
        });

        self.turn += 1;
        
        // update board and group and check wins
        let group = self.get_mut_group(chosen_id);
        group.add_hex(input_hex);

        // if input_hex is corner or edge
        match input_hex.to_cubic_array().map(|c| (c / RADIUS as i32)) {
            // battlefield
            [0, 0, 0] => {}

            // goes around the board in order
            // extensive ascii testing led here

            // edges
            [0, y, 0] if y > 0 => group.add_edge(0),
            [x, 0, 0] if x < 0 => group.add_edge(1),
            [0, 0, z] if z > 0 => group.add_edge(2),
            [0, _, 0] => group.add_edge(3),
            [_, 0, 0] => group.add_edge(4),
            [0, 0, _] => group.add_edge(5),

            // corners
            [x, _, 0] if x < 0 => group.add_corner(0),
            [x, 0, _] if x < 0 => group.add_corner(1),
            [0, y, _] if y < 0 => group.add_corner(2),
            [_, _, 0] => group.add_corner(3),
            [_, 0, _] => group.add_corner(4),
            [0, _, _] => group.add_corner(5),

            _ => unreachable!("out of bounds"),
        }

        if group.check_ring() {
            self.game_state = GameState::Win(self.to_move, WinCon::Ring);
        } else if group.check_bridge() {
            self.game_state = GameState::Win(self.to_move, WinCon::Bridge);
        } else if group.check_fork() {
            self.game_state = GameState::Win(self.to_move, WinCon::Fork);
        } else if self.turn >= Self::CELL_COUNT {
            self.game_state = GameState::Draw;
        }

        self.to_move = self.to_move.flip();
        Ok(())
    }

    fn get_group(&self, group_id: GroupId) -> &Group {
        match self.groups.get(group_id) {
            Some(_) => self.groups.get(group_id),
            None => self.groups.values().find(|g| g.merged_with(&group_id)),
        }
        .expect("GroupId not found for any Group.")
    }

    fn get_mut_group(&mut self, group_id: GroupId) -> &mut Group {
        match self.groups.get(group_id) {
            Some(_) => self.groups.get_mut(group_id),
            None => self.groups.values_mut().find(|g| g.merged_with(&group_id)),
        }
        .expect("GroupId not found for any Group.")
    }
}

impl<const RADIUS: usize> Display for Board<RADIUS> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        crate::ascii::draw_game_position::<RADIUS>(&self.state, f)
    }
}

#[derive(Debug, Clone)]
pub struct Group {
    edges: u8,
    corners: u8,
    stones: Vec<Hex>,
    merged_ids: HashSet<GroupId>,
}
impl Group {
    pub fn new(id: GroupId) -> Self {
        Self {
            edges: 0,
            corners: 0,
            stones: Vec::new(),
            merged_ids: HashSet::from([id]),
        }
    }

    pub fn merge(&mut self, other: &Self) {
        self.edges |= other.edges;
        self.corners |= other.corners;
        self.stones.extend(&other.stones);
        self.merged_ids.extend(&other.merged_ids);
    }

    pub fn merged_with(&self, id: &GroupId) -> bool {
        self.merged_ids.contains(id)
    }

    pub fn add_hex(&mut self, hex: Hex) {
        self.stones.push(hex);
    }

    pub fn check_ring(&self) -> bool {
        // assumes this function is called after every move. assumption might not hold with the minimax library
        let Some(&hex) = self.stones.last() else {
            return false;
        };

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
        self.corners |= 1 << index;
    }

    pub fn add_edge(&mut self, index: usize) {
        assert!(index < 6);
        self.edges |= 1 << index;
    }

    pub fn check_bridge(&self) -> bool {
        u8::count_ones(self.corners) >= 2
    }

    pub fn check_fork(&self) -> bool {
        u8::count_ones(self.edges) >= 3
    }
}
