use w9l::board::Board;

fn main() {
    let mut game = Board::<3>::new();

    let _x = game.move_at('d', 4).unwrap();
    let _x = game.move_at('c', 3).unwrap();
    let _x = game.move_at('b', 2).unwrap();
    let _x = game.move_at('d', 3).unwrap();
    let _x = game.move_at('c', 2).unwrap(); // 5
    let _x = game.move_at('e', 4).unwrap();
    let _x = game.move_at('d', 2).unwrap();
    let _x = game.move_at('e', 5).unwrap();
    let _x = game.move_at('d', 1).unwrap();
    let _x = game.move_at('d', 5).unwrap();
    let _x = game.move_at('a', 1).unwrap();

    println!("{}", game);

    let foo = game.get_game_state();

    println!("{foo:?}");

    // w9l::ascii::run(3).map_err(|e| {eprint!("error:{e}"); e}).ok();

    // w9l::brain::run()
}
