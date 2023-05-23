use crate::board::Board;
use crate::bitboard::{Bitboard, BitboardIterator, BitboardOperations, RANK_2, RANK_3, RANK_6, RANK_7, WHITE_KING_SIDE, WHITE_QUEEN_SIDE, BLACK_KING_SIDE, BLACK_QUEEN_SIDE};
use crate::table::Table;
use crate::pieces::{Piece, Color, PromotionPieceIterator};
use crate::moves::{Move, MoveType, NORTH, EAST, SOUTH, WEST};
use crate::square::{Square, C1, C8, E1, E8, G1, G8};

pub struct MoveGenerator {
    pub lookup: Table
}

impl MoveGenerator {
    pub fn new() -> Self {
        Self {
            lookup: Table::init(),
        }
    }

    pub fn generate_moves(&self, board: &Board) -> Vec<Move>{
        let mut moves = Vec::new();
        
        // Generate moves for each piece type
        self.generate_pseudo_legal_castles(board, &mut moves);
        self.generate_pseudo_legal_pawn_moves(board, &mut moves);
        self.generate_pseudo_legal_moves(board, Piece::King, &mut moves);
        self.generate_pseudo_legal_moves(board, Piece::Knight, &mut moves);
        self.generate_pseudo_legal_moves(board, Piece::Bishop, &mut moves);
        self.generate_pseudo_legal_moves(board, Piece::Rook, &mut moves);
        self.generate_pseudo_legal_moves(board, Piece::Queen, &mut moves);
    
        moves
    }
    
    fn generate_pseudo_legal_pawn_moves(&self, board: &Board, moves: &mut Vec<Move>) {
        use crate::pieces::Piece::*;
    
        let color = board.active_color();
        let pawns = board.bb(color, Pawn);
        let direction = PawnDirection::new(color);
    
        self.generate_quiet_pawn_pushes(board, pawns, direction, moves);
        self.generate_pawn_captures(board, pawns, direction, moves);
        self.generate_en_passants(board, pawns, direction, moves);
        self.generate_promotions(board, pawns, direction, moves);
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
        self.extract_pawn_moves(single_pushes, direction.north, MoveType::Quiet, moves);
        self.extract_pawn_moves(double_pushes, direction.north + direction.north, MoveType::Quiet, moves);
    }
    
    fn generate_pawn_captures(&self, board: &Board, pawns: Bitboard, direction: PawnDirection, moves: &mut Vec<Move>) {
        let pawns = pawns & !direction.rank_7;
        let color = board.active_color();
    
        // Generate valid pawn attacks
        let enemy_pieces = board.bb_color(!color);
        let left_pawn_attacks = pawns.shift(direction.north + WEST) & enemy_pieces;
        let right_pawn_attacks = pawns.shift(direction.north + EAST) & enemy_pieces;
        
        // Store moves
        self.extract_pawn_moves(left_pawn_attacks, direction.north + WEST, MoveType::Capture, moves);
        self.extract_pawn_moves(right_pawn_attacks, direction.north + EAST, MoveType::Capture, moves);
    }

    fn generate_en_passants(&self, board: &Board, pawns: Bitboard, direction: PawnDirection, moves: &mut Vec<Move>) {
        // Bitboard with en passant target set, or empty
        let en_passant_target = match board.en_passant_target {
            Some(square) => Bitboard::square_to_bitboard(square),
            None => Bitboard::empty(),
        };

        // Generate valid pawn en passant attacks
        let left_pawn_attacks = pawns.shift(direction.north + WEST) & en_passant_target;
        let right_pawn_attacks = pawns.shift(direction.north + EAST) & en_passant_target;

        // Store moves
        self.extract_pawn_moves(left_pawn_attacks, direction.north + WEST, MoveType::EnPassant, moves);
        self.extract_pawn_moves(right_pawn_attacks, direction.north + EAST, MoveType::EnPassant, moves);

    }

    fn generate_promotions(&self, board: &Board, pawns: Bitboard, direction: PawnDirection, moves: &mut Vec<Move>) {
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
        self.extract_promotions(single_pushes, direction.north, MoveType::Promotion, moves);
        self.extract_promotions(left_pawn_attacks, direction.north + WEST, MoveType::Promotion, moves);
        self.extract_promotions(right_pawn_attacks, direction.north + EAST, MoveType::Promotion, moves);
    }
    
    fn extract_pawn_moves(&self, bitboard: Bitboard, offset: i8, move_type: MoveType, moves: &mut Vec<Move>) {
        let iter = BitboardIterator::new(bitboard);
        for square in iter {
            let mv = Move::new(square, (square as i8 - offset) as u8, Piece::Pawn, move_type);
            moves.push(mv);
        }
    }

    fn extract_promotions(&self, bitboard: Bitboard, offset: i8, move_type: MoveType, moves: &mut Vec<Move>) {
        let bb_iter = BitboardIterator::new(bitboard);
        let promotion_pieces = PromotionPieceIterator::new();
        for square in bb_iter {
            for piece in promotion_pieces.clone() {
                let mv = Move::new(square, (square as i8 - offset) as u8, piece, move_type);
                moves.push(mv);
            }
        }
    }

    fn generate_pseudo_legal_castles(&self, board: &Board, moves: &mut Vec<Move>) {
        let color = board.active_color();
        let all_pieces = board.bb_all();
        let (king_side_rights, queen_side_rights) = board.castling_ability(color);

        let (king_side_mask, queen_side_mask) = match color {
            Color::White => (WHITE_KING_SIDE, WHITE_QUEEN_SIDE),
            Color::Black => (BLACK_KING_SIDE, BLACK_QUEEN_SIDE),
        };

        let king_side_occupancy = king_side_mask & all_pieces;
        let queen_side_occupancy = queen_side_mask & all_pieces;

        // Castle king side if they have the rights and nothing blocks
        if king_side_rights && king_side_occupancy == 0 {
            self.extract_castles(color, Piece::King, MoveType::Castle, moves);
        }

        // Castle queen side if they have the rights and nothing blocks
        if queen_side_rights && queen_side_occupancy == 0 {
            self.extract_castles(color, Piece::Queen, MoveType::Castle, moves);
        }
    }

    fn extract_castles(&self, color: Color, side_to_castle: Piece, move_type: MoveType, moves: &mut Vec<Move>) {
        let (starting_square, king_side_square, queen_side_square) = match color {
            Color::White => (E1, G1, C1),
            Color::Black => (E8, G8, C8),
        };

        match side_to_castle {
            Piece::King => {
                let mv = Move::new(king_side_square, starting_square as u8, Piece::King, move_type);
                moves.push(mv);
            },
            Piece::Queen => {
                let mv = Move::new(queen_side_square, starting_square as u8, Piece::King, move_type);
                moves.push(mv);
            },
            _ => {} // Only care about King and Queen for king side and queen side castling respectively
        };
    }
    
    fn generate_pseudo_legal_moves(&self, board: &Board, piece: Piece, moves: &mut Vec<Move>) {
        let color = board.active_color();
        let pieces = board.bb(color, piece);
        let enemy_pieces = board.bb_color(!color);
        let empty_squares = board.bb_empty();
    
        let iter = BitboardIterator::new(pieces);
        for square in iter {
            let destinations = match piece {
                Piece:: Knight | Piece::King => self.lookup.non_sliding_moves(square, piece),
                _ => self.lookup.sliding_moves(square, board.bb_all(), piece)
            };

            let quiet_moves = destinations & empty_squares;
            let capture_moves = destinations & enemy_pieces;

            self.extract_moves(quiet_moves, square, piece, MoveType::Quiet, moves);
            self.extract_moves(capture_moves, square, piece, MoveType::Capture, moves);
        }
    }
    
    fn extract_moves(&self, bitboard: Bitboard, from: u8, piece_type:Piece, move_type: MoveType, moves: &mut Vec<Move>) {
        let iter = BitboardIterator::new(bitboard);
        for square in iter {
            let mv = Move::new(square, from, piece_type, move_type);
            moves.push(mv);
        }
    }

    // Returns a bitboard with all pieces attacking a certain square
    pub fn attacks_to(&self, square: Square, board: Board) -> Bitboard {
        let color = board.active_color();
        let occupancy = board.bb_all() & !board.bb(color, Piece::King);

        // Get all attacks from square
        let pawn_attacks = Self::pawn_attacks_to(square, color);
        let knight_attacks = self.lookup.non_sliding_moves(square, Piece::Knight);
        let bishop_attacks = self.lookup.sliding_moves(square, occupancy, Piece::Bishop);
        let rook_attacks = self.lookup.sliding_moves(square, occupancy, Piece::Rook);
        let king_attacks = self.lookup.non_sliding_moves(square, Piece::King);
        let queen_attacks = bishop_attacks | rook_attacks;

        // Get relevant pieces that can attack the square
        let pawns = pawn_attacks & board.bb_piece(Piece::Pawn);
        let knights = knight_attacks & board.bb_piece(Piece::Knight);
        let bishops = bishop_attacks & board.bb_piece(Piece::Bishop);
        let rooks = rook_attacks & board.bb_piece(Piece::Rook);
        let king = king_attacks & board.bb_piece(Piece::King);
        let queens = queen_attacks & board.bb_piece(Piece::Queen);

        // Get only the pieces for the opponent
        (pawns | knights | bishops | rooks | king | queens) & board.bb_color(!color)
    }

    fn king_square(board: &Board) -> Square {
        let color = board.active_color();
        board.bb(color, Piece::King).trailing_zeros() as Square
    }

    fn pawn_attacks_to(square: Square, color: Color) -> Bitboard {
        let bb = Bitboard::square_to_bitboard(square);
        match color {
            Color::White => bb.shift(NORTH + WEST) | bb.shift(NORTH + EAST),
            Color::Black => bb.shift(SOUTH + WEST) | bb.shift(SOUTH + EAST),
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