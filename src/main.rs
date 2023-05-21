mod board;
mod pieces;
mod moves;
mod move_gen;
mod square;
mod bitboard;
mod util;
mod table;
mod fen;
use board::*;
use move_gen::MoveGenerator;
use pieces::{Piece, Color};
use square::{square_to_algebraic, algebraic_to_square};

fn main() {
    // Initialize the bitboard with some sample positions
    let board = Board::new("rnbq1bnr/p1p2kp1/7p/8/1PPp4/5PPP/p3P3/RNBQKBNR b KQ - 0 10");
    let move_gen = MoveGenerator::new();

    board.print();

    for mv in move_gen.generate_moves(&board) {
        mv.print();
    }
}