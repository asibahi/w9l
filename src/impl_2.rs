use crate::game_data::*;
use hexx::{hex, Direction, Hex};
use itertools::Either::{self, Left, Right};
use slotmap::SlotMap;
use std::{collections::HashMap, fmt::Display};

type Error = Box<dyn std::error::Error>;

#[derive(Debug)]
pub struct Board<const RADIUS: usize> {
    pub state: HashMap<Hex, Option<Stone>>,
    to_move: Player,
    last_move: Option<Hex>,
    groups: SlotMap<GroupId, Either<Group, GroupId>>,
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
        let neighbors = input_hex.all_neighbors();
        let mut neighbor_friendlies = neighbors.iter().filter_map(|h| {
            let stone = self.state.get(h)?.as_ref()?;
            (stone.owner == self.to_move).then_some(stone.group_id)
        });
        // .unique(); // is it important?

        // determine group membership
        let chosen_id = {
            match neighbor_friendlies.next() {
                // no surrounding stones.
                None => self.groups.insert(Left(Group::new())),
                // one or more surrounding stones. Just pick one.
                Some(gid) => self.get_true_id(gid),
            }
        };

        // only runs if there is more than 1 group.
        for working_id in neighbor_friendlies {
            let working_id = self.get_true_id(working_id);

            // find the two groups
            let Some([chosen_group, working_group]) = self
                .groups
                .get_disjoint_mut([chosen_id, working_id])
                .map(|gs| gs.map(|g| g.as_mut().unwrap_left()))
            else {
                // if they're the same group get_disjoin is None
                continue;
            };

            // merge
            chosen_group.merge(&working_group);
            self.groups[working_id] = Right(chosen_id);
        }

        // update neighboring hex's group_id
        for hex in neighbors {
            match self.state.get_mut(&hex) {
                Some(Some(s)) if s.owner == self.to_move => s.group_id = chosen_id,
                _ => {}
            }
        }

        // place the stone
        _ = self.state.get_mut(&input_hex).unwrap().insert(Stone {
            owner: self.to_move,
            group_id: chosen_id,
        });

        // update board and group and check wins
        self.last_move = Some(input_hex);
        let group = self.get_mut_group(chosen_id);

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
        match &self.groups[group_id] {
            Right(gid) => self.get_mut_group(*gid),
            _ => self.groups[group_id].as_mut().unwrap_left(),
        }
    }

    fn get_true_id(&self, working_id: GroupId) -> GroupId {
        match self.groups[working_id] {
            Left(_) => working_id,
            Right(gid) => self.get_true_id(gid),
        }
    }
}


impl<const RADIUS: usize> Display for Board<RADIUS> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        crate::ascii::draw_game_position::<RADIUS>(&self.state, f)
    }
}

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
