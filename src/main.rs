mod board;
mod pieces;
mod moves;
mod move_gen;
mod square;
mod bitboard;
mod util;
mod table;
mod fen;
use board::*;
use move_gen::MoveGenerator;
use pieces::{Piece, Color};
use square::{square_to_algebraic, algebraic_to_square};
use util::{print_board, board_to_hex};
use fen::parse_piece_placement;

fn main() {
    // Initialize the bitboard with some sample positions
    // let board = Board::new();
    // let move_gen = MoveGenerator::new();

    // println!("Move positions: \n");
    // // Generate and print the moves
    // let moves = move_gen.generate_moves(&board);
    // for m in moves {
    //     println!("FROM: {} TO: {} PIECE: {} MOVE: {}", square_to_algebraic(m.from), square_to_algebraic(m.to), m.piece_type, m.move_type);
    // }

    let position: Position = parse_piece_placement("8/5Kn1/2P1nP2/2k3P1/p3p1qP/8/1r2P2N/5R1Q");

    println!("Staring position: \n");
    println!("White pieces: \n");
    print_board(position.bb_color(Color::White));

    println!("Black pieces: \n");
    print_board(position.bb_color(Color::Black));

}