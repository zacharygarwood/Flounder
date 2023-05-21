use crate::bitboard::{Bitboard, BitboardOperations, RANKS, FILES};
use crate::moves::{NORTH, SOUTH, EAST, WEST};
use crate::square::{rank_file_to_square, square_to_rank_file};
use crate::square::{Square, square_to_algebraic};
use crate::pieces::Piece;
use crate::util::print_bitboard;


/*

FOR TOMORROW
https://github.com/maksimKorzh/chess_programming/blob/master/src/magics/magics.c#L400

 */
pub struct Table {
    pub knight_lookup: [Bitboard; 64],
    pub king_lookup: [Bitboard; 64],
    pub bishop_magics: Magic,
    pub rook_magics: Magic,
}

pub struct Magic {
    pub rook_attack_masks: [Bitboard; 64],
    pub bishop_attack_masks: [Bitboard; 64],
    pub rook_blockers: Vec<Vec<Bitboard>>, // size should be [64][4096]
    pub bishop_blockers: Vec<Vec<Bitboard>>, // size should be [64][512]

}

impl Magic {
    pub fn new() -> Self {
        let rook_attack_masks = Self::generate_attack_masks(Piece::Rook);
        let bishop_attack_masks = Self::generate_attack_masks(Piece::Bishop);
        let rook_blockers = Self::generate_blockers(Piece::Rook, rook_attack_masks);
        let bishop_blockers = Self::generate_blockers(Piece::Bishop, bishop_attack_masks);

        Self {
            rook_attack_masks,
            bishop_attack_masks,
            rook_blockers,
            bishop_blockers,
        }
    }

    fn generate_blockers(piece: Piece, attack_mask: [Bitboard; 64]) -> Vec<Vec<Bitboard>> {
        let mut blocker_board = match piece {
            Piece::Bishop => (0..64).map(|_| vec![0; 512]).collect::<Vec<Vec<Bitboard>>>(),
            _ => (0..64).map(|_| vec![0; 4096]).collect::<Vec<Vec<Bitboard>>>(), // Rook is the only other option
        };

        for rank in 0..RANKS {
            for file in 0..FILES {
                let square = rank_file_to_square(rank, file);
                let bits = attack_mask[square as usize].count_ones();

                for i in 0..(1 << bits) {
                    blocker_board[square as usize][i] = Self::generate_blocker_board(i as u8, attack_mask[square as usize]);
                }
            }
        }
        blocker_board
    }

    fn generate_blocker_board(index: u8, attack_mask: Bitboard) -> Bitboard {
        let mut blocker_board: Bitboard = attack_mask;
    
        let mut bit_index: i8 = 0;
        for rank in 0..RANKS {
            for file in 0..FILES {
                let square = rank_file_to_square(rank, file);
                if attack_mask & Bitboard::square_to_bitboard(square) != 0 {
                    if index & Bitboard::square_to_bitboard(bit_index as u8) as u8 == 0 {
                        blocker_board &= !Bitboard::square_to_bitboard(square);
                    }
                    bit_index += 1;
                }
            }
        }
        blocker_board
    }

    fn generate_attack_masks(piece: Piece) -> [Bitboard; 64] {
        let mut attack_masks: [Bitboard; 64] = [0; 64];
    
        for rank in 0..RANKS {
            for file in 0..FILES {
                let square = rank_file_to_square(rank, file);
                match piece {
                    Piece::Bishop => attack_masks[square as usize] = Self::generate_bishop_attack_mask_with_blockers_or_not(square, Bitboard::empty(), false),
                    Piece::Rook => attack_masks[square as usize] = Self::generate_rook_attack_mask_with_blockers_or_not(square, Bitboard::empty(), false),
                    _ => {}
                };
            }
        }
        attack_masks
    }

    fn generate_rook_attack_mask_with_blockers_or_not(square: u8, blockers: Bitboard, block: bool) -> Bitboard {
        let mut mask: Bitboard = 0;
        let (rank, file) = square_to_rank_file(square);
    
        // Generate mask in the bottom direction
        let mut f = file as i8;
        let mut r = rank as i8 - 1;
        while r >= 0 {
            let square_bb = Bitboard::rank_file_to_bitboard(r as u8, f as u8);
            mask |= square_bb;
            if block && blockers & square_bb != 0 {
                break;
            }
            r -= 1;
        }
    
        // Generate mask in the left direction
        f = file as i8 - 1;
        r = rank as i8;
        while f >= 0 {
            let square_bb = Bitboard::rank_file_to_bitboard(r as u8, f as u8);
            mask |= square_bb;
            if block && blockers & square_bb != 0 {
                break;
            }
            f -= 1;
        }
    
        // Generate mask in the up direction
        f = file as i8;
        r = rank as i8 + 1;
        while f >= 0 && r < 8 {
            let square_bb = Bitboard::rank_file_to_bitboard(r as u8, f as u8);
            mask |= square_bb;
            if block && blockers & square_bb != 0 {
                break;
            }
            r += 1;
        }
    
        // Generate mask in the right direction
        f = file as i8 + 1;
        r = rank as i8;
        while f < 8 && r < 8 {
            let square_bb = Bitboard::rank_file_to_bitboard(r as u8, f as u8);
            mask |= square_bb;
            if block && blockers & square_bb != 0 {
                break;
            }
            f += 1;
        }

        if !block {
            mask &= !Bitboard::rank_file_to_edge_mask(rank, file);
        }
        mask
    }
    
    fn generate_bishop_attack_mask_with_blockers_or_not(square: Square, blockers: Bitboard, block: bool) -> Bitboard {
        let mut mask: Bitboard = 0;
        let (rank, file) = square_to_rank_file(square);
    
        // Generate mask in the bottom-left direction
        let mut f = file as i8 - 1;
        let mut r = rank as i8 - 1;
        while f >= 0 && r >= 0 {
            let square_bb = Bitboard::rank_file_to_bitboard(r as u8, f as u8);
            mask |= square_bb;
            if block && blockers & square_bb != 0 {
                break;
            }
            f -= 1;
            r -= 1;
        }
    
        // Generate mask in the bottom-right direction
        f = file as i8 + 1;
        r = rank as i8 - 1;
        while f < 8 && r >= 0 {
            let square_bb = Bitboard::rank_file_to_bitboard(r as u8, f as u8);
            mask |= square_bb;
            if block && blockers & square_bb != 0 {
                break;
            }
            f += 1;
            r -= 1;
        }
    
        // Generate mask in the top-left direction
        f = file as i8 - 1;
        r = rank as i8 + 1;
        while f >= 0 && r < 8 {
            let square_bb = Bitboard::rank_file_to_bitboard(r as u8, f as u8);
            mask |= square_bb;
            if block && blockers & square_bb != 0 {
                break;
            }
            f -= 1;
            r += 1;
        }
    
        // Generate mask in the top-right direction
        f = file as i8 + 1;
        r = rank as i8 + 1;
        while f < 8 && r < 8 {
            let square_bb = Bitboard::rank_file_to_bitboard(r as u8, f as u8);
            mask |= square_bb;
            if block && blockers & square_bb != 0 {
                break;
            }
            f += 1;
            r += 1;
        }
        
        if !block {
            mask &= !Bitboard::rank_file_to_edge_mask(rank, file);
        }
        mask
    }
}

impl Table {
    pub fn init() -> Self {
        Self {
            knight_lookup: generate_knight_lookup_table(),
            king_lookup: generate_king_lookup_table(),
            bishop_magics: Magic::new(), // TOOD: need to implement magics
            rook_magics: Magic::new(), // TODO: need to implement magics
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
    let mut table: [Bitboard; 64] = [Bitboard::empty(); 64];
    for rank in 0..RANKS {
        for file in 0..FILES {
            let square = rank_file_to_square(rank, file);
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
    }
    table
}

// Used to populte king_lookup. Each generated attack set can be indexed by the square of the king 
pub fn generate_king_lookup_table() -> [Bitboard; 64] {
    let mut table: [Bitboard; 64] = [Bitboard::empty(); 64];
    for rank in 0..RANKS {
        for file in 0..FILES {
            let square = rank_file_to_square(rank, file);
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
    }
    table
}
