use super::{ActivePiece, Board};

impl ActivePiece {
    pub fn left(&mut self, board: &Board) -> Result<(), ()> {
        let shape = &self.shape[self.rotation];

        if self.x + shape.padding.left <= 0 {
            return Err(());
        } else {
            if !shape.collides_at(self.x - 1, self.y, board) {
                self.x -= 1;
                return Ok(());
            }

            return Err(());
        }
    }

    pub fn right(&mut self, board: &Board) -> Result<(), ()> {
        let shape = &self.shape[self.rotation];

        if self.x + shape.width + shape.padding.left >= board.size().0 as isize {
            return Err(());
        } else {
            if !shape.collides_at(self.x + 1, self.y, board) {
                self.x += 1;
                return Ok(());
            }

            return Err(());
        }
    }

    pub fn drop(&mut self, board: &Board) -> Result<(), ()> {
        let shape = &self.shape[self.rotation];

        if self.y + shape.height + shape.padding.top >= board.size().1 as isize {
            return Err(());
        } else {
            if !shape.collides_at(self.x, self.y + 1, board) {
                self.y += 1;
                return Ok(());
            }

            return Err(());
        }
    }
}
