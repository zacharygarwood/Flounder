use crate::pieces::{PieceType, Color, PIECE_COUNT, COLOR_COUNT};
use crate::square::Square;
use crate::bitboard::{Bitboard, RANKS, FILES, RANK_3, FILE_A, FILE_H};

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

    pub fn generate_moves(&self) -> Vec<u64> {
        use crate::pieces::{PieceType::*, Color::*};

        let mut moves = Vec::new();

        // Generate moves for each piece type (pawns, knights, bishops, rooks, etc.)
        // Example: White Pawns
        let pawns = self.bb_piece(Pawn);
        let pawn_moves = self.generate_pawn_moves();
        moves.extend(pawn_moves);

        // Generate moves for other piece types

        moves
    }

    fn generate_pawn_moves(&self) -> Vec<u64> {
        use crate::pieces::{PieceType::*, Color::*};

        let mut moves = Vec::new();
        let color = self.active_color;
        let pawns = self.bb_piece(Pawn);
        let empty_squares = !(self.bb_color(White) | self.bb_color(Black));

        let forward_mask = match color {
            Color::White => 8,
            Color::Black => -8,
        };
        let left_attack_mask = match color {
            Color::White => 7,
            Color::Black => -9,
        };
        let right_attack_mask = match color {
            Color::White => 9,
            Color::Black => -7,
        };

        // Generate single pawn pushes
        let single_pushes = (pawns << forward_mask) & empty_squares;
        moves.push(single_pushes);
        
        // Generate double pawn pushes
        let double_pawns = single_pushes & RANK_3;
        let double_pushes = (double_pawns << forward_mask) & empty_squares;
        moves.push(double_pushes);

        // Generate pawn attacks
        let pawn_attacks = match color {
            Color::White => ((pawns & !FILE_H) << left_attack_mask) | ((pawns & !FILE_A) << right_attack_mask),
            Color::Black => ((pawns & !FILE_H) >> right_attack_mask) | ((pawns & !FILE_A) >> left_attack_mask),
        };
        let enemy_pieces = match color {
            Color::White => self.position.colors[Black],
            Color::Black => self.position.colors[White],
        };
        let valid_pawn_attacks = pawn_attacks & enemy_pieces;
        moves.push(valid_pawn_attacks);

        moves
    }

        // Returns select pieces of a certain color e.g. white pawns
        pub fn bb(&self, color: Color, piece: PieceType) -> u64 {
            self.position.bb(color, piece)
        }
    
        // Returns all pieces of a certain color e.g. white pieces
        pub fn bb_color(&self, color: Color) -> u64 {
            self.position.bb_color(color)
        }
    
        // Returns all pieces of a select type e.g. pawns
        pub fn bb_piece(&self, piece: PieceType) -> u64 {
            self.position.bb_piece(piece)
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
        
        // DELETEME: Testing pawn captures
        colors[Black] = colors[Black] | 0x0000000000880000;

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