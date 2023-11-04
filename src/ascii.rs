use crate::game_data::*;
use colored::Colorize;
use hexx::Hex;
use itertools::Itertools;
use std::{
    cell::OnceCell,
    collections::{HashMap, HashSet},
    error::Error,
    fmt::Write,
};

pub fn run(size: u32) -> Result<(), Box<dyn Error>> {
    // let havannah_board = get_board(size);
    // draw_board_pointy(havannah_board)
    draw_board_flat(size)
}

fn get_board(size: u32) -> impl ExactSizeIterator<Item = hexx::Hex> {
    hexx::shapes::hexagon(hexx::Hex::ZERO, size - 1)
}

fn draw_board_pointy(
    havannah_board: impl ExactSizeIterator<Item = hexx::Hex>,
) -> Result<(), Box<dyn Error>> {
    let rows = havannah_board.group_by(|h| h.x);
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
            (radius - rank + 1).to_string().blue()
        )?;
        write!(bot_jags, "{:>j_o$}", "")?;

        for hex in row {
            let fill = match hex.to_cubic_array().map(|c| (c / radius)) {
                // battlefield
                [0, 0, 0] => "",

                // edges
                [0, y, 0] if y > 0 => "e0",
                [x, 0, 0] if x < 0 => "e1",
                [0, 0, z] if z > 0 => "e2",
                [0, _, 0] => "e3",
                [_, 0, 0] => "e4",
                [0, 0, _] => "e5",

                // corners
                [x, _, 0] if x < 0 => "c",
                [x, 0, _] if x < 0 => "c",
                [0, y, _] if y < 0 => "c",
                [_, _, 0] => "c",
                [_, 0, _] => "c",
                [0, _, _] => "c",

                // else
                _ => unreachable!("out of bounds"),
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
            write!(values, " {}", file.to_string().purple())?;
        } else {
            let file = (rank + radius + b'a' as i32) as u8 as char;
            write!(file_idxs, "{:>4}", file)?;
        }

        if rank == *radius {
            writeln!(bot_jags, "\n{:>j_o$}{}", "", file_idxs.purple())?;
        }

        if rank <= 0 {
            println!("{}", top_jags);
        }
        println!("{}", values);
        if rank >= 0 {
            println!("{}", bot_jags);
        }

        top_jags.clear();
        values.clear();
        bot_jags.clear();
    }

    Ok(())
}

fn draw_board_flat(radius: u32) -> Result<(), Box<dyn Error>> {
    let board = hexx::shapes::hexagon(hexx::Hex::ZERO, radius)
        .enumerate()
        .map(|(i, x)| (x, i % 100))
        .collect::<HashMap<_, _>>();

    // starting hex [x, y, z]
    let mut tracker = Hex::new(radius as i32, 0); // [radius, 0, -radius]

    let fill = |h| board[&h];

    // directions
    let bt_lft = Hex::new(-1, 1);
    let bottom = Hex::new(-1, 0);
    let bt_rgt = Hex::new(0, -1);
    let dia_rgt = Hex::new(1, -2);

    // buffer
    let mut f = String::new();

    let radius = radius as usize;

    writeln!(f, "{:<pad$}__", "", pad = 3 * radius + 1)?;

    for i in (0..radius).rev() {
        let mut cursor = tracker;
        write!(f, "{:<pad$}__/", "", pad = 3 * i + 1)?;
        // inner cells
        for _ in 0..(radius - i - 1) {
            write!(f, "{:>2}\\__/", fill(cursor))?;
            cursor += dia_rgt;
        }

        writeln!(f, "{:>2}\\__", fill(cursor))?;
        tracker += bt_lft;
    }

    for i in 0..=radius {
        // top line
        let mut cursor = tracker;
        write!(f, "/")?;
        for _ in 0..radius {
            write!(f, "{:>2}\\__/", fill(cursor))?;
            cursor += dia_rgt;
        }
        writeln!(f, "{:>2}\\", fill(cursor))?;

        // bottom line
        cursor = tracker + bt_rgt;
        for _ in 0..radius {
            write!(f, "\\__/{:>2}", fill(cursor))?;
            cursor += dia_rgt;
        }
        writeln!(f, "\\__/")?;
        tracker += bottom;
    }

    tracker += dia_rgt;

    for i in 0..radius {
        let mut cursor = tracker;
        write!(f, "{:>pad$}", "", pad = 3 * i + 3)?;

        for _ in 0..(radius - i - 1) {
            write!(f, "\\__/{:>2}", fill(cursor))?;
            cursor += dia_rgt;
        }
        writeln!(f, "\\__/")?;
        tracker += bt_rgt;
    }

    println!("{}", f);
    Ok(())
}

// print the boards in impl_1 and impl_2
pub fn draw_game_position(
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
            let _file = (b'a' as i32 + 2 * radius - rank + 1) as u8 as char;
            write!(values, " {}", (radius - rank + 1).to_string().purple())?;
            // write!(values, " {}", file.to_string().purple())?;
        } else {
            let _file = (rank + radius + b'a' as i32) as u8 as char;
            write!(file_idxs, "{:>4}", rank)?;
            // write!(file_idxs, "{:>4}", file)?;
        }

        if rank == *radius {
            writeln!(bot_jags, "\n{:>j_o$}{}", "", file_idxs.purple())?;
        }

        if rank <= 0 {
            writeln!(f, "{}", top_jags)?;
        }
        writeln!(f, "{}", values)?;
        if rank >= 0 {
            writeln!(f, "{}", bot_jags)?;
        }

        top_jags.clear();
        values.clear();
        bot_jags.clear();
    }

    Ok(())
}
