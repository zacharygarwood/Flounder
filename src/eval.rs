use crate::board::Board;
use crate::pieces::{Piece, Color};
use crate::bitboard::BitboardIterator;

pub static PAWN_PIECE_SQUARE_TABLE: [i16; 64] = [
    100, 100, 100, 100, 100, 100, 100, 100,
    150, 150, 150, 150, 150, 150, 150, 150,
    110, 110, 120, 130, 130, 120, 110, 110,
    105, 105, 110, 125, 125, 110, 105, 105,
    100, 100, 100, 120, 120, 100, 100, 100,
    105,  95,  90, 100, 100,  90,  95, 105,
    105, 110, 110,  80,  80, 110, 110, 105,
    100, 100, 100, 100, 100, 100, 100, 100, 
];

pub static KNIGHT_PIECE_SQUARE_TABLE: [i16; 64] = [
    270, 280, 290, 290, 290, 290, 280, 270,
    280, 300, 320, 320, 320, 320, 300, 280,
    290, 320, 330, 335, 335, 330, 320, 290,
    290, 325, 335, 340, 340, 335, 325, 290,
    290, 320, 335, 340, 340, 335, 320, 290,
    290, 325, 330, 335, 335, 330, 325, 290,
    280, 300, 320, 325, 325, 320, 300, 280,
    270, 280, 290, 290, 290, 290, 280, 270,    
];

pub static BISHOP_PIECE_SQUARE_TABLE: [i16; 64] = [
    310, 320, 320, 320, 320, 320, 320, 310,
    320, 330, 330, 330, 330, 330, 330, 320,
    320, 330, 335, 340, 340, 335, 330, 320,
    320, 335, 335, 340, 340, 335, 335, 320,
    320, 330, 340, 340, 340, 340, 330, 320,
    320, 340, 340, 340, 340, 340, 340, 320,
    320, 335, 330, 330, 330, 330, 335, 320,
    310, 320, 320, 320, 320, 320, 320, 310,    
];

pub static ROOK_PIECE_SQUARE_TABLE: [i16; 64] = [
    500, 500, 500, 500, 500, 500, 500, 500,
    505, 510, 510, 510, 510, 510, 510, 505,
    495, 500, 500, 500, 500, 500, 500, 495,
    495, 500, 500, 500, 500, 500, 500, 495,
    495, 500, 500, 500, 500, 500, 500, 495,
    495, 500, 500, 500, 500, 500, 500, 495,
    495, 500, 500, 500, 500, 500, 500, 495,
    500, 500, 500, 505, 505, 500, 500, 500,    
];

pub static QUEEN_PIECE_SQUARE_TABLE: [i16; 64] = [
    880, 890, 890, 895, 895, 890, 890, 880,
    890, 900, 900, 900, 900, 900, 900, 890,
    890, 900, 905, 905, 905, 905, 900, 890,
    895, 900, 905, 905, 905, 905, 900, 895,
    900, 900, 905, 905, 905, 905, 900, 895,
    890, 905, 905, 905, 905, 905, 900, 890,
    890, 900, 905, 900, 900, 900, 900, 890,
    880, 890, 890, 895, 895, 890, 890, 880,
];

pub static KING_OPENING_PIECE_SQUARE_TABLE: [i16; 64] = [
    19970, 19960, 19960, 19950, 19950, 19960, 19960, 19970,
    19970, 19960, 19960, 19950, 19950, 19960, 19960, 19970,
    19970, 19960, 19960, 19950, 19950, 19960, 19960, 19970,
    19970, 19960, 19960, 19950, 19950, 19960, 19960, 19970,
    19980, 19970, 19970, 19960, 19960, 19970, 19970, 19980,
    19990, 19980, 19980, 19980, 19980, 19980, 19980, 19990,
    20020, 20020, 20000, 20000, 20000, 20000, 20020, 20020,
    20020, 20030, 20010, 20000, 20000, 20010, 20030, 20020,
];

pub static KING_ENDGAME_PIECE_SQUARE_TABLE: [i16; 64] = [
    19950, 19960, 19970, 19980, 19980, 19970, 19960, 19950,
    19970, 19980, 19990, 20000, 20000, 19990, 19980, 19970,
    19970, 19990, 20020, 20030, 20030, 20020, 19990, 19970,
    19970, 19990, 20030, 20040, 20040, 20030, 19990, 19970,
    19970, 19990, 20030, 20040, 20040, 20030, 19990, 19970,
    19970, 19990, 20020, 20030, 20030, 20020, 19990, 19970,
    19970, 19970, 20000, 20000, 20000, 20000, 19970, 19970,
    19950, 19970, 19970, 19970, 19970, 19970, 19970, 19950,
];

pub fn is_endgame(board: &Board) -> bool {
    no_queens(board) || (has_queen_with_most_one_minor_piece(Color::White, board) && has_queen_with_most_one_minor_piece(Color::Black, board))
}

fn no_queens(board: &Board) -> bool {
    board.bb_piece(Piece::Queen) == 0
}

fn has_queen_with_most_one_minor_piece(color: Color, board: &Board) -> bool {
    let has_queen = board.bb(color, Piece::Queen) != 0;

    if has_queen {
        let pieces = board.bb(color, Piece::Knight) | board.bb(color, Piece::Bishop) | board.bb(color, Piece::Rook);
        let has_no_other_pieces = pieces == 0;
        let has_one_minor_piece = (pieces & !board.bb(color, Piece::Rook)).count_ones() <= 1;

        return has_no_other_pieces || has_one_minor_piece;
    }
    true
}

pub fn evaluate(board: &Board) -> i16 {
    let color = board.active_color();

    let pawn_eval = eval_piece_position(color, Piece::Pawn, &PAWN_PIECE_SQUARE_TABLE, board) - eval_piece_position(!color, Piece::Pawn, &PAWN_PIECE_SQUARE_TABLE, board);
    let knight_eval = eval_piece_position(color, Piece::Knight, &KNIGHT_PIECE_SQUARE_TABLE, board) - eval_piece_position(!color, Piece::Knight, &KNIGHT_PIECE_SQUARE_TABLE, board);
    let bishop_eval = eval_piece_position(color, Piece::Bishop, &BISHOP_PIECE_SQUARE_TABLE, board) - eval_piece_position(!color, Piece::Bishop, &BISHOP_PIECE_SQUARE_TABLE, board);
    let rook_eval = eval_piece_position(color, Piece::Rook, &ROOK_PIECE_SQUARE_TABLE, board) - eval_piece_position(!color, Piece::Rook, &ROOK_PIECE_SQUARE_TABLE, board);
    let queen_eval = eval_piece_position(color, Piece::Queen, &QUEEN_PIECE_SQUARE_TABLE, board) - eval_piece_position(!color, Piece::Queen, &QUEEN_PIECE_SQUARE_TABLE, board);
    
    let king_eval = if is_endgame(board) {
        eval_piece_position(color, Piece::King, &KING_ENDGAME_PIECE_SQUARE_TABLE, board) - eval_piece_position(!color, Piece::King, &KING_ENDGAME_PIECE_SQUARE_TABLE, board)
    } else {
        eval_piece_position(color, Piece::King, &KING_OPENING_PIECE_SQUARE_TABLE, board) - eval_piece_position(!color, Piece::King, &KING_OPENING_PIECE_SQUARE_TABLE, board)
    };

    pawn_eval + knight_eval + bishop_eval + rook_eval + queen_eval + king_eval
}

fn eval_piece_position(color:Color, piece: Piece, piece_square_table: &[i16; 64], board: &Board) -> i16 {
    let pieces = board.bb(color, piece);

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