use crate::{board::Board, game_data::GameState};
use hexx::Hex;
use minimax::*;

struct Game;
struct Move(i32, i32);

impl minimax::Game for Game {
    type S = Board<9>;
    type M = hexx::Hex;

    fn generate_moves(state: &Self::S, moves: &mut Vec<Self::M>) {
        moves.extend(
            state
                .state
                .iter()
                .filter_map(|(k, s)| s.is_none().then_some(k)),
        )
    }

    fn apply(state: &mut Self::S, m: Self::M) -> Option<Self::S> {
        let mut state = state.clone();
        state.move_at(m).ok();
        Some(state)
    }

    fn get_winner(state: &Self::S) -> Option<Winner> {
        match state.get_game_state() {
            crate::game_data::GameState::Win(_, _) => Some(Winner::PlayerJustMoved),
            crate::game_data::GameState::Draw => Some(Winner::Draw),
            crate::game_data::GameState::Ongoing => None,
        }
    }
}

pub fn run_() {
    let mut state = Board::new();
    perft::<Game>(&mut state, 100, true);
}

pub fn run() {
    use minimax::strategies::negamax::Negamax;
    use minimax::{Game, Strategy};
    let mut gs = GameState::Ongoing;
    while !matches!(gs, GameState::Draw) {
        let mut b = Board::new();
        let mut s = 0;
        while self::Game::get_winner(&b).is_none() {
            // println!("{}", b);
            if s == 0 {
                let mut strategy = MonteCarloTreeSearch::<crate::brain::Game>::new(
                    MCTSOptions::default().with_rollouts_before_expanding(5),
                );
                strategy.set_timeout(std::time::Duration::from_secs(1));
                // let mut strategy = Random::<crate::brain::Game>::new();

                match strategy.choose_move(&mut b) {
                    Some(m) => {
                        print!("{m:?}\t");
                        b = self::Game::apply(&mut b, m).unwrap();
                    }
                    None => break,
                };
            } else {
                // let mut strategy = MonteCarloTreeSearch::<crate::brain::Game>::new(
                //     MCTSOptions::default().with_rollouts_before_expanding(5),
                // );
                // strategy.set_timeout(Duration::from_secs(1));
                let mut strategy = Random::<crate::brain::Game>::new();
                match strategy.choose_move(&mut b) {
                    Some(m) => {
                        println!("{m:?}");
                        b = self::Game::apply(&mut b, m).unwrap();
                    }
                    None => break,
                };
            };
            s = 1 - s;
            // thread::sleep(Duration::from_secs(1));
        }

        let state = b.get_game_state();

        println!("{state:?}");
        println!("{b}");
        gs = state;
    }
}
