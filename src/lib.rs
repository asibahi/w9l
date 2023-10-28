#![allow(dead_code, unused)]
pub mod ascii;
pub mod stone;

use hexx::{hex, Hex};
use itertools::Itertools;
use slotmap::SlotMap;
use std::collections::HashMap;
use stone::*;

type Error = Box<dyn std::error::Error>;

#[derive(Debug)]
pub struct Board<const N: usize> {
    pub state: HashMap<Hex, Option<Stone>>,
    to_move: Player,
    last_move: Option<Hex>,
    groups: SlotMap<GroupId, Group>,
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
        let mut edges = [[Hex::ZERO; N]; 6];
        let mut i = 0;
        while i < 6 {
            let mut j = 0;
            while j < N {
                edges[i][j] = hex(-(1 + j as i32), Self::RADIUS).rotate_cw(i as u32);
                j += 1;
            }
            i += 1;
        }
        edges
    };

    pub fn new() -> Self {
        let state = hexx::shapes::hexagon(Hex::ZERO, Self::RADIUS as u32)
            .map(|h| (h, None))
            .collect::<HashMap<_, _>>();

        Self {
            state,
            to_move: Player::Black,
            last_move: None,
            groups: SlotMap::with_key(),
        }
    }

    pub fn move_at(&mut self, hex: Hex) -> Result<(), Error> {
        match self.state.get(&hex) {
            None => return Err("Illegal Move: Hex out of bounds".into()),
            Some(Some(_)) => return Err("Illegal Move: Hex already occupied".into()),
            _ => {}
        }

        // Get surrounding groups.
        let neighbor_groups = hex
            .all_neighbors()
            .iter()
            .filter_map(|h| {
                self.state
                    .get(h)
                    .and_then(|c| c.as_ref())
                    .and_then(|s| (s.owner == self.to_move).then(|| (*h, s.group_id)))
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
                    let working_id = neighbor_groups[i].1;
                    self.groups.remove(working_id).unwrap_or_else(|| {
                        let actual_id = self
                            .groups
                            .iter()
                            .find(|(_, g)| g.merged_with(&working_id))
                            .expect("GroupId not found for any Group.")
                            .0;

                        self.groups.remove(actual_id).unwrap()
                    })
                })
                .collect::<Vec<_>>();

            // determine the placed stone's group as the chosen one
            let final_group = self.groups.get_mut(group_id);
            let final_group = match final_group {
                // manual unwrap_or_else;
                Some(g) => g,
                None => self
                    .groups
                    .values_mut()
                    .find(|g| g.merged_with(&group_id))
                    .expect("GroupId not found for any Group."),
            };

            for grp in grps_to_be_merged {
                final_group.merge(&grp);
            }

            // semi-clean up : all neighboring stones are set to the final group.
            for (h, _) in neighbor_groups {
                self.state.get_mut(&h).unwrap().as_mut().unwrap().group_id = group_id;
            }
        }

        // place the stone
        _ = self.state.get_mut(&hex).unwrap().insert(Stone {
            owner: self.to_move,
            group_id,
        });

        // AND FINALLY
        self.last_move = Some(hex);
        self.to_move = self.to_move.flip();

        Ok(())
    }
}

impl<const N: usize> Default for Board<N> {
    fn default() -> Self {
        Self::new()
    }
}
