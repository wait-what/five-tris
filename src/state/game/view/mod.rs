use macroquad::prelude::*;

use super::Game;

impl Game {
    pub fn render(&self) {
        clear_background(color_u8!(0x20, 0x20, 0x26, 0xff));

        let board_size = self.board.size();
        let width = board_size.0;
        let height = board_size.1 - 20;

        let tile_size = 40.0;
        let line_width = 1.2;

        // Draw Board lines
        for x in 0..(width + 1) {
            // Vertical
            let x = x as f32 * tile_size;

            draw_line(
                x,
                tile_size * 3.0, // 3 tile offset
                x,
                (height as f32) * tile_size + tile_size * 3.0,
                line_width,
                color_u8!(0xff, 0xff, 0xff, 0xff),
            );
        }

        for y in 0..(height + 1) {
            // Horizontal
            let y = (y + 3) as f32 * tile_size;

            draw_line(
                0.0,
                y,
                (width as f32) * tile_size,
                y,
                line_width,
                color_u8!(0xff, 0xff, 0xff, 0xff),
            );
        }

        // Draw Board
        for (y, row) in self.board.iter().enumerate() {
            if y <= (20 - 3) {
                continue;
            }

            for (x, tile) in row.iter().enumerate() {
                // Draw board
                if let Some(piece) = tile {
                    let x = x as f32 * tile_size;
                    let y = (y - (20 - 3)) as f32 * tile_size;

                    draw_rectangle(x, y, tile_size, tile_size, piece.color);
                }
            }
        }

        // Draw active piece
        if let Some(active_piece) = &self.active_piece {
            let active_piece_rules = self.rules.pieces.get(&active_piece.name).unwrap();
            let shape = &active_piece_rules.shape[active_piece.rotation];

            for (y, row) in shape.iter().enumerate() {
                for (x, tile) in row.iter().enumerate() {
                    if tile.filled {
                        let color: Color = active_piece_rules.color;
                        draw_rectangle(
                            (x as isize + active_piece.x) as f32 * tile_size,
                            (y as isize + active_piece.y - (20 - 3)) as f32 * tile_size,
                            tile_size,
                            tile_size,
                            color,
                        );
                    }
                }
            }
        }
    }
}
