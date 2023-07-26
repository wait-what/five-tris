use super::{super::Board, Tile};

#[derive(Clone, Debug)]
pub struct Padding {
    pub left: isize,
    pub right: isize,
    pub top: isize,
    pub bottom: isize,
}

#[derive(Clone, Debug)]
pub struct Shape {
    pub shape: Vec<Vec<Tile>>,
    pub padding: Padding,
    pub width: isize,
    pub height: isize,
}

#[derive(PartialEq)]
enum Side {
    Left,
    Right,
    Top,
    Bottom,
}

impl Shape {
    fn rotate_2d_array(shape: &Vec<Vec<Tile>>) -> Vec<Vec<Tile>> {
        let mut rotated = Vec::new();
        for i in 0..shape[0].len() {
            let mut row = Vec::new();
            for j in 0..shape.len() {
                row.push(shape[j][i].clone());
            }
            rotated.push(row);
        }
        rotated
    }

    fn calculate_padding(shape: &Vec<Vec<Tile>>, side: Side) -> isize {
        let padding = match side {
            Side::Left => {
                let shape = Self::rotate_2d_array(shape);
                shape
                    .iter()
                    .position(|row| row.iter().any(|tile| tile.filled))
                    .unwrap_or(shape.len())
            }
            Side::Right => {
                let shape = Self::rotate_2d_array(shape);
                let last_row_index = shape.len();
                shape
                    .iter()
                    .rposition(|row| row.iter().any(|tile| tile.filled))
                    .map(|idx| (last_row_index - 1 - idx))
                    .unwrap_or(shape.len())
            }
            Side::Top => shape
                .iter()
                .position(|row| row.iter().any(|tile| tile.filled))
                .unwrap_or(shape.len()),
            Side::Bottom => {
                let last_row_index = shape.len();
                shape
                    .iter()
                    .rposition(|row| row.iter().any(|tile| tile.filled))
                    .map(|idx| (last_row_index - 1 - idx))
                    .unwrap_or(shape.len())
            }
        };

        padding as isize
    }

    pub fn new(shape: Vec<Vec<Tile>>) -> Self {
        let padding = Padding {
            left: Self::calculate_padding(&shape, Side::Left),
            right: Self::calculate_padding(&shape, Side::Right),
            top: Self::calculate_padding(&shape, Side::Top),
            bottom: Self::calculate_padding(&shape, Side::Bottom),
        };

        let width = shape[0].len() as isize - padding.left - padding.right;
        let height = shape.len() as isize - padding.top - padding.bottom;

        Self {
            shape,
            padding: padding,
            width: width as isize,
            height: height as isize,
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = &Vec<Tile>> {
        self.shape.iter()
    }

    pub fn collides_at(&self, piece_x: isize, piece_y: isize, board: &Board) -> bool {
        for (y, row) in self.iter().enumerate() {
            for (x, tile) in row.iter().enumerate() {
                if tile.filled {
                    let tile = board.get(
                        (x as isize + piece_x) as usize,
                        (y as isize + piece_y) as usize,
                    );

                    match tile {
                        Ok(tile) => {
                            if tile.is_some() {
                                return true;
                            }
                        }
                        Err(_) => return true,
                    }
                }
            }
        }

        false
    }
}
