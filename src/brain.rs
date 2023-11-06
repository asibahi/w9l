use crate::impl_2::Board;
use minimax::*;

struct Game;
struct Move(i32, i32);

impl minimax::Game for Game {
    type S = Board<7>;
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
        state.move_at(m).ok();
        None
    }

    fn undo(state: &mut Self::S, m: Self::M) {
        let mut foo = state.state[&m];
        foo = None;
    }

    fn get_winner(state: &Self::S) -> Option<Winner> {
        match state.get_game_state() {
            crate::game_data::GameState::Win(_, _) => Some(Winner::PlayerJustMoved),
            crate::game_data::GameState::Draw => Some(Winner::Draw),
            crate::game_data::GameState::Ongoing => None,
        }
    }
}

pub fn run() {
    use minimax::strategies::negamax::Negamax;
    use minimax::{Game, Strategy};

    let mut b = Board::new();

    let mut s = 0;
    while self::Game::get_winner(&b).is_none() {
        println!("{}", b);
        if s == 0 {
            let mut strategy =
                MonteCarloTreeSearch::<crate::brain::Game>::new(MCTSOptions::default());

            match strategy.choose_move(&mut b) {
                Some(m) => self::Game::apply(&mut b, m),
                None => break,
            };
        } else {
            let mut strategy = Random::<crate::brain::Game>::new();
            match strategy.choose_move(&mut b) {
                Some(m) => self::Game::apply(&mut b, m),
                None => break,
            };
        };
        s = 1 - s;
    }
    println!("{}", b);
}
