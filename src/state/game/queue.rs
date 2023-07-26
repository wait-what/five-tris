use rand::prelude::*;

#[derive(Debug)]
pub struct Queue {
    pub queue: Vec<String>,
    pub pieces: Vec<String>,
    rng: ThreadRng,
}

impl Queue {
    pub fn new(pieces: Vec<String>) -> Self {
        let piece_count = pieces.len();
        let mut queue = Vec::with_capacity(piece_count * 2);

        let mut rng = rand::thread_rng();
        for _ in 0..2 {
            for piece in Self::generate_bag(&pieces, &mut rng) {
                queue.push(piece);
            }
        }

        Self { queue, pieces, rng }
    }

    pub fn shift(&mut self) -> String {
        let piece = self.queue.remove(0);

        let piece_count = self.pieces.len();
        if self.queue.get(piece_count).is_none() {
            for piece in Self::generate_bag(&self.pieces, &mut self.rng) {
                self.queue.push(piece);
            }
        }

        piece
    }

    fn generate_bag(pieces: &Vec<String>, rng: &mut ThreadRng) -> Vec<String> {
        let mut bag = pieces.clone();
        let len = bag.len();

        for i in 0..(len * 2) {
            let j = rng.gen_range(0..len);
            bag.swap(i % len, j);
        }

        bag
    }

    fn clear(&mut self) {
        self.queue.clear();
    }

    fn insert(&mut self, piece: String, index: usize) {
        self.queue.insert(index, piece.clone());
    }
}
