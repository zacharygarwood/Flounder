use crate::bitboard::{Bitboard, BitOperations, RANKS, FILES};
use crate::moves::{NORTH, SOUTH, EAST, WEST};
use crate::square::{rank_file_to_square, square_to_rank_file};
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


// Used to populte knight_lookup. Each generated attack set can be indexed by the square of the knight 
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

// Used to populte king_lookup. Each generated attack set can be indexed by the square of the king 
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

pub fn generate_rook_blocker_mask(square: u8) -> Bitboard {
    let mut mask: Bitboard = 0;
    let (rank, file) = square_to_rank_file(square);

    // Generate mask for each rank
    for r in 0..RANKS {
        if r == rank {
            continue; // Skip the current rank
        }
        let blocker_mask = Bitboard::rank_file_to_bitboard(r, file);
        mask |= blocker_mask;
    }

    // Generate mask for each file
    for f in 0..FILES {
        if f == file {
            continue; // Skip the current file
        }
        let blocker_mask = Bitboard::rank_file_to_bitboard(rank, f);
        mask |= blocker_mask;
    }

    mask &= !Bitboard::rank_file_to_edge_mask(rank, file);
    mask
}

pub fn generate_bishop_blocker_mask(square: Square) -> Bitboard {
    let mut mask: Bitboard = 0;
    let (rank, file) = square_to_rank_file(square);

    // Generate mask in the bottom-left direction
    let mut f = file - 1;
    let mut r = rank - 1;
    while f > 0 && r > 0 {
        mask |= Bitboard::rank_file_to_bitboard(r as u8, f as u8);
        f -= 1;
        r -= 1;
    }

    // Generate mask in the bottom-right direction
    f = file + 1;
    r = rank - 1;
    while f < 7 && r > 0 {
        mask |= Bitboard::rank_file_to_bitboard(r as u8, f as u8);
        f += 1;
        r -= 1;
    }

    // Generate mask in the top-left direction
    f = file - 1;
    r = rank + 1;
    while f > 0 && r < 7 {
        mask |= Bitboard::rank_file_to_bitboard(r as u8, f as u8);
        f -= 1;
        r += 1;
    }

    // Generate mask in the top-right direction
    f = file + 1;
    r = rank + 1;
    while f < 7 && r < 7 {
        mask |= Bitboard::rank_file_to_bitboard(r as u8, f as u8);
        f += 1;
        r += 1;
    }

    mask
}