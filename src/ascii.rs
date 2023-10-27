use colored::Colorize;
use itertools::Itertools;
use std::{cell::OnceCell, error::Error, fmt::Write};

pub fn run(size: u32) -> Result<(), Box<dyn Error>> {
    let havannah_board = get_board(size);

    draw_board(havannah_board)
}

fn get_board(size: u32) -> impl ExactSizeIterator<Item = hexx::Hex> {
    hexx::shapes::hexagon(hexx::Hex::ZERO, size - 1)
}

fn draw_board(
    havannah_board: impl ExactSizeIterator<Item = hexx::Hex>,
) -> Result<(), Box<dyn Error>> {
    let rows = havannah_board.group_by(|h| h.x);
    let radius = OnceCell::new();

    let mut top_jags = String::from("\n");
    let mut values = String::new();
    let mut bot_jags = String::new();

    let mut col_idxs = String::new();

    for (rank, row) in &rows {
        let radius = radius.get_or_init(|| rank.abs());
        let v_o = 2 * rank.unsigned_abs() as usize;
        let j_o = 7 + v_o;

        write!(top_jags, "{:>j_o$}", "")?;
        write!(values, "{:>v_o$}{:>5} ", "", rank.to_string().blue())?;
        write!(bot_jags, "{:>j_o$}", "")?;

        for h in row {
            let is_corner = ((h.x.abs() == *radius || h.x == 0)
                && (h.y.abs() == *radius || h.y == 0))
                ^ (h.x == 0 && h.y == 0);

            let is_edge =
                (h.x.abs() == *radius) ^ (h.y.abs() == *radius) ^ (h.z().abs() == *radius);

            let fill = if is_edge {
                "E"
            } else if is_corner {
                "C"
            } else {
                ""
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
            write!(values, " {}", (*radius + 1 - rank).to_string().purple())?;
        } else {
            write!(col_idxs, "{:>4}", rank)?;
        }

        if rank == *radius {
            writeln!(bot_jags, "\n{:>j_o$}{}", "", col_idxs.purple())?;
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
