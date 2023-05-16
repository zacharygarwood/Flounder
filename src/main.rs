mod board;
mod defs;
mod pieces;
use board::Board;

fn main() {
    let board = Board::new();
    board.print_board();
}
