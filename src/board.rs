use crate::pieces::{Pieces, Color};
use crate::defs::*;

pub struct Board {
    board: [Pieces; 128],
    turn_color: Color,
}
impl Board {
    pub fn new() -> Self {
        use crate::pieces::{Pieces::*, Color::*};
        Self {
            board: [ // 0x88 Board Representation
                r, n, b, q, k, b, n, r,  o, o, o, o, o, o, o, o, 
                p, p, p, p, p, p, p, p,  o, o, o, o, o, o, o, o, 
                e, e, e, e, e, e, e, e,  o, o, o, o, o, o, o, o, 
                e, e, e, e, e, e, e, e,  o, o, o, o, o, o, o, o,
                e, e, e, e, e, e, e, e,  o, o, o, o, o, o, o, o,
                e, e, e, e, e, e, e, e,  o, o, o, o, o, o, o, o,
                P, P, P, P, P, P, P, P,  o, o, o, o, o, o, o, o,
                R, N, B, Q, K, B, N, R,  o, o, o, o, o, o, o, o, 
            ], 
            turn_color: WHITE
        }
    }

    pub fn print_board(&self) {
        println!();
        for rank in 0..RANKS {
            for file in 0..FILES {
                let square = Self::get_square(rank, file);

                // Print ranks
                if file == 0 {
                    print!(" {}  ", 8 - rank);
                }

                // Print board contents, avoiding the off board values
                if (square & 0x88) == 0 {
                    print!(" {} ", self.board[square])
                }
            }
            println!();
        }
        println!("\n     a  b  c  d  e  f  g  h")
    }

    fn get_file(square: usize) -> usize {
        square & 7
    }
    
    fn get_rank(square: usize) -> usize {
        square >> 4
    }
    
    fn get_square(rank: usize, file: usize) -> usize {
        (rank << 4) + file
    }
}