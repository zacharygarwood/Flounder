use rand::Rng;

use crate::pieces::{PIECE_COUNT, COLOR_COUNT};
use crate::bitboard::SQUARES;

const CASTLE_RIGHTS_COUNT: usize = 4;

pub struct ZobristTable {
    table: [[[u64; SQUARES as usize]; PIECE_COUNT]; COLOR_COUNT],
    white_to_move: u64,
    castling_rights: [u64; CASTLE_RIGHTS_COUNT],
    en_passant_target: [u64; SQUARES as usize],
}

impl ZobristTable {
    pub fn new() -> Self {
        let mut rng = rand::thread_rng();
        let mut table = [[[0; SQUARES as usize]; PIECE_COUNT]; COLOR_COUNT];
        let mut white_to_move = 0;
        let mut castling_rights = [0; CASTLE_RIGHTS_COUNT];
        let mut en_passant_target = [0; SQUARES as usize];

        for color in 0..COLOR_COUNT {
            for piece in 0..PIECE_COUNT {
                for square in 0..SQUARES {
                    table[color][piece][square as usize] = rng.gen();
                }
            }
        }

        for castling_right in 0..CASTLE_RIGHTS_COUNT {
            castling_rights[castling_right] = rng.gen();
        }

        for square in 0..SQUARES {
            en_passant_target[square as usize] = rng.gen();
        }

        white_to_move = rng.gen();

        Self {
            table,
            white_to_move,
            castling_rights,
            en_passant_target,
        }

    }
}