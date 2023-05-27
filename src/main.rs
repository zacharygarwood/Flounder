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
use moves::{Move, MoveType};

fn main() {
    // Initialize the bitboard with some sample positions
    let mut board = Board::new("r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1");
    let move_gen = MoveGenerator::new();

    // board.print();

    // let mv = Move::new(8, 24, Piece::Pawn, MoveType::Quiet);
    // board.make_move(&mv);

    // board.print();

    // board.make_move(&mv)
    println!("{}", move_gen.run_perft(&board, 1));
    move_gen.divide(&mut board, 1);
}