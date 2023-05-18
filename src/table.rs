use crate::bitboard::{Bitboard, Shift, RANKS, FILES};
use crate::moves::{NORTH, SOUTH, EAST, WEST};
use crate::square::{rank_file_to_index, square_to_algebraic};
use crate::util::{print_board, board_to_hex};

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
    pub fn new() -> Self {
        Self {
            knight_lookup: generate_knight_lookup_table(),
            king_lookup: generate_king_lookup_table(),
            bishop_magics: Magics::new(), // TOOD: need to implement magics
            rook_magics: Magics::new(), // TODO: need to implement magics
        }
    }
}

pub fn generate_knight_lookup_table() -> [Bitboard; 64] {
    let mut table: [Bitboard; 64] = [0; 64];
    for rank in 0..RANKS {
        for file in 0..FILES {
            let mut board: Bitboard = 0;
            let square = rank_file_to_index(rank as u8, file as u8) as usize;

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

pub fn generate_king_lookup_table() -> [Bitboard; 64] {
    let mut table: [Bitboard; 64] = [0; 64];
    for rank in 0..RANKS {
        for file in 0..FILES {
            let mut board: Bitboard = 0;
            let square = rank_file_to_index(rank as u8, file as u8) as usize;

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