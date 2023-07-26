use super::{ActivePiece, Game};
use macroquad::prelude::*;

pub enum Signal {
    Quit,
    Restart,
}

impl Game {
    pub fn input(&mut self) -> Option<Signal> {
        if is_key_pressed(KeyCode::Escape) {
            return Some(Signal::Quit);
        }

        if is_key_pressed(KeyCode::R) {
            return Some(Signal::Restart);
        }

        if let Some(mut active_piece) = self.active_piece.take() {
            // Movement
            if is_key_pressed(KeyCode::Kp4) {
                // Left
                active_piece.left(&self.board);
            }
            if is_key_pressed(KeyCode::Kp6) {
                // Right
                active_piece.right(&self.board);
            }
            if is_key_pressed(KeyCode::Kp5) {
                // Soft drop
                active_piece.drop(&self.board);
            }
            if is_key_pressed(KeyCode::Space) {
                // Hard drop
                loop {
                    match active_piece.drop(&self.board) {
                        Ok(_) => {}
                        Err(_) => {
                            active_piece.write(&mut self.board);

                            active_piece = ActivePiece::new(
                                self.board.size(),
                                self.queue.shift(),
                                &self.rules,
                            );

                            break;
                        }
                    }
                }
            }

            // Rotation
            if is_key_pressed(KeyCode::A) {
                // CCW
                active_piece.rotate_by(3, &self.board);
            }
            if is_key_pressed(KeyCode::D) {
                // CW
                active_piece.rotate_by(1, &self.board);
            }
            if is_key_pressed(KeyCode::S) {
                // 180
                active_piece.rotate_by(2, &self.board);
            }

            // Hold
            if is_key_pressed(KeyCode::LeftShift) {
                match self.hold_piece.clone() {
                    None => {
                        self.hold_piece = Some(active_piece.name.clone());
                        active_piece = ActivePiece::new(
                            self.board.size(),
                            self.queue.shift(),
                            &self.rules,
                        );
                    }
                    Some(hold_piece) => {
                        self.hold_piece = Some(active_piece.name.clone());
                        active_piece = ActivePiece::new(
                            self.board.size(),
                            hold_piece.clone(),
                            &self.rules,
                        );
                    }
                }
            }

            self.active_piece = Some(active_piece);
        }

        None
    }
}
