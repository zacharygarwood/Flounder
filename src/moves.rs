use crate::board::Board;
use crate::bitboard::{Bitboard, BitboardIterator, Shift, RANKS, FILES, RANK_2, RANK_3, RANK_6, RANK_7, FILE_A, FILE_H};
use crate::pieces::{PieceType, Color};

pub const NORTH: i8 = 8;
pub const EAST: i8 = 1;
pub const SOUTH: i8 = -NORTH;
pub const WEST: i8 = -EAST;

#[derive(Debug)]
pub struct Move {
    pub from: u8,
    pub to: u8,
    pub move_type: MoveType,
}

impl Move {
    pub fn new(from: u8, to: u8, move_type: MoveType) -> Self {
        Self {
            from,
            to,
            move_type,
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub enum MoveType {
    Quiet,    // Non-capturing move
    Capture,  // Capturing move 
    EnPassant,
    Castle,
}

impl std::fmt::Display for MoveType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let piece_str = match self {
            MoveType::Quiet => "Quiet",
            MoveType::Capture => "Capture",
            MoveType::EnPassant => "En Passant",
            MoveType::Castle => "Castle",
        };
        write!(f, "{}", piece_str)
    }
}

pub fn generate_moves(board: &Board) -> Vec<Move>{

    let mut moves = Vec::new();
    // Generate moves for each piece type (pawns, knights, bishops, rooks, etc.)
    generate_psuedo_legal_pawn_moves(board, &mut moves);

    // Generate moves for other piece types

    moves
}

fn generate_psuedo_legal_pawn_moves(board: &Board, moves: &mut Vec<Move>) {
    use crate::pieces::{PieceType::*, Color::*};

    let color = board.active_color();
    let pawns = board.bb(color, Pawn);
    let direction = PawnDirection::new(color);

    generate_quiet_pawn_pushes(board, pawns, direction, moves);
    generate_pawn_captures(board, pawns, direction, moves);
}

fn generate_quiet_pawn_pushes(board: &Board, pawns: Bitboard, direction: PawnDirection, moves: &mut Vec<Move>) {
    let pawns = pawns & !direction.rank_7;
    let empty_squares = board.bb_empty();

    // Generate single pawn pushes
    let single_pushes = pawns.shift(direction.north) & empty_squares;

    // Generate double pawn pushes
    let double_pawns = single_pushes & direction.rank_3;
    let double_pushes = double_pawns.shift(direction.north) & empty_squares;

    // Store moves
    extract_pawn_moves(single_pushes, direction.north, MoveType::Quiet, moves);
    extract_pawn_moves(double_pushes, direction.north + direction.north, MoveType::Quiet, moves)
}

fn generate_pawn_captures(board: &Board, pawns: Bitboard, direction: PawnDirection, moves: &mut Vec<Move>) {
    let pawns = pawns & !direction.rank_7;
    let color = board.active_color();

    // Generate valid pawn attacks
    let enemy_pieces = board.bb_color(!color);
    let left_pawn_attacks = pawns.shift(direction.north + WEST) & enemy_pieces;
    let right_pawn_attacks = pawns.shift(direction.north + EAST) & enemy_pieces;
    
    // Store moves
    extract_pawn_moves(left_pawn_attacks, direction.north + WEST, MoveType::Capture, moves);
    extract_pawn_moves(right_pawn_attacks, direction.north + EAST, MoveType::Capture, moves);


}

fn extract_pawn_moves(mut bitboard: Bitboard, offset: i8, move_type: MoveType, moves: &mut Vec<Move>) {
    let iter = BitboardIterator::new(bitboard);
    for (square, _) in iter {
        let m = Move {
            to: square,
            from: (square as i8 - offset) as u8,
            move_type,
        };
        moves.push(m);
    }
}

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