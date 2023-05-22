use crate::bitboard::{Bitboard, BitboardOperations, SQUARES};
use crate::moves::{NORTH, SOUTH, EAST, WEST};
use crate::square::Square;
use crate::pieces::Piece;
use crate::magic::Magic;

pub struct Table {
    pub knight_lookup: [Bitboard; 64],
    pub king_lookup: [Bitboard; 64],
    pub magic_table: Magic,
}

impl Table {
    pub fn init() -> Self {
        Self {
            knight_lookup: generate_knight_lookup_table(),
            king_lookup: generate_king_lookup_table(),
            magic_table: Magic::new(),
        }
    }

    pub fn non_sliding_moves(&self, square: Square, piece: Piece) -> Bitboard {
        match piece {
            Piece::Knight => self.knight_lookup[square as usize],
            Piece::King => self.king_lookup[square as usize],
            _ => 0 
        }
    }

    pub fn sliding_moves(&self, square: Square, occupancy: Bitboard, piece: Piece) -> Bitboard {
        match piece {
            Piece::Bishop => self.magic_table.get_bishop_attacks(square, occupancy),
            Piece::Rook => self.magic_table.get_rook_attacks(square, occupancy),
            Piece::Queen => self.magic_table.get_bishop_attacks(square, occupancy) |
                            self.magic_table.get_rook_attacks(square, occupancy),
            _ => 0 
        }
    }
}


// Used to populte knight_lookup. Each generated attack set can be indexed by the square of the knight 
pub fn generate_knight_lookup_table() -> [Bitboard; 64] {
    let mut table: [Bitboard; 64] = [Bitboard::empty(); 64];
    for square in 0..SQUARES {
        let board = Bitboard::square_to_bitboard(square);

        table[square as usize] |= board.shift(NORTH + NORTH + EAST) |
            board.shift(NORTH + NORTH + WEST) |
            board.shift(SOUTH + SOUTH + EAST) |
            board.shift(SOUTH + SOUTH + WEST) |
            board.shift(NORTH + WEST + WEST) |
            board.shift(NORTH + EAST + EAST) |
            board.shift(SOUTH + WEST + WEST) |
            board.shift(SOUTH + EAST + EAST);
    }

    table
}

// Used to populte king_lookup. Each generated attack set can be indexed by the square of the king 
pub fn generate_king_lookup_table() -> [Bitboard; 64] {
    let mut table: [Bitboard; 64] = [Bitboard::empty(); 64];
    for square in 0..SQUARES {
        let board = Bitboard::square_to_bitboard(square);

        table[square as usize] |= board.shift(NORTH) |
            board.shift(SOUTH) |
            board.shift(EAST) |
            board.shift(WEST) |
            board.shift(NORTH + EAST) |
            board.shift(NORTH + WEST) |
            board.shift(SOUTH + EAST) |
            board.shift(SOUTH + WEST);
    }

    table
}
