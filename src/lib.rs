#![allow(dead_code, unused)]
pub mod ascii;
pub mod stone;

use hexx::{hex, Hex};
use itertools::Itertools;
use slotmap::SlotMap;
use std::collections::HashMap;
use stone::*;

type Error = Box<dyn std::error::Error>;

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

#[derive(Debug)]
pub struct Board<const RADIUS: usize> {
    pub state: HashMap<Hex, Option<Stone>>,
    to_move: Player,
    last_move: Option<Hex>,
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
            last_move: None,
            groups: SlotMap::with_key(),
            turn: 0,
        }
    }

    pub fn move_at(&mut self, input_hex: Hex) -> Result<GameState, Error> {
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
        let group_id = {
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
            let mut final_group = self.get_mut_group(group_id);

            for grp in grps_to_be_merged {
                final_group.merge(&grp);
            }

            // semi-clean up : all neighboring stones are set to the final group.
            for (h, _) in neighbor_groups {
                self.state.get_mut(&h).unwrap().as_mut().unwrap().group_id = group_id;
            }
        }

        // place the stone
        _ = self.state.get_mut(&input_hex).unwrap().insert(Stone {
            owner: self.to_move,
            group_id,
        });

        // update board and group and check wins
        self.last_move = Some(input_hex);
        let group = self.get_mut_group(group_id);

        if group.add_hex_and_check_ring(input_hex) {
            return Ok(GameState::Win(self.to_move, WinCon::Ring));
        }

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

        if group.check_bridge() {
            return Ok(GameState::Win(self.to_move, WinCon::Bridge));
        }
        if group.check_fork() {
            return Ok(GameState::Win(self.to_move, WinCon::Fork));
        }

        // AND FINALLY
        self.turn += 1;
        if self.turn >= Self::CELL_COUNT {
            Ok(GameState::Draw)
        } else {
            self.to_move = self.to_move.flip();
            Ok(GameState::Ongoing)
        }
    }

    fn get_mut_group(&mut self, group_id: GroupId) -> &mut Group {
        match self.groups.get(group_id) {
            Some(_) => self.groups.get_mut(group_id),
            None => self.groups.values_mut().find(|g| g.merged_with(&group_id)),
        }
        .expect("GroupId not found for any Group.")
    }
}

impl<const RADIUS: usize> Default for Board<RADIUS> {
    fn default() -> Self {
        Self::new()
    }
}
