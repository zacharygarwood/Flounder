use crate::bitboard::{Bitboard, BitboardOperations, SQUARES};
use crate::moves::{NORTH, SOUTH, EAST, WEST};
use crate::square::Square;
use crate::pieces::Piece;
use crate::magic::Magic;

pub struct Table {
    pub knight_lookup: [Bitboard; 64],
    pub king_lookup: [Bitboard; 64],
    pub magic_table: Magic,
    pub inclusive_between_lookup: [[Bitboard; 64]; 64],
    pub exclusive_between_lookup: [[Bitboard; 64]; 64],
}

impl Table {
    pub fn init() -> Self {
        let knight_lookup = generate_knight_lookup_table();
        let king_lookup = generate_king_lookup_table();
        let magic_table = Magic::new();
        let inclusive_between_lookup = generate_inclusive_between_rays_table(&magic_table);
        let exclusive_between_lookup = generate_exclusive_between_rays_table(&magic_table);

        Self {
            knight_lookup,
            king_lookup,
            magic_table,
            inclusive_between_lookup,
            exclusive_between_lookup,
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

    pub fn between(&self, from: Square, to: Square, inclusive: bool) -> Bitboard {
        match inclusive {
            true => self.inclusive_between_lookup[from as usize][to as usize],
            false => self.exclusive_between_lookup[from as usize][to as usize],
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

// Ray is only between the squares to and from and doesnt't extend past
// Ex: when (from = a2, to = a4) the ray would equal a2 a3 a4
pub fn generate_inclusive_between_rays_table(magic_table: &Magic) -> [[Bitboard; 64]; 64] {
    let mut table = [[Bitboard::empty(); 64]; 64];

    for to in 0..SQUARES {
        for from in 0..SQUARES {
            let from_bb = Bitboard::square_to_bitboard(from);
            let to_bb = Bitboard::square_to_bitboard(to);

            let from_bishop_attacks = magic_table.get_bishop_attacks(from, to_bb);
            let from_rook_attacks = magic_table.get_rook_attacks(from, to_bb);
            
            let to_bishop_attacks = magic_table.get_bishop_attacks(to, from_bb);
            let to_rook_attacks = magic_table.get_rook_attacks(to, from_bb);

            if from_bishop_attacks & to_bb != 0 {
                table[from as usize][to as usize] = (from_bishop_attacks & to_bishop_attacks) | from_bb | to_bb;
            }

            if from_rook_attacks & to_bb != 0 {
                table[from as usize][to as usize] = (from_rook_attacks & to_rook_attacks) | from_bb | to_bb;
            }
        }
    }

    table
}

// Ray is between the squares to and from and extends past to the edges of the board
// Ex: when (from = a2, to = a4) the ray would equal a1 a2 a3 a4 a5 a6 a7 a8
pub fn generate_exclusive_between_rays_table(magic_table: &Magic) -> [[Bitboard; 64]; 64] {
    let mut table = [[Bitboard::empty(); 64]; 64];
    let empty = Bitboard::empty();

    for to in 0..SQUARES {
        for from in 0..SQUARES {
            let from_bb = Bitboard::square_to_bitboard(from);
            let to_bb = Bitboard::square_to_bitboard(to);

            let from_bishop_attacks = magic_table.get_bishop_attacks(from, empty);
            let from_rook_attacks = magic_table.get_rook_attacks(from, empty);
            
            let to_bishop_attacks = magic_table.get_bishop_attacks(to, empty);
            let to_rook_attacks = magic_table.get_rook_attacks(to, empty);

            if from_bishop_attacks & to_bb != 0 {
                table[from as usize][to as usize] = (from_bishop_attacks & to_bishop_attacks) | from_bb | to_bb;
            }

            if from_rook_attacks & to_bb != 0 {
                table[from as usize][to as usize] = (from_rook_attacks & to_rook_attacks) | from_bb | to_bb;
            }
        }
    }

    table
}
