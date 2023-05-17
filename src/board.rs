use crate::defs::*;
use crate::pieces::{Color, Piece, PieceKind};
use crate::moves::{Move, MoveType, Deltas};

pub struct Board {
    board: [Piece; 128],
    turn_color: Color,
}
impl Board {
    pub fn new() -> Self {
        Self {
            board: [
                // 0x88 Board Representation
                Piece::r, Piece::n, Piece::b, Piece::q, Piece::k, Piece::b, Piece::n, Piece::r,  Piece::o, Piece::o, Piece::o, Piece::o, Piece::o, Piece::o, Piece::o, Piece::o,
                Piece::p, Piece::p, Piece::p, Piece::p, Piece::p, Piece::p, Piece::p, Piece::p,  Piece::o, Piece::o, Piece::o, Piece::o, Piece::o, Piece::o, Piece::o, Piece::o, 
                Piece::e, Piece::e, Piece::e, Piece::e, Piece::e, Piece::e, Piece::e, Piece::e,  Piece::o, Piece::o, Piece::o, Piece::o, Piece::o, Piece::o, Piece::o, Piece::o, 
                Piece::e, Piece::e, Piece::e, Piece::e, Piece::e, Piece::e, Piece::e, Piece::e,  Piece::o, Piece::o, Piece::o, Piece::o, Piece::o, Piece::o, Piece::o, Piece::o, 
                Piece::e, Piece::e, Piece::e, Piece::e, Piece::e, Piece::e, Piece::e, Piece::e,  Piece::o, Piece::o, Piece::o, Piece::o, Piece::o, Piece::o, Piece::o, Piece::o, 
                Piece::e, Piece::e, Piece::e, Piece::e, Piece::e, Piece::e, Piece::e, Piece::e,  Piece::o, Piece::o, Piece::o, Piece::o, Piece::o, Piece::o, Piece::o, Piece::o, 
                Piece::P, Piece::P, Piece::P, Piece::P, Piece::P, Piece::P, Piece::P, Piece::P,  Piece::o, Piece::o, Piece::o, Piece::o, Piece::o, Piece::o, Piece::o, Piece::o, 
                Piece::R, Piece::N, Piece::B, Piece::Q, Piece::K, Piece::B, Piece::N, Piece::R,  Piece::o, Piece::o, Piece::o, Piece::o, Piece::o, Piece::o, Piece::o, Piece::o, 
            ],
            turn_color: Color::White,
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
                'r' => self.set_square(rank, file, Piece::r),
                'n' => self.set_square(rank, file, Piece::n),
                'b' => self.set_square(rank, file, Piece::b),
                'q' => self.set_square(rank, file, Piece::q),
                'k' => self.set_square(rank, file, Piece::k),
                'p' => self.set_square(rank, file, Piece::p),
                'R' => self.set_square(rank, file, Piece::R),
                'N' => self.set_square(rank, file, Piece::N),
                'B' => self.set_square(rank, file, Piece::B),
                'Q' => self.set_square(rank, file, Piece::Q),
                'K' => self.set_square(rank, file, Piece::K),
                'P' => self.set_square(rank, file, Piece::P),
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

            // Special chars already changed file count
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
        for (square, piece) in &mut self.board.iter_mut().enumerate() {
            // If square in on the board excluding off board, then make it empty
            if (square & 0x88) == 0 {
                *piece = Piece::e;
            }
        }
    }

    // pub fn generate_moves(&self) -> Vec<Move> {
    //     let mut moves = Vec::new();
    //     let deltas = Deltas::new();

    //     // Iterate over all squares on the board
    //     for (square, piece) in self.board.iter_mut().enumerate() {
    //         if (square & 0x88) == 0 {
    //             if piece.color() == self.turn_color {
    //                 match piece.kind() {
    //                     PieceKind::Pawn => {
    //                         self.generate_pawn_moves(square, &mut moves);
    //                     }
    //                     PieceKind::Rook => {
    //                         self.generate_slider_moves(square, &mut moves, deltas.rook);
    //                     }
    //                     PieceKind::Knight => {
    //                         self.generate_knight_moves(square, &mut moves);
    //                     }
    //                     PieceKind::Bishop => {
    //                         self.generate_slider_moves(square, &mut moves, deltas.bishop);
    //                     }
    //                     PieceKind::Queen => {
    //                         self.generate_slider_moves(square, &mut moves, deltas.queen);
    //                     }
    //                     PieceKind::King => {
    //                         self.generate_king_moves(square, &mut moves);
    //                     }
    //                     _ => {}
    //                 }
    //             }
    //         }
    //     }

    //     moves
    // }

    // // Generates all pawn moves for a given square
    // fn generate_pawn_moves(&self, square: usize, moves: &mut Vec<Move>) {
    //     let piece = self.board[square];
    //     let color = piece.color();
    //     let forward = if color == Color::White { NORTH } else { SOUTH };

    //     // Generate non-capturing pawn moves
    //     let single_push = square + forward;
    //     if self.is_square_empty(single_push) {
    //         self.add_pawn_move(square, single_push, moves);

    //         // Generate double push if pawn is on the starting rank
    //         let starting_rank = if color == Color::White {
    //             RANK_2
    //         } else {
    //             RANK_7
    //         };
    //         if Board::get_rank(square) == starting_rank {
    //             let double_push = single_push + forward;
    //             if self.is_square_empty(double_push) {
    //                 self.add_pawn_move(square, double_push, moves);
    //             }
    //         }
    //     }

    //     // Generate pawn captures
    //     let capture_left = square + forward + WEST;
    //     if self.is_square_enemy(capture_left, color) {
    //         self.add_pawn_capture(square, capture_left, moves);
    //     }

    //     let capture_right = square + forward + EAST;
    //     if self.is_square_enemy(capture_right, color) {
    //         self.add_pawn_capture(square, capture_right, moves);
    //     }
    // }

    // // Adds a non-capturing pawn move to the list of moves
    // fn add_pawn_move(&self, from: usize, to: usize, moves: &mut Vec<Move>) {
    //     moves.push(Move::new(from, to, MoveType::Quiet));
    // }

    // // Adds a pawn capture move to the list of moves
    // fn add_pawn_capture(&self, from: usize, to: usize, moves: &mut Vec<Move>) {
    //     moves.push(Move::new(from, to, MoveType::Capture));
    // }

    // // Generates all knight moves for a given square
    // fn generate_knight_moves(&self, square: usize, moves: &mut Vec<Move>) {
    //     let piece = self.board[square];

    //     for delta in KNIGHT_DELTAS.iter() {
    //         let target = square + *delta;

    //         if (target & 0x88) == 0 {
    //             if self.is_square_empty(target) || self.is_square_enemy(target, piece.color()) {
    //                 moves.push(Move::new(
    //                     square,
    //                     target,
    //                     if self.is_square_empty(target) {
    //                         MoveType::Quiet
    //                     } else {
    //                         MoveType::Capture
    //                     },
    //                 ));
    //             }
    //         }
    //     }
    // }

    // // Generates all sliding piece moves (rook, bishop, queen) for a given square
    // fn generate_slider_moves(&self, square: usize, moves: &mut Vec<Move>, deltas: &[i32]) {
    //     let piece = self.board[square];
    //     let color = piece.color();

    //     for delta in deltas {
    //         let mut target = (square as i32) + *delta;

    //         while (target & 0x88) == 0 {
    //             if self.is_square_empty(target) {
    //                 moves.push(Move::new(square, target as usize, MoveType::Quiet));
    //             } else if self.is_square_enemy(target as usize, color) {
    //                 moves.push(Move::new(square, target as usize, MoveType::Capture));
    //                 break;
    //             } else {
    //                 break;
    //             }

    //             target += *delta;
    //         }
    //     }
    // }

    // // Generates all king moves for a given square
    // fn generate_king_moves(&self, square: usize, moves: &mut Vec<Move>) {
    //     let piece = self.board[square];

    //     for delta in KING_DELTAS.iter() {
    //         let target = square + *delta;

    //         if (target & 0x88) == 0 {
    //             if self.is_square_empty(target) || self.is_square_enemy(target, piece.color()) {
    //                 moves.push(Move::new(
    //                     square,
    //                     target,
    //                     if self.is_square_empty(target) {
    //                         MoveType::Quiet
    //                     } else {
    //                         MoveType::Capture
    //                     },
    //                 ));
    //             }
    //         }
    //     }
    // }

    // // Checks if a square is empty
    // fn is_square_empty(&self, square: usize) -> bool {
    //     self.board[square].is_empty()
    // }

    // // Checks if a square contains an enemy piece
    // fn is_square_enemy(&self, square: usize, color: Color) -> bool {
    //     let piece = self.board[square];
    //     piece.color() != color && !piece.is_empty()
    // }

    fn get_file(square: usize) -> usize {
        square & 7
    }

    fn get_rank(square: usize) -> usize {
        square >> 4
    }

    fn get_square(rank: usize, file: usize) -> usize {
        (rank << 4) + file
    }

    fn set_square(&mut self, rank: usize, file: usize, piece: Piece) {
        let square = Self::get_square(rank, file);
        self.board[square] = piece;
    }
}
