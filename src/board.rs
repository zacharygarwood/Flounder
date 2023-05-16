use crate::defs::*;
use crate::pieces::{Color, Pieces};

pub struct Board {
    board: [Pieces; 128],
    turn_color: Color,
}
impl Board {
    pub fn new() -> Self {
        use crate::pieces::{Color::*, Pieces::*};
        Self {
            board: [
                // 0x88 Board Representation
                r, n, b, q, k, b, n, r, o, o, o, o, o, o, o, o, p, p, p, p, p, p, p, p, o, o, o, o,
                o, o, o, o, e, e, e, e, e, e, e, e, o, o, o, o, o, o, o, o, e, e, e, e, e, e, e, e,
                o, o, o, o, o, o, o, o, e, e, e, e, e, e, e, e, o, o, o, o, o, o, o, o, e, e, e, e,
                e, e, e, e, o, o, o, o, o, o, o, o, P, P, P, P, P, P, P, P, o, o, o, o, o, o, o, o,
                R, N, B, Q, K, B, N, R, o, o, o, o, o, o, o, o,
            ],
            turn_color: WHITE,
        }
    }

    pub fn set_board_from_fen(&mut self, fen: &str) {
        // Clear the board
        self.reset_board();

        // Split the FEN string into separate components
        let fen_parts: Vec<&str> = fen.split(' ').collect();
        let fen_board = fen_parts[0];

        // Iterate over the FEN board representation and populate the actual board
        let mut file = 0;
        let mut rank = RANKS - 1;

        // Used for modifying the file number
        let mut special_fen_char = false;

        for fen_char in fen_board.chars() {
            match fen_char {
                'r' => self.set_square(rank, file, Pieces::r),
                'n' => self.set_square(rank, file, Pieces::n),
                'b' => self.set_square(rank, file, Pieces::b),
                'q' => self.set_square(rank, file, Pieces::q),
                'k' => self.set_square(rank, file, Pieces::k),
                'p' => self.set_square(rank, file, Pieces::p),
                'R' => self.set_square(rank, file, Pieces::R),
                'N' => self.set_square(rank, file, Pieces::N),
                'B' => self.set_square(rank, file, Pieces::B),
                'Q' => self.set_square(rank, file, Pieces::Q),
                'K' => self.set_square(rank, file, Pieces::K),
                'P' => self.set_square(rank, file, Pieces::P),
                '/' => {
                    rank -= 1;
                    file = 0;
                    special_fen_char = true;
                }
                _ => {
                    if let Some(spaces) = fen_char.to_digit(10) {
                        file += spaces as usize;
                        special_fen_char = true;
                    }
                }
            }

            // Special chars already changed file
            if !special_fen_char {
                file += 1;
            }
            special_fen_char = false;
        }
    }

    pub fn print_board(&self) {
        println!();
        for rank in (0..RANKS).rev() {
            for file in 0..FILES {
                let square = Self::get_square(rank, file);

                // Print ranks
                if file == 0 {
                    print!(" {}  ", rank);
                }

                // Print board contents, avoiding the off board values
                if (square & 0x88) == 0 {
                    print!(" {} ", self.board[square])
                }
            }
            println!();
        }
        println!("\n     a  b  c  d  e  f  g  h");
    }

    fn reset_board(&mut self) {
        use crate::pieces::Pieces::*;
        for square in &mut self.board {
            *square = e;
        }
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

    fn set_square(&mut self, rank: usize, file: usize, piece: Pieces) {
        let square = Self::get_square(rank, file);
        self.board[square] = piece;
    }
}
