use crate::board::Board;
use crate::pieces::{Piece, Color};
use crate::bitboard::BitboardIterator;

pub const PAWN_VALUE: isize = 100;
pub const KNIGHT_VALUE: isize = 320;
pub const BISHOP_VALUE: isize = 330;
pub const ROOK_VALUE: isize = 500;
pub const QUEEN_VALUE: isize = 900;
pub const KING_VALUE: isize = 20000;

pub const PAWN_PIECE_SQUARE_TABLE: [isize; 64] = [
    0,  0,  0,  0,  0,  0,  0,  0,
    50, 50, 50, 50, 50, 50, 50, 50,
    10, 10, 20, 30, 30, 20, 10, 10,
     5,  5, 10, 25, 25, 10,  5,  5,
     0,  0,  0, 20, 20,  0,  0,  0,
     5, -5,-10,  0,  0,-10, -5,  5,
     5, 10, 10,-20,-20, 10, 10,  5,
     0,  0,  0,  0,  0,  0,  0,  0
];

pub const KNIGHT_PIECE_SQUARE_TABLE: [isize; 64] = [
    -50,-40,-30,-30,-30,-30,-40,-50,
    -40,-20,  0,  0,  0,  0,-20,-40,
    -30,  0, 10, 15, 15, 10,  0,-30,
    -30,  5, 15, 20, 20, 15,  5,-30,
    -30,  0, 15, 20, 20, 15,  0,-30,
    -30,  5, 10, 15, 15, 10,  5,-30,
    -40,-20,  0,  5,  5,  0,-20,-40,
    -50,-40,-30,-30,-30,-30,-40,-50,
];

pub const BISHOP_PIECE_SQUARE_TABLE: [isize; 64] = [
    -20,-10,-10,-10,-10,-10,-10,-20,
    -10,  0,  0,  0,  0,  0,  0,-10,
    -10,  0,  5, 10, 10,  5,  0,-10,
    -10,  5,  5, 10, 10,  5,  5,-10,
    -10,  0, 10, 10, 10, 10,  0,-10,
    -10, 10, 10, 10, 10, 10, 10,-10,
    -10,  5,  0,  0,  0,  0,  5,-10,
    -20,-10,-10,-10,-10,-10,-10,-20,
];

pub const ROOK_PIECE_SQUARE_TABLE: [isize; 64] = [
    0,  0,  0,  0,  0,  0,  0,  0,
    5, 10, 10, 10, 10, 10, 10,  5,
   -5,  0,  0,  0,  0,  0,  0, -5,
   -5,  0,  0,  0,  0,  0,  0, -5,
   -5,  0,  0,  0,  0,  0,  0, -5,
   -5,  0,  0,  0,  0,  0,  0, -5,
   -5,  0,  0,  0,  0,  0,  0, -5,
    0,  0,  0,  5,  5,  0,  0,  0
];

pub const QUEEN_PIECE_SQUARE_TABLE: [isize; 64] = [
    -20,-10,-10, -5, -5,-10,-10,-20,
    -10,  0,  0,  0,  0,  0,  0,-10,
    -10,  0,  5,  5,  5,  5,  0,-10,
     -5,  0,  5,  5,  5,  5,  0, -5,
      0,  0,  5,  5,  5,  5,  0, -5,
    -10,  5,  5,  5,  5,  5,  0,-10,
    -10,  0,  5,  0,  0,  0,  0,-10,
    -20,-10,-10, -5, -5,-10,-10,-20
];

pub const KING_OPENING_PIECE_SQUARE_TABLE: [isize; 64] = [
    -30,-40,-40,-50,-50,-40,-40,-30,
    -30,-40,-40,-50,-50,-40,-40,-30,
    -30,-40,-40,-50,-50,-40,-40,-30,
    -30,-40,-40,-50,-50,-40,-40,-30,
    -20,-30,-30,-40,-40,-30,-30,-20,
    -10,-20,-20,-20,-20,-20,-20,-10,
     20, 20,  0,  0,  0,  0, 20, 20,
     20, 30, 10,  0,  0, 10, 30, 20
];

pub const KING_ENDGAME_PIECE_SQUARE_TABLE: [isize; 64] = [
    -50,-40,-30,-20,-20,-30,-40,-50,
    -30,-20,-10,  0,  0,-10,-20,-30,
    -30,-10, 20, 30, 30, 20,-10,-30,
    -30,-10, 30, 40, 40, 30,-10,-30,
    -30,-10, 30, 40, 40, 30,-10,-30,
    -30,-10, 20, 30, 30, 20,-10,-30,
    -30,-30,  0,  0,  0,  0,-30,-30,
    -50,-30,-30,-30,-30,-30,-30,-50
];

pub fn is_endgame(board: &Board) -> bool {
    no_queens(board) || (has_queen_with_most_one_minor_piece(Color::White, board) && has_queen_with_most_one_minor_piece(Color::Black, board))
}

fn no_queens(board: &Board) -> bool {
    board.bb_piece(Piece::Queen).count_ones() == 0
}

fn has_queen_with_most_one_minor_piece(color: Color, board: &Board) -> bool {
    let has_queen = board.bb(color, Piece::Queen).count_ones() != 0;
    let mut result = false;

    if has_queen {
        let pieces = (board.bb(color, Piece::Knight) | board.bb(color, Piece::Bishop) | board.bb(color, Piece::Rook));
        let has_no_other_pieces = pieces.count_ones() == 0;
        let has_one_minor_piece = (pieces & !board.bb(color, Piece::Rook)).count_ones() <= 1;

        result = has_no_other_pieces || has_one_minor_piece;
    }

    !has_queen || result
}

pub fn evaluate(board: &Board) -> isize {
    eval_material(board) + eval_position(board)
}

fn eval_material(board: &Board) -> isize {
    let color = board.active_color();

    let pawn_eval = piece_difference(color, Piece::Pawn, board) * PAWN_VALUE;
    let knight_eval = piece_difference(color, Piece::Knight, board) * KNIGHT_VALUE;
    let bishop_eval = piece_difference(color, Piece::Bishop, board) * BISHOP_VALUE;
    let rook_eval = piece_difference(color, Piece::Rook, board) * ROOK_VALUE;
    let queen_eval = piece_difference(color, Piece::Queen, board) * QUEEN_VALUE;
    let king_eval = piece_difference(color, Piece::King, board) * KING_VALUE;

    pawn_eval + knight_eval + bishop_eval + rook_eval + queen_eval + king_eval
}

fn piece_difference(color: Color, piece: Piece, board: &Board) -> isize {
    board.bb(color, piece).count_ones() as isize - board.bb(!color, piece).count_ones() as isize
}

fn eval_position(board: &Board) -> isize {
    let color = board.active_color();

    let pawn_eval = eval_piece_position(color, Piece::Pawn, board) - eval_piece_position(!color, Piece::Pawn, board);
    let knight_eval = eval_piece_position(color, Piece::Knight, board) - eval_piece_position(!color, Piece::Knight, board);
    let bishop_eval = eval_piece_position(color, Piece::Bishop, board) - eval_piece_position(!color, Piece::Bishop, board);
    let rook_eval = eval_piece_position(color, Piece::Rook, board) - eval_piece_position(!color, Piece::Rook, board);
    let queen_eval = eval_piece_position(color, Piece::Queen, board) - eval_piece_position(!color, Piece::Queen, board);
    let king_eval = eval_piece_position(color, Piece::King, board) - eval_piece_position(!color, Piece::King, board);

    pawn_eval + knight_eval + bishop_eval + rook_eval + queen_eval + king_eval
}

fn eval_piece_position(color:Color, piece: Piece, board: &Board) -> isize {
    let pieces = board.bb(color, piece);

    let piece_square_table = match piece {
        Piece::Pawn => PAWN_PIECE_SQUARE_TABLE,
        Piece::Knight => KNIGHT_PIECE_SQUARE_TABLE,
        Piece::Bishop => BISHOP_PIECE_SQUARE_TABLE,
        Piece::Rook => ROOK_PIECE_SQUARE_TABLE,
        Piece::Queen => QUEEN_PIECE_SQUARE_TABLE,
        Piece::King => {
            if is_endgame(board) {
                KING_ENDGAME_PIECE_SQUARE_TABLE
            } else {
                KING_OPENING_PIECE_SQUARE_TABLE
            }
        },
    };

    let mut score = 0;
    let iter = BitboardIterator::new(pieces);
    for square in iter {
        match color {
            Color::White => score += piece_square_table[63 - square as usize],
            Color::Black => score += piece_square_table[square as usize],
        }
    }
    score
}