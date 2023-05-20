use crate::bitboard::{Bitboard, BitOperations, RANKS, FILES};
use crate::moves::{NORTH, SOUTH, EAST, WEST};
use crate::square::{rank_file_to_square, square_to_rank_file};
use crate::square::{Square, square_to_algebraic};
use crate::pieces::Piece;
use crate::util::print_bitboard;

pub struct Table {
    pub knight_lookup: [Bitboard; 64],
    pub king_lookup: [Bitboard; 64],
    pub bishop_magics: Magic,
    pub rook_magics: Magic,
}

pub struct Magic {
    pub rook_blocker_masks: [Bitboard; 64],
    pub bishop_blocker_masks: [Bitboard; 64],
    pub rook_blocker_boards: Vec<Vec<Bitboard>>, // size should be [64][4096]
    pub bishop_blocker_boards: Vec<Vec<Bitboard>>, // size should be [64][512]

}

impl Magic {
    pub fn new() -> Self {
        let rook_blocker_masks = Self::generate_rook_blocker_masks();
        let bishop_blocker_masks = Self::generate_bishop_blocker_masks();
        let rook_blocker_boards = Self::generate_rook_blocker_boards(rook_blocker_masks);
        let bishop_blocker_boards = Self::generate_bishop_blocker_boards(bishop_blocker_masks);

        Self {
            rook_blocker_masks,
            bishop_blocker_masks,
            rook_blocker_boards,
            bishop_blocker_boards,
        }
    }

    fn generate_rook_blocker_boards(blocker_mask: [Bitboard; 64]) -> Vec<Vec<Bitboard>> {
        let mut rook_blocker_board = (0..64)
            .map(|_| vec![0; 4096])
            .collect::<Vec<Vec<Bitboard>>>();

        for rank in 0..RANKS {
            for file in 0..FILES {
                let square = rank_file_to_square(rank, file);
                let bits = blocker_mask[square as usize].count_ones();

                for i in 0..(1 << bits) {
                    rook_blocker_board[square as usize][i] = Self::generate_blocker_board(i as u8, blocker_mask[square as usize]);
                }
            }
        }
        rook_blocker_board
    }

    fn generate_bishop_blocker_boards(blocker_mask: [Bitboard; 64]) -> Vec<Vec<Bitboard>> {
        let mut bishop_blocker_board = (0..64)
            .map(|_| vec![0; 512])
            .collect::<Vec<Vec<Bitboard>>>();
        
        for rank in 0..RANKS {
            for file in 0..FILES {
                let square = rank_file_to_square(rank, file);
                let bits = blocker_mask[square as usize].count_ones();

                for i in 0..(1 << bits) {
                    bishop_blocker_board[square as usize][i] = Self::generate_blocker_board(i as u8, blocker_mask[square as usize]);
                }
            }
        }
        bishop_blocker_board
    }

    fn generate_blocker_board(index: u8, blocker_mask: Bitboard) -> Bitboard {
        let mut blocker_board: Bitboard = blocker_mask;
    
        let mut bit_index: i8 = 0;
        for rank in 0..RANKS {
            for file in 0..FILES {
                let square = rank_file_to_square(rank, file);
                if blocker_mask & Bitboard::square_to_bitboard(square) != 0 {
                    if index & Bitboard::square_to_bitboard(bit_index as u8) as u8 == 0 {
                        blocker_board &= !Bitboard::square_to_bitboard(square);
                    }
                    bit_index += 1;
                }
            }
        }
        blocker_board
    }

    fn generate_rook_blocker_masks() -> [Bitboard; 64] {
        let mut rook_blocker_masks: [Bitboard; 64] = [0; 64];
    
        for rank in 0..RANKS {
            for file in 0..FILES {
                let square = rank_file_to_square(rank, file);
                rook_blocker_masks[square as usize] = Self::generate_rook_blocker_mask(square);
            }
        }
        rook_blocker_masks
    }

    fn generate_bishop_blocker_masks() -> [Bitboard; 64] {
        let mut bishop_blocker_masks: [Bitboard; 64] = [0; 64];
    
        for rank in 0..RANKS {
            for file in 0..FILES {
                let square = rank_file_to_square(rank, file);
                bishop_blocker_masks[square as usize] = Self::generate_bishop_blocker_mask(square);
            }
        }
        bishop_blocker_masks
    }

    fn generate_rook_blocker_mask(square: u8) -> Bitboard {
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
    
    fn generate_bishop_blocker_mask(square: Square) -> Bitboard {
        let mut mask: Bitboard = 0;
        let (rank, file) = square_to_rank_file(square);
    
        // Generate mask in the bottom-left direction
        let mut f = file as i8 - 1;
        let mut r = rank as i8 - 1;
        while f >= 0 && r >= 0 {
            mask |= Bitboard::rank_file_to_bitboard(r as u8, f as u8);
            f -= 1;
            r -= 1;
        }
    
        // Generate mask in the bottom-right direction
        f = file as i8 + 1;
        r = rank as i8 - 1;
        while f < 8 && r >= 0 {
            mask |= Bitboard::rank_file_to_bitboard(r as u8, f as u8);
            f += 1;
            r -= 1;
        }
    
        // Generate mask in the top-left direction
        f = file as i8 - 1;
        r = rank as i8 + 1;
        while f >= 0 && r < 8 {
            mask |= Bitboard::rank_file_to_bitboard(r as u8, f as u8);
            f -= 1;
            r += 1;
        }
    
        // Generate mask in the top-right direction
        f = file as i8 + 1;
        r = rank as i8 + 1;
        while f < 8 && r < 8 {
            mask |= Bitboard::rank_file_to_bitboard(r as u8, f as u8);
            f += 1;
            r += 1;
        }
    
        mask &= !Bitboard::rank_file_to_edge_mask(rank, file);
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
