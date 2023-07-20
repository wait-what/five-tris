use std::time::Duration;

use super::State;

impl State {
    pub fn update(&mut self, delta: Duration) {
        // Input
        self.input();

        // Gravity
        if delta.as_millis() > 500 {
            if let Some(piece) = &mut self.active_piece {
                piece.soft_drop(&self.board);
            };
        }
    }
}
