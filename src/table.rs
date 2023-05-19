use crate::bitboard::{Bitboard, BitOperations, RANKS, FILES};
use crate::moves::{NORTH, SOUTH, EAST, WEST};
use crate::square::rank_file_to_square;
use crate::square::Square;
use crate::pieces::Piece;

pub struct Table {
    pub knight_lookup: [Bitboard; 64],
    pub king_lookup: [Bitboard; 64],
    pub bishop_magics: Magics,
    pub rook_magics: Magics,
}

pub struct Magics {
    // TODO: imeplement magics
}

impl Magics {
    pub fn new() -> Self {
        Self {}
    }
}

impl Table {
    pub fn init() -> Self {
        Self {
            knight_lookup: generate_knight_lookup_table(),
            king_lookup: generate_king_lookup_table(),
            bishop_magics: Magics::new(), // TOOD: need to implement magics
            rook_magics: Magics::new(), // TODO: need to implement magics
        }
    }

    pub fn moves(&self, square: Square, piece: Piece) -> Bitboard {
        match piece {
            Piece::Knight => self.knight_lookup[square as usize],
            Piece::King => self.king_lookup[square as usize],
            Piece::Bishop => 0, // TODO: magic stuff 
            Piece::Rook => 0,
            Piece::Queen => 0,
            _ => 0 // No need for Pawns as they are generated separately
        }
    }
}


// Used to populte knigh_lookup. Each generated attack set can be indexed by the square of the knight 
pub fn generate_knight_lookup_table() -> [Bitboard; 64] {
    let mut table: [Bitboard; 64] = [0; 64];
    for rank in 0..RANKS {
        for file in 0..FILES {
            let mut board: Bitboard = 0;
            let square = rank_file_to_square(rank as u8, file as u8) as usize;

            board |= 1 << square;
            table[square] |= board.shift(NORTH + NORTH + EAST) |
                board.shift(NORTH + NORTH + WEST) |
                board.shift(SOUTH + SOUTH + EAST) |
                board.shift(SOUTH + SOUTH + WEST) |
                board.shift(NORTH + WEST + WEST) |
                board.shift(NORTH + EAST + EAST) |
                board.shift(SOUTH + WEST + WEST) |
                board.shift(SOUTH + EAST + EAST);
        }
    }
    table
}

// Used to populte knigh_lookup. Each generated attack set can be indexed by the square of the king 
pub fn generate_king_lookup_table() -> [Bitboard; 64] {
    let mut table: [Bitboard; 64] = [0; 64];
    for rank in 0..RANKS {
        for file in 0..FILES {
            let mut board: Bitboard = 0;
            let square = rank_file_to_square(rank as u8, file as u8) as usize;

            board |= 1 << square;
            table[square] |= board.shift(NORTH) |
                board.shift(SOUTH) |
                board.shift(EAST) |
                board.shift(WEST) |
                board.shift(NORTH + EAST) |
                board.shift(NORTH + WEST) |
                board.shift(SOUTH + EAST) |
                board.shift(SOUTH + WEST);
        }
    }
    table
}