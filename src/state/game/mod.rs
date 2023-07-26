mod board;
pub use board::Board;

mod queue;
use queue::Queue;

mod active_piece;
pub use active_piece::ActivePiece;

mod rules;
pub use rules::{Rules, Tile};

mod input;
pub use input::Signal;

mod update;
mod view;

pub struct Game {
    pub board: Board,
    pub queue: Queue,
    pub rules: Rules,
    pub active_piece: Option<ActivePiece>,
    pub hold_piece: Option<String>,
}

impl Game {
    pub fn new(width: usize, height: usize) -> Self {
        let rules = Rules::from_json(include_str!("./rules.jsonc"));
        let mut queue = Queue::new(rules.get_piece_list());

        let active_piece = ActivePiece::new((width, height + 20), queue.shift(), &rules);

        Self {
            board: Board::new(width, height),
            rules,
            active_piece: Some(active_piece),
            queue,
            hold_piece: None,
        }
    }
}
