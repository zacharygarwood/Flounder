use crate::pieces::{Piece, Color};

// Represents the chess board using bitboards
pub struct Board {
    pieces: [u64; 6], // Six bitboards, one for each piece type
    colors: [u64; 2], // Two bitboards, one for each color
    turn_color: Color, // Whos turn it is
}

impl Board {
    pub fn new() -> Self{
        use crate::pieces::{Piece::*, Color::*};
        let mut pieces = [0; 6];
        let mut colors = [0; 2];
        let turn_color = White;
        
        pieces[Pawn as usize] = 0x00ff00000000ff00;
        pieces[Knight as usize] = 0x4200000000000042;
        pieces[Bishop as usize] = 0x2400000000000024;
        pieces[Rook as usize] = 0x8100000000000081;
        pieces[Queen as usize] = 0x0800000000000008;
        pieces[King as usize] = 0x1000000000000010;
        
        colors[White as usize] = 0x000000000000ffff;
        colors[Black as usize] = 0xffff000000000000;
        
        // Testing pawn captures
        colors[Black as usize] = colors[Black as usize] | 0x0000000000880000;

        Self { pieces, colors, turn_color}
    }

    // Returns select pieces of a certain color e.g. white pawns
    pub fn get_colored_pieces(&self, color: Color, piece: Piece) -> u64 {
        self.pieces[piece as usize] & self.colors[color as usize]
    }

    // Returns all pieces of a certain color e.g. white pieces
    pub fn get_colors(&self, color: Color) -> u64 {
        self.colors[color as usize]
    }

    // Returns all pieces of a select type e.g. pawns
    pub fn get_pieces(&self, piece: Piece) -> u64 {
        self.pieces[piece as usize]
    }

    pub fn generate_moves(&self) -> Vec<u64> {
        use crate::pieces::{Piece::*, Color::*};

        let mut moves = Vec::new();

        // Generate moves for each piece type (pawns, knights, bishops, rooks, etc.)
        // Example: White Pawns
        let pawns = self.pieces[Pawn as usize] & self.colors[White as usize];
        let pawn_moves = self.generate_pawn_moves(pawns, self.turn_color);
        moves.extend(pawn_moves);

        // Generate moves for other piece types

        moves
    }

    fn generate_pawn_moves(&self, pawns: u64, color: Color) -> Vec<u64> {
        use crate::pieces::Color::*;

        let mut moves = Vec::new();
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

        // Generate single forward moves
        let single_forward_moves = (pawns << forward_mask) & !(self.colors[White as usize] | self.colors[Black as usize]);
        moves.push(single_forward_moves);
        
        // Generate double forward moves for pawns in the starting position
        let starting_pawns = match color {
            Color::White => pawns & 0x000000000000FF00,
            Color::Black => pawns & 0x00FF000000000000,
        };
        let double_forward_moves = (starting_pawns << (forward_mask * 2)) & !(self.colors[White as usize] | self.colors[Black as usize]);
        
        // Perform additional blocking check for double pawn moves
        // FIXME: There is a bug here where a double pawn push can jump over a piece. Frisky little buggers >:( (probably doing this wayyyy wrong)
        let blocking_squares = ((starting_pawns << forward_mask) | (starting_pawns << (forward_mask * 2))) & (self.colors[White as usize] | self.colors[Black as usize]);

        let valid_double_forward_moves = double_forward_moves & !blocking_squares;

        moves.push(valid_double_forward_moves);

        // Generate pawn attacks
        let pawn_attacks = match color {
            Color::White => ((pawns & !File::H.bitboard()) << left_attack_mask) | ((pawns & !File::A.bitboard()) << right_attack_mask),
            Color::Black => ((pawns & !File::H.bitboard()) >> right_attack_mask) | ((pawns & !File::A.bitboard()) >> left_attack_mask),
        };
        let enemy_pieces = match color {
            Color::White => self.colors[Black as usize],
            Color::Black => self.colors[White as usize],
        };
        let valid_pawn_attacks = pawn_attacks & enemy_pieces;
        moves.push(valid_pawn_attacks);

        moves
    }
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum File {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
}

impl File {
    pub fn bitboard(self) -> u64 {
        match self {
            File::A => 0x0101010101010101,
            File::B => 0x0202020202020202,
            File::C => 0x0404040404040404,
            File::D => 0x0808080808080808,
            File::E => 0x1010101010101010,
            File::F => 0x2020202020202020,
            File::G => 0x4040404040404040,
            File::H => 0x8080808080808080,
        }
    }
}