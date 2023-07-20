use super::{ActivePiece, Piece, Board};

impl ActivePiece {
    pub fn project(&self) -> Board {
        let mut board = [[None; 10]; 40];

        board[self.y as usize][self.x as usize] = Some(self.piece);

        board
    }
}
