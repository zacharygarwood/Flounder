use crate::board::Board;
use crate::bitboard::{Bitboard, BitboardIterator, BitboardOperations, RANK_2, RANK_3, RANK_6, RANK_7};
use crate::table::Table;
use crate::pieces::{Piece, Color, PromotionPieceIterator};
use crate::moves::{Move, MoveType, NORTH, EAST, SOUTH, WEST};

pub struct MoveGenerator {
    lookup: Table
}

impl MoveGenerator {
    pub fn new() -> Self {
        Self {
            lookup: Table::init(),
        }
    }

    pub fn generate_moves(&self, board: &Board) -> Vec<Move>{
        let mut moves = Vec::new();
        
        // Generate moves for each piece type (pawns, knights, bishops, rooks, etc.)
        self.generate_psuedo_legal_pawn_moves(board, &mut moves);
        self.generate_psuedo_legal_moves(board, Piece::Knight, &mut moves);
        self.generate_psuedo_legal_moves(board, Piece::King, &mut moves);
    
        moves
    }
    
    fn generate_psuedo_legal_pawn_moves(&self, board: &Board, moves: &mut Vec<Move>) {
        use crate::pieces::Piece::*;
    
        let color = board.active_color();
        let pawns = board.bb(color, Pawn);
        let direction = PawnDirection::new(color);
    
        self.generate_quiet_pawn_pushes(board, pawns, direction, moves);
        self.generate_pawn_captures(board, pawns, direction, moves);
        self.generate_pawn_promotions(board, pawns, direction, moves);
    
        // TODO: En passant
    }
    
    fn generate_quiet_pawn_pushes(&self, board: &Board, pawns: Bitboard, direction: PawnDirection, moves: &mut Vec<Move>) {
        let pawns = pawns & !direction.rank_7;
        let empty_squares = board.bb_empty();
    
        // Generate single pawn pushes
        let single_pushes = pawns.shift(direction.north) & empty_squares;
    
        // Generate double pawn pushes
        let double_pawns = single_pushes & direction.rank_3;
        let double_pushes = double_pawns.shift(direction.north) & empty_squares;
    
        // Store moves
        self.extract_pawn_moves(single_pushes, direction.north, Piece::Pawn, MoveType::Quiet, moves);
        self.extract_pawn_moves(double_pushes, direction.north + direction.north, Piece::Pawn, MoveType::Quiet, moves)
    }
    
    fn generate_pawn_captures(&self, board: &Board, pawns: Bitboard, direction: PawnDirection, moves: &mut Vec<Move>) {
        let pawns = pawns & !direction.rank_7;
        let color = board.active_color();
    
        // Generate valid pawn attacks
        let enemy_pieces = board.bb_color(!color);
        let left_pawn_attacks = pawns.shift(direction.north + WEST) & enemy_pieces;
        let right_pawn_attacks = pawns.shift(direction.north + EAST) & enemy_pieces;
        
        // Store moves
        self.extract_pawn_moves(left_pawn_attacks, direction.north + WEST, Piece::Pawn, MoveType::Capture, moves);
        self.extract_pawn_moves(right_pawn_attacks, direction.north + EAST, Piece::Pawn, MoveType::Capture, moves);
    }

    fn generate_pawn_promotions(&self, board: &Board, pawns: Bitboard, direction: PawnDirection, moves: &mut Vec<Move>) {
        // Only look at pawns that can promote
        let pawns = pawns & direction.rank_7;
        let color = board.active_color();
        let enemy_pieces = board.bb_color(!color);
        let empty_squares = board.bb_empty();
    
        // Generate single pawn pushes
        let single_pushes = pawns.shift(direction.north) & empty_squares;

        // Generate valid pawn attacks
        let left_pawn_attacks = pawns.shift(direction.north + WEST) & enemy_pieces;
        let right_pawn_attacks = pawns.shift(direction.north + EAST) & enemy_pieces;
        
        // Store moves
        self.extract_pawn_promotions(single_pushes, direction.north, MoveType::Promotion, moves);
        self.extract_pawn_promotions(left_pawn_attacks, direction.north + WEST, MoveType::Promotion, moves);
        self.extract_pawn_promotions(right_pawn_attacks, direction.north + EAST, MoveType::Promotion, moves);
    }
    
    // Pawns use an offset to find where they came from. piece_type is used for promotions for pawns.
    fn extract_pawn_moves(&self, mut bitboard: Bitboard, offset: i8, piece_type: Piece, move_type: MoveType, moves: &mut Vec<Move>) {
        let iter = BitboardIterator::new(bitboard);
        for square in iter {
            let m = Move {
                to: square,
                from: (square as i8 - offset) as u8,
                piece_type,
                move_type,
            };
            moves.push(m);
        }
    }

    fn extract_pawn_promotions(&self, mut bitboard: Bitboard, offset: i8, move_type: MoveType, moves: &mut Vec<Move>) {
        let bb_iter = BitboardIterator::new(bitboard);
        let promotion_pieces = PromotionPieceIterator::new();
        for square in bb_iter {
            for piece in promotion_pieces.clone() {
                let m = Move {
                    to: square,
                    from: (square as i8 - offset) as u8,
                    piece_type: piece,
                    move_type,
                };
                moves.push(m);
            }
        }
    }
    
    fn generate_psuedo_legal_moves(&self, board: &Board, piece: Piece, moves: &mut Vec<Move>) {
        let color = board.active_color();
        let pieces = board.bb(color, piece);
        let enemy_pieces = board.bb_color(!color);
        let empty_squares = board.bb_empty();
    
        let iter = BitboardIterator::new(pieces);
        for square in iter {
            let destinations = self.lookup.moves(square, piece);

            let quiet_moves = destinations & empty_squares;
            let capture_moves = destinations & enemy_pieces;

            self.extract_moves(quiet_moves, square, piece, MoveType::Quiet, moves);
            self.extract_moves(capture_moves, square, piece, MoveType::Capture, moves);
        }
    }
    
    fn extract_moves(&self, mut bitboard: Bitboard, from: u8, piece_type:Piece, move_type: MoveType, moves: &mut Vec<Move>) {
        let iter = BitboardIterator::new(bitboard);
        for square in iter {
            let m = Move {
                to: square,
                from,
                piece_type,
                move_type,
            };
            moves.push(m);
        }
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