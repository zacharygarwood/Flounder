use crate::bitboard::{Bitboard, RANKS, FILES, RANK_2, RANK_3, RANK_6, RANK_7, FILE_A, FILE_H};
use crate::pieces::Color;

pub const NORTH: i8 = 8;
pub const EAST: i8 = 1;
pub const SOUTH: i8 = -NORTH;
pub const WEST: i8 = -EAST;

#[derive(Debug)]
pub struct Move {
    pub from: u8,
    pub to: u8,
    pub move_type: MoveType,
}

impl Move {
    pub fn new(from: u8, to: u8, move_type: MoveType) -> Self {
        Self {
            from,
            to,
            move_type,
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub enum MoveType {
    Quiet,    // Non-capturing move
    Capture,  // Capturing move 
    EnPassant,
    Castle,
    Promotion,
}

impl std::fmt::Display for MoveType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let piece_str = match self {
            MoveType::Quiet => "Quiet",
            MoveType::Capture => "Capture",
            MoveType::EnPassant => "En Passant",
            MoveType::Castle => "Castle",
            MoveType::Promotion => "Promotion",
        };
        write!(f, "{}", piece_str)
    }
}