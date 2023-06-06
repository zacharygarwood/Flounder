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
mod repetition;

use board::*;
use search::Searcher;

use std::time::Instant;

fn main() {
    // Initialize the bitboard with some sample positions
    let board = Board::new("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
    let mut searcher = Searcher::new();

    board.print();

    println!("// Negamax AB//");
    let now = Instant::now();
    let (eval, mv) = searcher.best_move(&board, 6);
    println!("Time: {}", now.elapsed().as_secs());
    println!("Eval: {}", eval);

    if mv != None {
        mv.unwrap().pretty_print();
    } else {
        println!("No moves found");
    }
    println!();

}

// r2q1rk1/ppp2pp1/2n1b2p/3p4/3PnB1P/2P2NP1/P1Q1PPB1/R3K2R b KQ - 4 12