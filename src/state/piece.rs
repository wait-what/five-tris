use macroquad::prelude::*;

#[derive(Copy, Clone)]
pub enum Piece {
    L,
    J,
    T,
    O,
    S,
    Z,
    I,
}

impl Into<Color> for Piece {
    fn into(self) -> Color {
        match self {
            Piece::L => color_u8!(0xff, 0x80, 0x00, 0xff),
            Piece::J => color_u8!(0x00, 0x00, 0xff, 0xff),
            Piece::T => color_u8!(0x80, 0x00, 0x80, 0xff),
            Piece::O => color_u8!(0xff, 0xff, 0x00, 0xff),
            Piece::S => color_u8!(0x00, 0xff, 0x00, 0xff),
            Piece::Z => color_u8!(0xff, 0x00, 0x00, 0xff),
            Piece::I => color_u8!(0x00, 0xff, 0xff, 0xff),
        }
    }
}

impl Into<&str> for Piece {
    fn into(self) -> &'static str {
        match self {
            Piece::L => "L",
            Piece::J => "J",
            Piece::T => "T",
            Piece::O => "O",
            Piece::S => "S",
            Piece::Z => "Z",
            Piece::I => "I",
        }
    }
}

impl From<&str> for Piece {
    fn from(s: &str) -> Self {
        match s {
            "L" => Piece::L,
            "J" => Piece::J,
            "T" => Piece::T,
            "O" => Piece::O,
            "S" => Piece::S,
            "Z" => Piece::Z,
            "I" => Piece::I,
            _ => panic!("Invalid piece"),
        }
    }
}
