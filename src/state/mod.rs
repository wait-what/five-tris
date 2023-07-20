pub type Board = [[Option<Piece>; 10]; 40];

mod queue;
use queue::Queue;

mod piece;
pub use piece::Piece;

mod active_piece;
pub use active_piece::ActivePiece;

mod input;
mod view;
mod update;

pub struct State {
    pub board: Board,
    pub queue: Queue,
    pub active_piece: Option<ActivePiece>,
    pub hold_piece: Option<Piece>,
}

impl State {
    pub fn new() -> Self {
        let mut queue = Queue::new();

        Self {
            board: [[None; 10]; 40],
            active_piece: Some(queue.shift().into()),
            queue,
            hold_piece: None,
        }
    }
}
