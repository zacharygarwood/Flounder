use crate::pieces::{PieceType, Color, PIECE_COUNT, COLOR_COUNT};
use crate::square::Square;
use crate::bitboard::{Bitboard};

// Represents the chess board using bitboards
pub struct Board {
    position: Position,
    active_color: Color,
    castling_availability: Castle,
    en_passant: Option<Square>,
    half_move: u8,
    full_move: u8,
}

impl Board {
    pub fn new() -> Self{
        Self { 
            position: Position::new(),
            active_color: Color::White,
            castling_availability: Castle::new(),
            en_passant: None,
            half_move: 0,
            full_move: 1,
        }
    }

    // Returns select pieces of a certain color e.g. white pawns
    pub fn bb(&self, color: Color, piece: PieceType) -> Bitboard {
        self.position.bb(color, piece)
    }

    // Returns all pieces of a certain color e.g. white pieces
    pub fn bb_color(&self, color: Color) -> Bitboard {
        self.position.bb_color(color)
    }

    // Returns all pieces of a select type e.g. pawns
    pub fn bb_piece(&self, piece: PieceType) -> Bitboard {
            self.position.bb_piece(piece)
        }

    // Returns the color of the player to play
    pub fn active_color(&self) -> Color {
        self.active_color
    }

    // Bitboard of all empty spaces
    pub fn bb_empty(&self) -> Bitboard {
        !(self.bb_color(Color::White) | self.bb_color(Color::Black))
    }

    // Bitboard of all pieces
    pub fn bb_all(&self) -> Bitboard {
        self.bb_color(Color::White) | self.bb_color(Color::Black)
    }
}


pub struct Position {
    pieces: [Bitboard; PIECE_COUNT], // Six bitboards for the pieces
    colors: [Bitboard; COLOR_COUNT], // Two bitboards for the colors
}

impl Position {
    pub fn new() -> Self{
        use crate::pieces::{PieceType::*, Color::*};
        let mut pieces = [0; PIECE_COUNT];
        let mut colors = [0; COLOR_COUNT];
        
        pieces[Pawn] = 0x00ff00000000ff00;
        pieces[Knight] = 0x4200000000000042;
        pieces[Bishop] = 0x2400000000000024;
        pieces[Rook] = 0x8100000000000081;
        pieces[Queen] = 0x0800000000000008;
        pieces[King] = 0x1000000000000010;
        
        colors[White] = 0x000000000000ffff;
        colors[Black] = 0xffff000000000000;

        Self { pieces, colors}
    }

    // Returns select pieces of a certain color e.g. white pawns
    pub fn bb(&self, color: Color, piece: PieceType) -> u64 {
        self.pieces[piece] & self.colors[color]
    }

    // Returns all pieces of a certain color e.g. white pieces
    pub fn bb_color(&self, color: Color) -> u64 {
        self.colors[color]
    }

    // Returns all pieces of a select type e.g. pawns
    pub fn bb_piece(&self, piece: PieceType) -> u64 {
        self.pieces[piece]
    }
}

pub struct Castle {
    pub white_king: bool,
    pub white_queen: bool,
    pub black_king: bool,
    pub black_queen: bool,
}

impl Castle {
    pub fn new() -> Self {
        Self {
            white_king: true,
            white_queen: true,
            black_king: true,
            black_queen: true,
        } 
    }
}