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
mod eval;
mod search;

use board::*;
use move_gen::MoveGenerator;
use pieces::{Piece, Color};
use square::{square_to_algebraic, algebraic_to_square};
use util::print_bitboard;
use moves::{Move, MoveType};
use search::Searcher;

fn main() {
    // Initialize the bitboard with some sample positions
    let mut board = Board::new("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
    let move_gen = MoveGenerator::new();
    let searcher = Searcher::new();

    let (eval, mv) = searcher.best_move(&board, 5);
    println!("Eval: {}", eval);

    if mv != None {
        mv.unwrap().pretty_print();
    } else {
        println!("No moves found");
    }



    // board.print();

    // let mv = Move::new(8, 24, Piece::Pawn, MoveType::Quiet);
    // board.make_move(&mv);

    // board.print();

    // board.make_move(&mv)
    // println!("{}", move_gen.run_perft(&board, 6));
    // move_gen.divide(&mut board, 1);
}