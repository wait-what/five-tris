use macroquad::color::Color;

#[derive(Copy, Clone, Debug)]
pub struct BoardTile {
    pub color: Color,
}

pub struct Board(Vec<Vec<Option<BoardTile>>>);

impl Board {
    pub fn new(width: usize, height: usize) -> Self {
        Self(vec![vec![None; width]; height + 20])
    }

    pub fn size(&self) -> (usize, usize) {
        (self.0[0].len(), self.0.len())
    }

    pub fn set(&mut self, x: usize, y: usize, color: Color) {
        self.0[y][x] = Some(BoardTile { color });
    }

    pub fn get(&self, x: usize, y: usize) -> Result<Option<BoardTile>, ()> {
        match self.0.get(y) {
            Some(row) => match row.get(x) {
                Some(tile) => Ok(*tile),
                None => Err(()),
            },
            None => Err(()),
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = &Vec<Option<BoardTile>>> {
        self.0.iter()
    }

    pub fn clear_row(&mut self, row: usize) {
        self.0.remove(row);
        self.0.insert(0, vec![None; self.0[0].len()]);
    }
}
