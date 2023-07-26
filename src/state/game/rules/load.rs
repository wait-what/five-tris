use super::{PieceRules, Rules, Shape};
use macroquad::color::Color;
use serde::{de::Deserializer, Deserialize};
use std::collections::HashMap;

// Tile
#[derive(Copy, Clone, Debug)]
pub struct Tile {
    pub filled: bool,
    pub pivot: bool,
}

impl<'de> Deserialize<'de> for Tile {
    fn deserialize<D>(deserializer: D) -> Result<Tile, D::Error>
    where
        D: Deserializer<'de>,
    {
        let n = u8::deserialize(deserializer)?;
        Ok(Self {
            filled: n & (1 << 0) == 1,
            pivot: n & (1 << 1) == 2,
        })
    }
}

#[derive(Deserialize, Debug)]
struct Piece {
    color: String,
    shape: [Vec<Vec<Tile>>; 4],
    kick_table: String,
}

#[derive(Deserialize, Debug)]
struct RulesJson {
    pub pieces: HashMap<String, Piece>,
    pub kick_tables: HashMap<String, [Vec<(isize, isize)>; 12]>,
}

impl Rules {
    pub fn from_json(json: &str) -> Self {
        let rules_json: RulesJson = match serde_jsonc::from_str(json) {
            Ok(data) => data,
            Err(err) => panic!("Failed to parse rules.jsonc: {}", err),
        };

        let mut rules: HashMap<String, PieceRules> =
            HashMap::with_capacity(rules_json.pieces.len());
        for (name, piece) in rules_json.pieces {
            let color = {
                let color = piece.color.trim_start_matches('#');
                let color = u32::from_str_radix(color, 16).unwrap();

                Color::from_rgba(
                    (color >> 24) as u8,
                    (color >> 16) as u8,
                    (color >> 8) as u8,
                    color as u8,
                )
            };

            rules.insert(
                name,
                PieceRules {
                    color,
                    shape: piece.shape.map(|shape| Shape::new(shape)),
                    kick_tables: rules_json
                        .kick_tables
                        .get(&piece.kick_table)
                        .unwrap()
                        .clone(),
                },
            );
        }

        Self { pieces: rules }
    }
}
