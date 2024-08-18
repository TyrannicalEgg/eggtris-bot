use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone, Copy)]
pub enum Piece {
    I, O, J, L, S, Z, T
}

impl Piece {
    pub fn get_matrix(&self) -> Vec<u16> {
        match self {
            Piece::I => {
                vec![
                    0b0000,
                    0b1111,
                    0b0000,
                    0b0000,
                ]
            },
            Piece::O => {
                vec![
                    0b11,
                    0b11,
                ]
            },
            Piece::J => {
                vec![
                    0b100,
                    0b111,
                    0b000,
                ]
            },
            Piece::L => {
                vec![
                    0b001,
                    0b111,
                    0b000,
                ]
            },
            Piece::S => {
                vec![
                    0b011,
                    0b110,
                    0b000,
                ]
            },
            Piece::Z => {
                vec![
                    0b110,
                    0b011,
                    0b000,
                ]
            },
            Piece::T => {
                vec![
                    0b010,
                    0b111,
                    0b000,
                ]
            },
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub enum Block {
    I, O, J, L, S, Z, T, G
}
