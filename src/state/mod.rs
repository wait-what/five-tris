pub mod game;
use game::Game;

pub struct State {
    pub game: Game,
}

impl State {
    pub fn new() -> Self {
        Self {
            game: Game::new(10, 20),
        }
    }
}
