pub use super::{Piece, Board};

mod rotation;
mod projection;
mod movement;

pub struct ActivePiece {
    pub piece: Piece,
    pub x: i32,
    pub y: i32,
    pub rotation: i32,
}

impl From<Piece> for ActivePiece {
    fn from(piece: Piece) -> Self {
        Self {
            piece,
            x: 0,
            y: 0,
            rotation: 0,
        }
    }
}
