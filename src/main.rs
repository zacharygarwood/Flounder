mod board;
mod pieces;
mod moves;
mod square;
mod bitboard;
mod util;
use board::*;
use moves::generate_moves;
use pieces::{Piece, Color};
use util::print_board;

fn main() {
    // Initialize the bitboard with some sample positions
    let board = Board::new();

    println!("Staring position: \n");
    println!("White pieces: \n");
    print_board(board.bb_color(Color::White));

    println!("Black pieces: \n");
    print_board(board.bb_color(Color::Black));

    println!("Move positions: \n");
    // Generate and print the moves
    let moves = generate_moves(&board);
    for m in moves {
        print_board(m)
    }
}
