pub use super::{rules::Shape, Board, Rules, Tile};
use macroquad::color::Color;

mod write;
mod movement;
mod rotation;

#[derive(Clone)]
pub struct ActivePiece {
    pub name: String,
    pub x: isize,
    pub y: isize,
    pub rotation: usize,
    pub shape: [Shape; 4],
    pub kick_tables: [Vec<(isize, isize)>; 12],
    pub color: Color,
}

impl ActivePiece {
    pub fn new(board_size: (usize, usize), piece: String, rules: &Rules) -> Self {
        let board_width = board_size.0 as isize;
        let board_height = board_size.1 as isize - (20 + 3);

        let piece_rules = rules.pieces.get(&piece).unwrap();

        let piece_width = piece_rules.shape[0].width as isize;
        let piece_height = piece_rules.shape[0].height as isize;

        let x = (board_width) / 2 - piece_width + 1; // TODO: Fix this
        let y = board_height;

        Self {
            name: piece,
            x,
            y,
            rotation: 0,
            shape: piece_rules.shape.clone(),
            kick_tables: piece_rules.kick_tables.clone(),
            color: piece_rules.color,
        }
    }
}
