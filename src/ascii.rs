use colored::Colorize;
use hexx::Hex;
use itertools::Itertools;
use std::{
    cell::OnceCell,
    collections::HashMap,
    error::Error,
    fmt::{Display, Write},
};

use crate::{stone::{Stone, Player}, Board, WinCon};

pub fn run() {
    let mut game = Board::<3>::new();

    let x = game.move_at(Hex { x: 0, y: 0 }).unwrap();
    let x = game.move_at(Hex { x: 0, y: -3 }).unwrap();
    let x = game.move_at(Hex { x: -1, y: 0 }).unwrap();
    let x = game.move_at(Hex { y: 0, x: -2 }).unwrap();
    let x = game.move_at(Hex { y: 1, x: -1 }).unwrap();
    let x = game.move_at(Hex { y: 1, x: -2 }).unwrap(); // 5
    let x = game.move_at(Hex { y: 1, x: 0 }).unwrap();
    let x = game.move_at(Hex { x: -2, y: 2 }).unwrap();
    let x = game.move_at(Hex { x: 1, y: 0 }).unwrap();
    let x = game.move_at(Hex { x: -3, y: 3 }).unwrap();
    let x = game.move_at(Hex { x: 1, y: -1 }).unwrap();
    // let x = game.move_at(Hex { x: 3, y: -1 }).unwrap();
    // let x = game.move_at(Hex { x: 0, y: -1 }).unwrap();

    println!("{}", game);

    match x {
        crate::GameState::Win(_, WinCon::Bridge) => println!("bridge win"),
        crate::GameState::Win(_, WinCon::Fork) => println!("fork win"),
        crate::GameState::Win(_, WinCon::Ring) => println!("ring win"),
        crate::GameState::Draw => println!("draw"),
        crate::GameState::Ongoing => println!("ongoing"),
    }
}

fn draw_game_position(
    havannah_board: &HashMap<Hex, Option<Stone>>,
    f: &mut std::fmt::Formatter<'_>,
) -> std::fmt::Result {
    let rows = havannah_board
        .into_iter()
        .sorted_by_key(|(h, _)| (h.x, h.y))
        //.sorted_by_key(|(h, _)| h.x)
        .group_by(|(h, _)| h.x);
    let radius = OnceCell::new();

    let mut top_jags = String::from("\n");
    let mut values = String::new();
    let mut bot_jags = String::new();

    let mut file_idxs = String::new();

    for (rank, row) in &rows {
        let radius = radius.get_or_init(|| rank.abs());
        let v_o = 2 * rank.unsigned_abs() as usize;
        let j_o = 7 + v_o;

        write!(top_jags, "{:>j_o$}", "")?;
        write!(
            values,
            "{:>v_o$}{:>5} ",
            "",
            // (radius - rank + 1).to_string().blue()
            rank.to_string().blue()
        )?;
        write!(bot_jags, "{:>j_o$}", "")?;

        for (_, content) in row {
            let fill = match content {
                Some(s) => match s.owner {
                    Player::Black => "b",
                    Player::White => "w",
                },
                None => "",
            };

            if rank <= 0 {
                write!(top_jags, "{} \\ ", "/".purple())?;
            }
            write!(values, "{}{:>2} ", "|".blue(), fill)?;
            if rank >= 0 {
                write!(bot_jags, "\\ {} ", "/".purple())?;
            }
        }

        write!(values, "{}", "|".blue())?;
        if rank > 0 {
            let file = (b'a' as i32 + 2 * radius - rank + 1) as u8 as char;
            write!(values, " {}", (radius - rank + 1).to_string().purple())?;
            // write!(values, " {}", file.to_string().purple())?;
        } else {
            let file = (rank + radius + b'a' as i32) as u8 as char;
            write!(file_idxs, "{:>4}", rank)?;
            // write!(file_idxs, "{:>4}", file)?;
        }

        if rank == *radius {
            writeln!(bot_jags, "\n{:>j_o$}{}", "", file_idxs.purple())?;
        }

        if rank <= 0 {
            writeln!(f, "{}", top_jags);
        }
        writeln!(f, "{}", values);
        if rank >= 0 {
            writeln!(f, "{}", bot_jags);
        }

        top_jags.clear();
        values.clear();
        bot_jags.clear();
    }

    Ok(())
}

impl<const N: usize> Display for crate::Board<N> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        draw_game_position(&self.state, f)
    }
}
