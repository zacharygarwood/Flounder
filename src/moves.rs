use crate::board::Board;
use crate::bitboard::{Bitboard, RANKS, FILES, RANK_2, RANK_3, RANK_6, RANK_7, FILE_A, FILE_H};
use crate::pieces::{PieceType, Color};

pub const NORTH: i8 = 8;
pub const EAST: i8 = 1;
pub const SOUTH: i8 = -NORTH;
pub const WEST: i8 = -EAST;

#[derive(Debug)]
pub struct Move {
    pub from: u8,
    pub to: u8,
}

impl Move {
    pub fn new(from: u8, to: u8) -> Self {
        Self {
            from,
            to,
        }
    }
}

#[derive(Debug)]
pub enum MoveType {
    Quiet,    // Non-capturing move
    Capture,  // Capturing move 
    EnPassant,
    Castle,
}

pub fn generate_moves(board: &Board) -> Vec<u64>{

    let mut moves = Vec::new();
    // Generate moves for each piece type (pawns, knights, bishops, rooks, etc.)
    generate_psuedo_legal_pawn_moves(board, &mut moves);

    // Generate moves for other piece types

    moves
}

fn generate_psuedo_legal_pawn_moves(board: &Board, moves: &mut Vec<u64>) {
    use crate::pieces::{PieceType::*, Color::*};

    let color = board.active_color();
    let pawns = board.bb(color, PieceType::Pawn);
    let direction = PawnDirection::new(color);

    generate_quiet_pawn_pushes(board, pawns, direction, moves);
    generate_pawn_captures(board, pawns, direction, moves);
}

fn generate_quiet_pawn_pushes(board: &Board, pawns: Bitboard, direction: PawnDirection, moves: &mut Vec<u64>) {
    let pawns = pawns & !direction.rank_7;
    let empty_squares = board.bb_empty();

    // Generate single pawn pushes
    let single_pushes = (pawns << direction.north) & empty_squares;

    // Generate double pawn pushes
    let double_pawns = single_pushes & direction.rank_3;
    let double_pushes = (double_pawns << direction.north) & empty_squares;
    moves.push(single_pushes);
    moves.push(double_pushes);

    // TODO: Extract moves from bitboards
}

fn generate_pawn_captures(board: &Board, pawns: Bitboard, direction: PawnDirection, moves: &mut Vec<u64>) {
    let pawns = pawns & !direction.rank_7;
    let color = board.active_color();

    // FIXME: Change this to not be bad
    let pawn_attacks = match color {
        Color::White => ((pawns & !FILE_H) << NORTH + WEST) | ((pawns & !FILE_A) << NORTH + EAST),
        Color::Black => ((pawns & !FILE_H) >> NORTH + EAST ) | ((pawns & !FILE_A) >> NORTH + WEST),
    };
    let enemy_pieces = match color {
        Color::White => board.bb_color(Color::Black),
        Color::Black => board.bb_color(Color::White),
    };

    let valid_pawn_attacks = pawn_attacks & enemy_pieces;
    moves.push(valid_pawn_attacks);
}

// fn extract_moves(mut bitboard: Bitboard, offset: i8) -> Vec<Move> {
//     let moves: Vec<Move> = Vec::new();
//     while bitboard != 0 {
//         let index = bitboard.trailing_zeros() as u8;
//         bitboard = bitboard.clear_bit(index);
//         let m = Move {
//             to: index,
//             from: index, 
//         };
//         moves.push(m);
//     }
//     moves
// }

#[derive(Copy, Clone)]
struct PawnDirection {
    rank_7: Bitboard,
    rank_3: Bitboard,
    north: i8,
}

impl PawnDirection {
    fn new(color: Color) -> Self {
        let rank_7 = match color {
            Color::White => RANK_7,
            Color::Black => RANK_2,
        };
        let rank_3 = match color {
            Color::White => RANK_3,
            Color::Black => RANK_6,
        };
        let north = match color {
            Color::White => NORTH,
            Color::Black => SOUTH,
        };
        Self { rank_7, rank_3, north }
    }
}