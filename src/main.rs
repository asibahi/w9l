// use hexx::Hex;

// use w9l::impl_1::Board;

fn main() {
    // let mut game = Board::<3>::new();

    // let _x = game.move_at(Hex { x: 0, y: 0 }).unwrap();
    // let _x = game.move_at(Hex { x: 0, y: -3 }).unwrap();
    // let _x = game.move_at(Hex { x: -1, y: 0 }).unwrap();
    // let _x = game.move_at(Hex { y: 0, x: -2 }).unwrap();
    // let _x = game.move_at(Hex { y: 1, x: -1 }).unwrap();
    // let _x = game.move_at(Hex { y: 1, x: -2 }).unwrap(); // 5
    // let _x = game.move_at(Hex { y: 1, x: 0 }).unwrap();
    // let _x = game.move_at(Hex { x: -2, y: 2 }).unwrap();
    // let _x = game.move_at(Hex { x: 1, y: 0 }).unwrap();
    // let _x = game.move_at(Hex { x: -3, y: 3 }).unwrap();
    // let _x = game.move_at(Hex { x: 1, y: -1 }).unwrap();
    // // let x = game.move_at(Hex { x: 3, y: -1 }).unwrap();
    // // let x = game.move_at(Hex { x: 0, y: -1 }).unwrap();

    // println!("{}", game);

    // let foo = game.get_game_state();

    // println!("{foo:?}");

    // //w9l::ascii::run(3).map_err(|e| {eprint!("error:{e}"); e}).ok();

    w9l::brain::run()
}
