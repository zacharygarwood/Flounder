#[derive(Debug)]
pub struct Move {
    pub from: usize,
    pub to: usize,
    pub move_type: MoveType,
}

impl Move {
    pub fn new(from: usize, to: usize, move_type: MoveType) -> Self {
        Self {
            from,
            to,
            move_type,
        }
    }
}

#[derive(Debug)]
pub enum MoveType {
    Quiet,    // Non-capturing move
    Capture,  // Capturing move
    Evasion, 
    EnPassant,
    Castling,
}