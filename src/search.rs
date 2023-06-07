use crate::move_gen::MoveGenerator;
use crate::eval::evaluate;
use crate::board::Board;
use crate::moves::{Move, MoveType};
use crate::transposition::{TranspositionTable, Bounds};
use crate::zobrist::ZobristTable;
use crate::repetition::RepetitionTable;
use std::cmp::{max, min};

// Using i16 MIN and MAX to separate out mating moves
// There was an issue where the engine would not play the move that leads to mate
// as the move values were the same 
const NEG_INF: i32 = (std::i16::MIN + 1) as i32;
const INF: i32 = -NEG_INF;

const MATE_VALUE: i32 = std::i32::MAX - 1;

pub struct Searcher {
    move_gen: MoveGenerator,
    zobrist: ZobristTable,
    transposition_table: TranspositionTable,
    repetition_table: RepetitionTable,
}

impl Searcher {
    pub fn new() -> Self {
        Self {
            move_gen: MoveGenerator::new(),
            zobrist: ZobristTable::new(),
            transposition_table: TranspositionTable::new(),
            repetition_table: RepetitionTable::new(),
        }
    }

    pub fn best_move(&mut self, board: &Board, max_depth: u8) -> (i32, Option<Move>) {
        let mut best_move = None;
        let mut best_score = NEG_INF as i32;

        for depth in 1..max_depth+1 {
            (best_score, best_move) = self.negamax_alpha_beta(board, NEG_INF, INF, depth);

            let board_hash = self.zobrist.hash(board);
            self.transposition_table.store(board_hash, best_score, best_move, depth, Bounds::Lower);
        }
        (best_score, best_move)
    }

    fn negamax_alpha_beta(&mut self, board: &Board, alpha: i32, beta: i32, depth: u8) -> (i32, Option<Move>) {
        let original_alpha = alpha;
        let mut alpha = alpha;
        let mut beta = beta;

        let board_hash = self.zobrist.hash(board);

        // Check transposition table for an entry
        let tt_entry = self.transposition_table.retrieve(board_hash);
        let mut tt_best_move = None;
        
        // If the depth is lower, the TT move is still likely to be the best in the position
        // from iterative deepening, so we sort it first. We dont want to modidy alpha and beta though
        // unless the depth is greater or equal.
        if let Some(entry) = tt_entry {
            tt_best_move = entry.best_move; 
            if entry.depth >= depth {
                match entry.bounds {
                    Bounds::Exact => return (entry.eval, entry.best_move),
                    Bounds::Lower => alpha = max(alpha, entry.eval),
                    Bounds::Upper => beta = min(beta, entry.eval),
                }
                if alpha >= beta {
                    return (entry.eval, entry.best_move);
                }
            }
        }

        // Perform quiescence search, going through all captures, promotions, and checks
        if depth == 0 {
            return (self.quiescence(board, alpha, beta) as i32, None);
        }

        let mut moves = self.move_gen.generate_moves(board);
        sort_moves(board, &mut moves, tt_best_move);

        // Checkmate or Stalemate
        if moves.len() == 0 {
            if self.move_gen.attacks_to(board, self.move_gen.king_square(board)) != 0 {
                return (-MATE_VALUE + depth as i32, None);
            } else { 
                return (0, None);
            }
        }

        let mut best_score = NEG_INF as i32;
        let mut best_move = None;
        for mv in moves {
            let new_board = board.clone_with_move(&mv);
            let score = -self.negamax_alpha_beta(&new_board, -beta, -alpha, depth - 1).0;
            if score > best_score {
                best_score = score;
                best_move = Some(mv);
            }

            alpha = max(alpha, best_score);
            if alpha >= beta {
                break;
            }
        }

        // Get bound and store best move in TT
        let bound = if best_score <= original_alpha {
            Bounds::Upper
        } else if best_score >= beta {
            Bounds::Lower
        } else {
            Bounds::Exact
        };

        self.transposition_table.store(board_hash, best_score, best_move, depth, bound);

        return (best_score, best_move);
    }

    fn quiescence(&mut self, board: &Board, alpha: i32, beta: i32) -> i32 {
        let mut alpha = alpha;

        let king_in_check = self.move_gen.attacks_to(board, self.move_gen.king_square(board)) != 0;
        let mut moves = match king_in_check {
            true => self.move_gen.generate_moves(board),
            false => self.move_gen.generate_quiescence_moves(board),
        };

        mvv_lva_sort_moves(board, &mut moves);

        if moves.len() == 0 && king_in_check {
            return -MATE_VALUE as i32;
        }

        let stand_pat = evaluate(board) as i32;
        if stand_pat >= beta {
            return beta;
        }
        if alpha < stand_pat {
            alpha = stand_pat;
        }

        for mv in moves {
            let new_board = board.clone_with_move(&mv);
            let score = -self.quiescence(&new_board, -beta, -alpha);
            if score >= beta {
                return beta;
            }
            if score > alpha {
                alpha = score;
            }
        }
        return alpha;
    }

}

pub const MVV_LVA: [[i8; 6]; 6] = [
    [0, 0, 0, 0, 0, 0],       // victim K, attacker K, Q, R, B, N, P, None
    [50, 51, 52, 53, 54, 55], // victim Q, attacker K, Q, R, B, N, P, None
    [40, 41, 42, 43, 44, 45], // victim R, attacker K, Q, R, B, N, P, None
    [30, 31, 32, 33, 34, 35], // victim B, attacker K, Q, R, B, N, P, None
    [20, 21, 22, 23, 24, 25], // victim N, attacker K, Q, R, B, N, P, None
    [10, 11, 12, 13, 14, 15], // victim P, attacker K, Q, R, B, N, P, None
];

// TT entry best move -> MVV LVA moves -> everything else
pub fn sort_moves(board: &Board, moves: &mut [Move], tt_best_move: Option<Move>) {
    moves.sort_by_cached_key(|mv: &Move| {
        if let Some(tt_mv) = tt_best_move {
            if tt_mv == *mv {
                return std::i8::MIN;
            }
        }

        if mv.move_type == MoveType::EnPassant {
            return 0;
        } 

        let capturing_piece = board.get_piece_at(mv.from);
        let captured_piece = board.get_piece_at(mv.to);
        if captured_piece != None && capturing_piece != None {
            return -MVV_LVA[captured_piece.unwrap().index()][capturing_piece.unwrap().index()];
        }
        0
    })
}

// Used in quiescence search as we dont use TT move
pub fn mvv_lva_sort_moves(board: &Board, moves: &mut [Move]) {
    moves.sort_by_cached_key(|mv: &Move| {
        if mv.move_type == MoveType::EnPassant {
            return 0;
        } 

        let capturing_piece = board.get_piece_at(mv.from);
        let captured_piece = board.get_piece_at(mv.to);
        if captured_piece != None && capturing_piece != None {
            return -MVV_LVA[captured_piece.unwrap().index()][capturing_piece.unwrap().index()];
        }
        0
    })
}
