use std::time::Duration;

use super::{Game, Signal};

impl Game {
    pub fn update(&mut self, delta: Duration) -> Option<Signal> {
        let signal = self.input();

        // Gravity
        // TODO
        if delta.as_millis() > 500 {
            if let Some(piece) = &mut self.active_piece {
                piece.drop(&self.board);
            };
        };

        signal
    }
}
