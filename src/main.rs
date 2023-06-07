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
mod uci;

use uci::Flounder;

fn main() {
    let mut flounder = Flounder::new();
    flounder.uci_loop();
}
