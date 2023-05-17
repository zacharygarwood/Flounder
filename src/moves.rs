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
}

#[derive(Debug)]
pub struct Deltas {
    pub pawn: [(i32, i32); 2],
    pub knight: [(i32, i32); 8],
    pub king: [(i32, i32); 8],
    pub bishop: [(i32, i32); 4],
    pub rook: [(i32, i32); 4],
    pub queen: [(i32, i32); 8],
}

impl Deltas {
    pub fn new() -> Self {
        Self {
            pawn: [(1, 0), (2, 0)],
            knight: [(-1, 2), (1, 2), (-2, 1), (2, 1), (-2, -1), (2, -1), (-1, -2), (1, -2)],
            king: [
                (-1, 0),
                (-1, 1),
                (0, 1),
                (1, 1),
                (1, 0),
                (1, -1),
                (0, -1),
                (-1, -1),
            ],
            bishop: [(1, 1), (1, -1), (-1, 1), (-1, -1)],
            rook: [(0, 1), (0, -1), (1, 0), (-1, 0)],
            queen: [
                (1, 1),
                (1, -1),
                (-1, 1),
                (-1, -1),
                (0, 1),
                (0, -1),
                (1, 0),
                (-1, 0),
            ],
        }
    }
}