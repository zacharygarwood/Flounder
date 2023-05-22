mod board;
mod pieces;
mod moves;
mod move_gen;
mod square;
mod bitboard;
mod util;
mod table;
mod fen;
mod magic;
use board::*;
use move_gen::MoveGenerator;
use pieces::{Piece, Color};
use square::{square_to_algebraic, algebraic_to_square};
use util::print_bitboard;

fn main() {
    // Initialize the bitboard with some sample positions
    let board = Board::new("r3k2r/pp1b1pbp/nq2p1p1/1N6/PPpN1BnP/6P1/3QPPB1/R3K2R b KQkq - 2 13");
    let move_gen = MoveGenerator::new();

    board.print();

    for mv in move_gen.generate_moves(&board) {
        mv.print();
    }
}