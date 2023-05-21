use std::num::Wrapping;
use rand::prelude::ThreadRng;
use rand::RngCore;

use crate::bitboard::{Bitboard, BitboardOperations, RANKS, FILES, SQUARES};
use crate::moves::{NORTH, SOUTH, EAST, WEST};
use crate::square::{rank_file_to_square, square_to_rank_file};
use crate::square::{Square, square_to_algebraic};
use crate::pieces::Piece;
use crate::util::print_bitboard;

const ROOK_RELEVANT_BITS: [usize; 64] = [
    12, 11, 11, 11, 11, 11, 11, 12,
    11, 10, 10, 10, 10, 10, 10, 11,
    11, 10, 10, 10, 10, 10, 10, 11,
    11, 10, 10, 10, 10, 10, 10, 11,
    11, 10, 10, 10, 10, 10, 10, 11,
    11, 10, 10, 10, 10, 10, 10, 11,
    11, 10, 10, 10, 10, 10, 10, 11,
    12, 11, 11, 11, 11, 11, 11, 12
];

const BISHOP_RELEVANT_BITS: [usize; 64] = [
    6, 5, 5, 5, 5, 5, 5, 6,
    5, 5, 5, 5, 5, 5, 5, 5,
    5, 5, 7, 7, 7, 7, 5, 5,
    5, 5, 7, 9, 9, 7, 5, 5,
    5, 5, 7, 9, 9, 7, 5, 5,
    5, 5, 7, 7, 7, 7, 5, 5,
    5, 5, 5, 5, 5, 5, 5, 5,
    6, 5, 5, 5, 5, 5, 5, 6
];

// Random number used in random number generation
static STATE: u64 = 1804289383; 

pub struct Table {
    pub knight_lookup: [Bitboard; 64],
    pub king_lookup: [Bitboard; 64],
    pub magic_table: Magic,
}

pub struct Magic {
    pub rook_attack_masks: [Bitboard; 64],
    pub bishop_attack_masks: [Bitboard; 64],
    pub rook_attacks: Vec<Vec<Bitboard>>,
    pub bishop_attacks: Vec<Vec<Bitboard>>,
    pub rook_magics: [u64; 64],
    pub bishop_magics: [u64; 64],
}

impl Magic {
    pub fn new() -> Self {
        let bishop_magics = Self::init_magics(Piece::Bishop);
        let rook_magics = Self::init_magics(Piece::Rook);
        let (bishop_attack_masks, bishop_attacks) = Self::init_slider_attacks(Piece::Bishop, bishop_magics);
        let (rook_attack_masks, rook_attacks) = Self::init_slider_attacks(Piece::Rook, rook_magics);

        Self {
            rook_attack_masks,
            bishop_attack_masks,
            rook_attacks,
            bishop_attacks,
            rook_magics,
            bishop_magics,
        }
    }

    pub fn get_bishop_attacks(&self, square: Square, mut occupancy: Bitboard) -> Bitboard {
        occupancy &= self.bishop_attack_masks[square as usize];
        occupancy = occupancy.wrapping_mul(self.bishop_magics[square as usize]);
        occupancy >>= 64 - BISHOP_RELEVANT_BITS[square as usize];
        
        self.bishop_attacks[square as usize][occupancy as usize]
    }

    pub fn get_rook_attacks(&self, square: Square, mut occupancy: Bitboard) -> Bitboard {
        occupancy &= self.rook_attack_masks[square as usize];
        occupancy = occupancy.wrapping_mul(self.rook_magics[square as usize]);
        occupancy >>= 64 - ROOK_RELEVANT_BITS[square as usize];

        self.rook_attacks[square as usize][occupancy as usize]
    }

    fn init_magics(piece: Piece) -> [u64; 64] {
        let mut magics = [0; 64];
        for square in 0..SQUARES {
            let relevant_bits = match piece {
                Piece::Bishop => BISHOP_RELEVANT_BITS[square as usize],
                _ => ROOK_RELEVANT_BITS[square as usize],
            };
            magics[square as usize] = Self::find_magic(square, piece, relevant_bits);
        }
        magics
    }

    fn find_magic(square: Square, piece: Piece, relevant_bits: usize) -> u64 {
        let mut occupancies = [Bitboard::empty(); 4096];
        let occupancy_variations = 1 << relevant_bits;

        let mut attacks = [Bitboard::empty(); 4096];
        let attack_mask = Self::generate_attack_mask(piece, square, Bitboard::empty(), false);

        // Generate attack masks for each occupancy variation
        for i in 0..occupancy_variations {
            occupancies[i] = Self::generate_occupancy_board(i as u8, attack_mask);
            attacks[i] = Self::generate_attack_mask(piece, square, occupancies[i], true);
        }

        // Testing magic numbers
        for random_count in 0..1000000 {
            let magic: u64 = Self::gen_random_number();

            // Reset used attacks
            let mut used_attacks = [Bitboard::empty(); 4096];

            let mut fail = false;
            for i in 0..occupancy_variations {
                let magic_index = occupancies[i].wrapping_mul(magic) >> (64 - relevant_bits);

                if used_attacks[magic_index as usize] == 0 { // Free index in used attacks
                    used_attacks[magic_index as usize] = attacks[i]
                } else if used_attacks[magic_index as usize] != attacks[i] { // Fail on collision
                    fail = true;
                }
            }

            // Found a magic number
            if !fail {
                println!("magic: {}", magic);
                return magic;
            }
        } 
        println!("Failed in magic number creation");
        return 0

    }

    // Returns the attacks masks 
    fn init_slider_attacks(piece: Piece, magics: [u64; 64]) -> ([Bitboard; 64], Vec<Vec<Bitboard>>) {
        // Initialize variables to return
        let mut piece_masks = [Bitboard::empty(); 64];
        let mut piece_attacks = match piece {
            Piece::Bishop => (0..64).map(|_| vec![0; 512]).collect::<Vec<Vec<Bitboard>>>(),
            _ => (0..64).map(|_| vec![0; 4096]).collect::<Vec<Vec<Bitboard>>>(), // Rook is the only other option
        };

        for square in 0..SQUARES {
            // Generate attack mask and store result
            let attack_mask = Self::generate_attack_mask(piece, square, Bitboard::empty(), false);
            piece_masks[square as usize] = attack_mask;
            
            let relevant_bits = match piece {
                Piece::Bishop => BISHOP_RELEVANT_BITS[square as usize],
                _ => ROOK_RELEVANT_BITS[square as usize],
            };

            // Create the piece attacks by mapping a certain (occupancy * magic) shifted to the attacks 
            let occupancy_variations = 1 << relevant_bits;
            for i in 0..occupancy_variations {
                let occupancy = Self::generate_occupancy_board(i as u8, attack_mask);
                let magic_index = occupancy.wrapping_mul(magics[square as usize]) >> (64 - relevant_bits);
                piece_attacks[square as usize][magic_index as usize] = Self::generate_attack_mask(piece, square, occupancy, true);
            }
        }
        (piece_masks, piece_attacks)
    }

    fn generate_occupancy_board(index: u8, attack_mask: Bitboard) -> Bitboard {
        let mut blocker_board: Bitboard = attack_mask;
    
        let mut bit_index: i8 = 0;
        for square in 0..SQUARES {
            if attack_mask & Bitboard::square_to_bitboard(square) != 0 {
                if index & Bitboard::square_to_bitboard(bit_index as u8) as u8 == 0 {
                    blocker_board &= !Bitboard::square_to_bitboard(square);
                }
                bit_index += 1;
            }
        }
        blocker_board
    }

    fn generate_attack_mask(piece: Piece, square: u8, blockers: Bitboard, block: bool) -> Bitboard {
        match piece {
            Piece::Bishop => Self::generate_bishop_attack_mask(square, blockers, block),
            _ => Self::generate_rook_attack_mask(square, blockers, block),
        }
    }

    fn generate_rook_attack_mask(square: u8, blockers: Bitboard, block: bool) -> Bitboard {
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
    
    fn generate_bishop_attack_mask(square: Square, blockers: Bitboard, block: bool) -> Bitboard {
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

    fn gen_random_number() -> u64 {
        let n1: u64 = Self::gen_u64();
        let n2: u64 = Self::gen_u64();
        let n3: u64 = Self::gen_u64();
        n1 & n2 & n3
    }


    fn gen_u64() -> u64 {
        let mut random = ThreadRng::default();
        let u1: u64 = random.next_u64() & 0xFFFF;
        let u2: u64 = random.next_u64() & 0xFFFF;
        let u3: u64 = random.next_u64() & 0xFFFF;
        let u4: u64 = random.next_u64() & 0xFFFF;
        u1 | (u2 << 16) | (u3 << 32) | (u4 << 48)
    }
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
