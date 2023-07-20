use super::Piece;
use rand::prelude::*;

pub struct Queue {
    pub pieces: [Option<Piece>; 100],
    rng: ThreadRng,
}

impl Queue {
    pub fn new() -> Self {
        let mut rng = rand::thread_rng();

        let mut pieces = [None; 100];
        pieces[0..7].copy_from_slice(&Self::generate_bag(&mut rng));
        pieces[7..14].copy_from_slice(&Self::generate_bag(&mut rng));

        Self {
            pieces,
            rng,
        }
    }

    pub fn shift(&mut self) -> Piece {
        let piece = self.pieces[0];

        self.pieces.rotate_left(1);
        if self.pieces[7].is_none() {
            let bag = Self::generate_bag(&mut self.rng);
            self.pieces[7..].copy_from_slice(&bag);
        }

        piece.unwrap()
    }

    fn generate_bag(rng: &mut ThreadRng) -> [Option<Piece>; 7] {
        let mut bag = [
            Some(Piece::L),
            Some(Piece::J),
            Some(Piece::T),
            Some(Piece::O),
            Some(Piece::S),
            Some(Piece::Z),
            Some(Piece::I),
        ];

        for i in 0..bag.len() {
            let j = rng.gen_range(0..bag.len());
            bag.swap(i, j);
        }

        bag
    }
}

impl Into<String> for Queue {
    fn into(self) -> String {
        let mut string = String::new();

        for piece in self.pieces.iter() {
            match *piece {
                Some(piece) => string.push_str(piece.into()),
                None => break,
            }
        }

        string
    }
}

impl From<&str> for Queue {
    fn from(string: &str) -> Self {
        assert!(string.len() <= 100);

        let mut pieces = [None; 100];

        for (i, c) in string.chars().enumerate() {
            pieces[i] = Some(Piece::from(c.to_string().as_str()));
        }

        Self {
            pieces,
            rng: rand::thread_rng(),
        }
    }
}
