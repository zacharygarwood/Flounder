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

fn main() {
    // Initialize the bitboard with some sample positions
    let board = Board::new("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
    let move_gen = MoveGenerator::new();

    board.print();

    println!("Move positions: \n");
    // Generate and print the moves
    let moves = move_gen.generate_moves(&board);
    for m in moves {
        println!("FROM: {} TO: {} PIECE: {} MOVE: {}", square_to_algebraic(m.from), square_to_algebraic(m.to), m.piece_type, m.move_type);
    }
}