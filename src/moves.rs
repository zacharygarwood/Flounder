use crate::{pieces::Piece, square::{square_to_algebraic, self}};

pub const NORTH: i8 = 8;
pub const EAST: i8 = 1;
pub const SOUTH: i8 = -NORTH;
pub const WEST: i8 = -EAST;

#[derive(Debug)]
pub struct Move {
    to: u8,
    from: u8,
    piece_type: Piece,
    move_type: MoveType,
}

impl Move {
    pub fn new(to: u8, from: u8, piece_type: Piece, move_type: MoveType) -> Self {
        Self {
            to,
            from,
            piece_type,
            move_type,
        }
    }

    pub fn print(&self) {
        println!("From: {} To: {} Piece: {} Move: {}", square_to_algebraic(self.from), square_to_algebraic(self.to), self.piece_type, self.move_type);
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