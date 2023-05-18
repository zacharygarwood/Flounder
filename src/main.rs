mod board;
mod pieces;
mod moves;
mod square;
mod bitboard;
use board::*;
use moves::generate_moves;
use pieces::{Piece, Color};

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

fn print_board(num: u64) {
    use crate::bitboard::{RANKS, FILES};

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
