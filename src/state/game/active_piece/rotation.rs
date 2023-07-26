use super::{ActivePiece, Board};

const ROTATIONS: [[u8; 2]; 12] = [
    // from >> to
    // 90 deg
    [0, 1],
    [1, 0],
    [1, 2],
    [2, 1],
    [2, 3],
    [3, 2],
    [3, 0],
    [0, 3],
    // 180 deg
    [0, 2],
    [2, 0],
    [1, 3],
    [3, 1],
];

pub enum RotationResult {
    Normal,
    Tspin,
    Kick,
}

impl ActivePiece {
    pub fn rotate_by(&mut self, rotation: usize, board: &Board) -> Result<RotationResult, ()> {
        let from = self.rotation;
        let to = (self.rotation + rotation) % 4;

        let shape = &self.shape[to];

        let kick_table = ROTATIONS.iter().position(|&x| x == [from as u8, to as u8]).unwrap();
        let kick_table = &self.kick_tables[kick_table];

        for (i, kick) in kick_table.iter().enumerate() {
            if !shape.collides_at(self.x + kick.0, self.y - kick.1, board) {
                self.x += kick.0;
                self.y -= kick.1;
                self.rotation = to;

                if i == 0 {
                    // TODO check for tspin
                    if false {
                        return Ok(RotationResult::Tspin);
                    }

                    return Ok(RotationResult::Normal);
                } else {
                    return Ok(RotationResult::Kick);
                }
            }
        }

        return Err(());
    }
}
