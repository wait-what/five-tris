use macroquad::color::Color;
use std::collections::HashMap;

mod load;
pub use load::Tile;

mod shape;
pub use shape::Shape;

type KickTable = Vec<(isize, isize)>;

pub struct PieceRules {
    pub color: Color,
    pub shape: [Shape; 4],
    pub kick_tables: [KickTable; 12],
}

pub struct Rules {
    pub pieces: HashMap<String, PieceRules>,
}

impl Rules {
    pub fn get_piece_list(&self) -> Vec<String> {
        self.pieces.keys().cloned().collect::<Vec<_>>()
    }
}
