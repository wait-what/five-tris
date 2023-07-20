use super::{ActivePiece, Piece, Board};

pub enum RotationResult {
    Normal,
    Tspin,
    Fail,
}

impl ActivePiece {
    pub fn rotate_cw(&mut self, board: &Board) -> RotationResult {
        unimplemented!()
    }

    pub fn rotate_ccw(&mut self, board: &Board) -> RotationResult {
        unimplemented!()
    }

    pub fn rotate_180(&mut self, board: &Board) -> RotationResult {
        unimplemented!()
    }
}
