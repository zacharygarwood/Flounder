use crate::pieces::{Piece, Color, PIECE_COUNT, COLOR_COUNT};
use crate::square::Square;
use crate::bitboard::{Bitboard, BitOperations};

// Represents the chess board using bitboards
pub struct Board {
    position: Position,
    active_color: Color,
    casting_ability: Castle,
    en_passant_target: Option<Square>,
    halfmove_clock: u8,
    fullmove_counter: u8,
}

impl Board {
    pub fn new() -> Self{
        Self { 
            position: Position::new(),
            active_color: Color::White,
            casting_ability: Castle::new(true, true, true, true),
            en_passant_target: None,
            halfmove_clock: 0,
            fullmove_counter: 1,
        }
    }

    // Returns select pieces of a certain color e.g. white pawns
    pub fn bb(&self, color: Color, piece: Piece) -> Bitboard {
        self.position.bb(color, piece)
    }

    // Returns all pieces of a certain color e.g. white pieces
    pub fn bb_color(&self, color: Color) -> Bitboard {
        self.position.bb_color(color)
    }

    // Returns all pieces of a select type e.g. pawns
    pub fn bb_piece(&self, piece: Piece) -> Bitboard {
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
        // use crate::pieces::{Piece::*, Color::*};
        let mut pieces = [0; PIECE_COUNT];
        let mut colors = [0; COLOR_COUNT];
        
        // pieces[Pawn] = 0x00ff00000000ff00;
        // pieces[Knight] = 0x4200000000000042;
        // pieces[Bishop] = 0x2400000000000024;
        // pieces[Rook] = 0x8100000000000081;
        // pieces[Queen] = 0x0800000000000008;
        // pieces[King] = 0x1000000000000010;
        
        // colors[White] = 0x000000000000ffff;
        // colors[Black] = 0xffff000000000000;

        Self { pieces, colors}
    }

    // Returns select pieces of a certain color e.g. white pawns
    pub fn bb(&self, color: Color, piece: Piece) -> u64 {
        self.pieces[piece] & self.colors[color]
    }

    // Returns all pieces of a certain color e.g. white pieces
    pub fn bb_color(&self, color: Color) -> u64 {
        self.colors[color]
    }

    // Returns all pieces of a select type e.g. pawns
    pub fn bb_piece(&self, piece: Piece) -> u64 {
        self.pieces[piece]
    }

    pub fn add_piece(&mut self, color: Color, piece: Piece, rank: u8, file: u8) {
        self.colors[color].set_bit(rank, file);
        self.pieces[piece].set_bit(rank, file);
    }
}

pub struct Castle {
    pub white_king: bool,
    pub white_queen: bool,
    pub black_king: bool,
    pub black_queen: bool,
}

impl Castle {
    pub fn new(white_king: bool, white_queen: bool, black_king: bool, black_queen: bool) -> Self {
        Self {
            white_king,
            white_queen,
            black_king,
            black_queen,
        } 
    }

    pub fn set(&mut self, castle: char, ability: bool) {
        match castle {
            'K' => self.white_king = ability,
            'Q' => self.white_queen = ability,
            'k' => self.black_king = ability,
            'q' => self.black_queen = ability,
            _ => {} // Can ignore any other character
        };
    }
}