use hexx::Hex;

use w9l::board_1::{Board, GameState, WinCon};

fn main() {
    {
        let mut game = Board::<3>::new();

        let _x = game.move_at(Hex { x: 0, y: 0 }).unwrap();
        let _x = game.move_at(Hex { x: 0, y: -3 }).unwrap();
        let _x = game.move_at(Hex { x: -1, y: 0 }).unwrap();
        let _x = game.move_at(Hex { y: 0, x: -2 }).unwrap();
        let _x = game.move_at(Hex { y: 1, x: -1 }).unwrap();
        let _x = game.move_at(Hex { y: 1, x: -2 }).unwrap(); // 5
        let _x = game.move_at(Hex { y: 1, x: 0 }).unwrap();
        let _x = game.move_at(Hex { x: -2, y: 2 }).unwrap();
        let _x = game.move_at(Hex { x: 1, y: 0 }).unwrap();
        let _x = game.move_at(Hex { x: -3, y: 3 }).unwrap();
        let _x = game.move_at(Hex { x: 1, y: -1 }).unwrap();
        // let x = game.move_at(Hex { x: 3, y: -1 }).unwrap();
        // let x = game.move_at(Hex { x: 0, y: -1 }).unwrap();

        println!("{}", game);

        match _x {
            GameState::Win(_, WinCon::Bridge) => println!("bridge win"),
            GameState::Win(_, WinCon::Fork) => println!("fork win"),
            GameState::Win(_, WinCon::Ring) => println!("ring win"),
            GameState::Draw => println!("draw"),
            GameState::Ongoing => println!("ongoing"),
        }
    };
}
