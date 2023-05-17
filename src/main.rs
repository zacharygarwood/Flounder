mod board;
mod defs;
mod pieces;
mod moves;
use board::Board;

fn main() {
    let mut board = Board::new();
    board.set_board_from_fen("r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - ");
    board.print_board();
}
