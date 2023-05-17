mod board;
mod defs;
mod pieces;
mod moves;
use board::Board;
use pieces::{Piece, Color};

fn main() {
    // Initialize the bitboard with some sample positions
    let board = Board::new();

    println!("Staring position: \n");
    println!("White pieces: \n");
    print_board(board.get_colors(Color::White));

    println!("Black pieces: \n");
    print_board(board.get_colors(Color::Black));

    println!("Move positions: \n");
    // Generate and print the moves
    let moves = board.generate_moves();
    for m in moves {
        print_board(m)
    }
}

fn print_board(num: u64) {
    use crate::defs::*;
    for rank in (0..RANKS).rev() {
        print!(" {} ", rank+1);
        for file in 0..FILES {
            let square = rank * 8 + file;
            let bit = (num >> square) & 1;
            print!(" {} ", bit);
        }
        println!();
    }
    println!("    a  b  c  d  e  f  g  h");
    println!();
}
