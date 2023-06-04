mod board;
mod pieces;
mod moves;
mod move_gen;
mod square;
mod bitboard;
mod util;
mod lookup;
mod fen;
mod magic;
mod eval;
mod search;
mod zobrist;
mod transposition;

use board::*;
use search::Searcher;


use std::time::Instant;
use search::sort_moves;
use move_gen::MoveGenerator;
use moves::{Move, MoveType};
use pieces::Piece;

fn main() {
    // Initialize the bitboard with some sample positions
    let board = Board::new("r1b1kb1r/ppppnppp/2n5/1N2P3/8/q4N2/P1PBPPPP/1R1QKB1R b Kkq - 5 8");
    let move_gen = MoveGenerator::new();
    let mut searcher = Searcher::new();
    let bad_move = Move::new(62, 47, Piece::Knight, MoveType::Quiet);

    board.print();

    println!("// Negamax AB//");
    let now = Instant::now();
    let (eval, mv) = searcher.best_move(&board, 7);
    println!("Time: {}", now.elapsed().as_secs());
    println!("Eval: {}", eval);

    if mv != None {
        mv.unwrap().pretty_print();
    } else {
        println!("No moves found");
    }
    println!();

}