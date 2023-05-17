use std::io::Empty;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum PieceKind {
    Empty,
    OffBoard,
    Pawn,
    Rook,
    Knight,
    Bishop,
    Queen,
    King,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Color {
    White,
    Black,
}

#[derive(Debug, Copy, Clone)]
pub struct Piece {
    kind: PieceKind,
    color: Color,
}

impl Piece {
    // Shorthand piece definitions to make them easier to use in board.rs
    pub const P: Self = Self::create_piece(PieceKind::Pawn, Color::White);
    pub const B: Self = Self::create_piece(PieceKind::Bishop, Color::White);
    pub const N: Self = Self::create_piece(PieceKind::Knight, Color::White);
    pub const R: Self = Self::create_piece(PieceKind::Rook, Color::White);
    pub const Q: Self = Self::create_piece(PieceKind::Queen, Color::White);
    pub const K: Self = Self::create_piece(PieceKind::King, Color::White);
    pub const p: Self = Self::create_piece(PieceKind::Pawn, Color::Black);
    pub const b: Self = Self::create_piece(PieceKind::Bishop, Color::Black);
    pub const n: Self = Self::create_piece(PieceKind::Knight, Color::Black);
    pub const r: Self = Self::create_piece(PieceKind::Rook, Color::Black);
    pub const q: Self = Self::create_piece(PieceKind::Queen, Color::Black);
    pub const k: Self = Self::create_piece(PieceKind::King, Color::Black);
    pub const e: Self = Self::create_piece(PieceKind::Empty, Color::White);
    pub const o: Self = Self::create_piece(PieceKind::OffBoard, Color::White); 

    pub fn new(kind: PieceKind, color: Color) -> Self {
        Self { kind, color }
    }

    // Not accessible outside the module ensuring that the associated consts are properly initialized
    const fn create_piece(kind: PieceKind, color: Color) -> Piece {
        Piece { kind, color }
    }

    pub fn kind(&self) -> PieceKind {
        self.kind
    }

    pub fn color(&self) -> Color {
        self.color
    }

    pub fn is_empty(&self) -> bool {
        self.kind == PieceKind::Empty
    }
}

impl std::fmt::Display for Piece {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let piece_str = match (self.kind, self.color) {
            (PieceKind::Pawn, Color::White) => '♟',
            (PieceKind::Bishop, Color::White) => '♝',
            (PieceKind::Knight, Color::White) => '♞',
            (PieceKind::Rook, Color::White) => '♜',
            (PieceKind::Queen, Color::White) => '♛',
            (PieceKind::King, Color::White) => '♚',
            (PieceKind::Pawn, Color::Black) => '♙',
            (PieceKind::Bishop, Color::Black) => '♗',
            (PieceKind::Knight, Color::Black) => '♘',
            (PieceKind::Rook, Color::Black) => '♖',
            (PieceKind::Queen, Color::Black) => '♕',
            (PieceKind::King, Color::Black) => '♔',
            (PieceKind::Empty, Color::White) => '.', // Color doesn't matter on the Empty and OffBoard piece kind
            (PieceKind::OffBoard, Color::White) => 'o',
            (PieceKind::Empty, Color::Black) => '.',
            (PieceKind::OffBoard, Color::Black) => 'o',
        };
        write!(f, "{}", piece_str)
    }
}