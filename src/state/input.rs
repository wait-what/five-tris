use super::State;
use macroquad::prelude::*;

impl State {
    pub fn input(&mut self) {
        if let Some(piece) = &mut self.active_piece {
            // Movement
            if is_key_pressed(KeyCode::Kp4) { // Left
                piece.left(&self.board);
            }
            if is_key_pressed(KeyCode::Kp6) { // Right
                piece.right(&self.board);
            }
            if is_key_pressed(KeyCode::Kp5) { // Soft drop
                piece.soft_drop(&self.board);
            }
            if is_key_pressed(KeyCode::Space) { // Hard drop
                piece.hard_drop(&self.board);
            }

            // Rotation
            if is_key_pressed(KeyCode::A) { // CCW
                piece.rotate_ccw(&self.board);
            }
            if is_key_pressed(KeyCode::D) { // CW
                piece.rotate_cw(&self.board);
            }
            if is_key_pressed(KeyCode::S) { // 180
                piece.rotate_180(&self.board);
            }

            // Hold
            if is_key_pressed(KeyCode::LeftShift) {
                match self.hold_piece {
                    Some(hold_piece) => {
                        self.hold_piece = Some(piece.piece);
                        self.active_piece = Some(hold_piece.into());
                    },
                    None => {
                        self.hold_piece = Some(piece.piece);
                        self.active_piece = Some(self.queue.shift().into());
                    }
                }
            }
        }
    }
}
