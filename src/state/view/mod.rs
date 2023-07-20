use macroquad::prelude::*;

use super::State;

impl State {
    pub fn render(&self) {
        clear_background(color_u8!(0x12, 0x34, 0x56, 0xff));

        // Draw Board lines
        for x in 0..11 {
            draw_line(
                x as f32 * 32.0,
                0.0,
                x as f32 * 32.0,
                640.0,
                2.0,
                color_u8!(0xff, 0xff, 0xff, 0xff),
            );
        }
        for y in 0..21 {
            draw_line(
                0.0,
                y as f32 * 32.0,
                320.0,
                y as f32 * 32.0,
                2.0,
                color_u8!(0xff, 0xff, 0xff, 0xff),
            );
        }

        // Draw Board
        for x in 0..10 {
            for y in 0..20 {
                if let Some(piece) = self.board[y + 20][x] {
                    draw_rectangle(
                        x as f32 * 32.0,
                        y as f32 * 32.0,
                        32.0,
                        32.0,
                        piece.into(),
                    );
                }
            }
        }

        // Draw active piece
        if let Some(piece) = &self.active_piece {
            let projection = piece.project();

            for x in 0..10 {
                for y in 0..20 {
                    if let Some(piece) = projection[y + 20][x] {
                        draw_rectangle(
                            x as f32 * 32.0,
                            y as f32 * 32.0,
                            32.0,
                            32.0,
                            piece.into(),
                        );
                    }
                }
            }
        }
    }
}
