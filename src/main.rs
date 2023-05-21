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
use util::print_bitboard;

fn main() {
    // Initialize the bitboard with some sample positions
    let board = Board::new("r5nr/pb1P2p1/4k3/7p/1nPp1B2/3B1PPP/p7/RN1QK1NR w KQ - 0 19");
    let move_gen = MoveGenerator::new();

    board.print();

    for mv in move_gen.generate_moves(&board) {
        mv.print();
    }
}