use crate::game_data::*;
use hexx::Hex;
use std::collections::HashMap;

// print the boards in impl_1 and impl_2
pub fn draw_game_position<const RADIUS: usize>(
    havannah_board: &HashMap<Hex, Option<Stone>>,
    f: &mut std::fmt::Formatter<'_>,
) -> std::fmt::Result {
    // starting hex
    let mut tracker = Hex::new(RADIUS as i32, 0); // [radius, 0, -radius]

    let fill = |h| match havannah_board[&h] {
        Some(Stone {
            owner: Player::Black,
            ..
        }) => "b",
        Some(Stone {
            owner: Player::White,
            ..
        }) => "w",
        None => "",
    };

    // directions
    let bt_lft = Hex::new(-1, 1);
    let bottom = Hex::new(-1, 0);
    let bt_rgt = Hex::new(0, -1);
    let dia_rgt = Hex::new(1, -2);

    writeln!(f, "{:<pad$}__", "", pad = 3 * RADIUS + 9)?;

    for i in (0..RADIUS).rev() {
        let mut cursor = tracker;
        write!(f, "{:<pad$}__/", "", pad = 3 * i + 9)?;
        // inner cells
        for _ in 0..(RADIUS - i - 1) {
            write!(f, "{:>2}\\__/", fill(cursor))?;
            cursor += dia_rgt;
        }

        writeln!(f, "{:>2}\\__", fill(cursor))?;
        tracker += bt_lft;
    }

    for i in 0..=RADIUS {
        // top line
        let mut cursor = tracker;
        write!(f, "{:8}/", "")?;
        for _ in 0..RADIUS {
            write!(f, "{:>2}\\__/", fill(cursor))?;
            cursor += dia_rgt;
        }
        writeln!(f, "{:>2}\\", fill(cursor))?;

        // bottom line
        write!(
            f,
            "{:>8}",
            ((RADIUS as i32 - tracker.z()) as u8 + b'a') as char
        )?;
        cursor = tracker + bt_rgt;
        for _ in 0..RADIUS {
            write!(f, "\\__/{:>2}", fill(cursor))?;
            cursor += dia_rgt;
        }
        writeln!(f, "\\__/{}", cursor.x() + RADIUS as i32 + 1)?;
        tracker += bottom;
    }

    tracker += dia_rgt;

    for i in 0..RADIUS {
        let mut cursor = tracker;
        write!(
            f,
            "{:>pad$}",
            ((RADIUS as i32 + 1 - tracker.z()) as u8 + b'a') as char,
            pad = 3 * i + 11
        )?;

        for _ in 0..(RADIUS - i - 1) {
            write!(f, "\\__/{:>2}", fill(cursor))?;
            cursor += dia_rgt;
        }
        writeln!(f, "\\__/{}", cursor.x() + RADIUS as i32 + 1)?;
        tracker += bt_rgt;
    }

    Ok(())
}
