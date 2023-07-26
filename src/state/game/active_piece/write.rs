use super::{ActivePiece, Board};

impl ActivePiece {
    pub fn write(&mut self, board: &mut Board) -> Vec<usize> {
        let shape = &self.shape[self.rotation];

        // Write piece
        for (y, row) in shape.iter().enumerate() {
            for (x, tile) in row.iter().enumerate() {
                if tile.filled {
                    board.set(
                        (x as isize + self.x) as usize,
                        (y as isize + self.y) as usize,
                        self.color,
                    );
                }
            }
        }

        // Clear lines
        let mut cleared_lines = Vec::new();
        for (y, row) in board.iter().enumerate() {
            if row.iter().all(|tile| tile.is_some()) {
                cleared_lines.push(y);
            }
        }

        for y in cleared_lines.iter() {
            board.clear_row(*y);
        }

        cleared_lines
    }
}
