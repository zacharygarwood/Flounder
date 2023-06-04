use crate::move_gen::MoveGenerator;
use crate::eval::evaluate;
use crate::board::Board;
use crate::moves::{Move, MoveType};
use crate::transposition::{TranspositionTable, Entry, Bounds};
use crate::zobrist::ZobristTable;
use std::cmp::{max, min};

// Using i16 MIN and MAX to separate out mating moves
// There was an issue where the engine would not play the move that leads to mate
// as the move values were the same 
const INITIAL_ALPHA: i16 = std::i16::MIN + 1;

const MATE_VALUE: i32 = std::i32::MIN + 1;

pub struct Searcher {
    move_gen: MoveGenerator,
    zobrist: ZobristTable,
    transposition_table: TranspositionTable,
}

impl Searcher {
    pub fn new() -> Self {
        Self {
            move_gen: MoveGenerator::new(),
            zobrist: ZobristTable::new(),
            transposition_table: TranspositionTable::new(),
        }
    }

    pub fn best_move_negamax_ab(&mut self, board: &Board, depth: u8) -> (i32, Option<Move>) {
        let mut moves = self.move_gen.generate_moves(board);
        let mut best_move = None;
        let mut best_score = INITIAL_ALPHA as i32;
        let mut tt_best_move = None;

        let board_hash = self.zobrist.hash(board);
        let tt_entry = self.transposition_table.retrieve(board_hash, depth);
        if let Some(entry) = tt_entry {
            tt_best_move = entry.best_move;
        }

        sort_moves(board, &mut moves, tt_best_move);

        for mv in moves {
            let new_board = board.clone_with_move(&mv);
            let score = -self.negamax_alpha_beta(&new_board, INITIAL_ALPHA as i32, -best_score, depth - 1);
            if score > best_score {
                best_move = Some(mv);
                best_score = score;
            }
        }

        (best_score, best_move)
    }

    fn negamax_alpha_beta(&mut self, board: &Board, alpha: i32, beta: i32, depth: u8) -> i32 {
        let mut alpha = alpha;
        let mut beta = beta;
        let mut tt_best_move = None;

        // Check transposition table for an entry
        let board_hash = self.zobrist.hash(board);
        let tt_entry = self.transposition_table.retrieve(board_hash, depth);
        if let Some(entry) = tt_entry {
            tt_best_move = entry.best_move;
            match entry.bounds {
                Bounds::Exact => return entry.eval,
                Bounds::Lower => alpha = max(alpha, entry.eval),
                Bounds::Upper => beta = min(beta, entry.eval),
            }
            if alpha >= beta {
                return entry.eval;
            }
        }

        if depth == 0 {
            let score = self.quiescence(board, alpha, beta, depth);
            self.transposition_table.store(board_hash, score,  None, depth, Bounds::Exact);

            return score;
        }

        let mut moves = self.move_gen.generate_moves(board);
        sort_moves(board, &mut moves, tt_best_move);

        // Checkmate or Stalemate
        if moves.len() == 0 {
            if self.move_gen.attacks_to(board, self.move_gen.king_square(board)) != 0 {
                let score = MATE_VALUE + depth as i32;
                self.transposition_table.store(board_hash, score,  None, depth, Bounds::Exact);

                return score;
            } else { 
                return 0;
            }
        }

        for mv in moves {
            let new_board = board.clone_with_move(&mv);
            let score = -self.negamax_alpha_beta(&new_board, -beta, -alpha, depth - 1);
            if score >= beta {
                self.transposition_table.store(board_hash, beta,  Some(mv), depth, Bounds::Lower);

                return beta;
            }
            if score > alpha {
                alpha = score;
            }
        }

        self.transposition_table.store(board_hash, alpha, None, depth, Bounds::Upper);

        return alpha;
    }

    fn quiescence(&mut self, board: &Board, alpha: i32, beta: i32, depth: u8) -> i32 {
        let mut alpha = alpha;
        let mut beta = beta;
        let mut tt_best_move = None;

        let stand_pat = evaluate(board) as i32;

        if stand_pat >= beta {
            return beta;
        }
        if alpha < stand_pat {
            alpha = stand_pat;
        }

        let board_hash = self.zobrist.hash(board);
        let tt_entry = self.transposition_table.retrieve(board_hash, depth);
        if let Some(entry) = tt_entry {
            tt_best_move = entry.best_move;
            match entry.bounds {
                Bounds::Exact => return entry.eval,
                Bounds::Lower => alpha = max(alpha, entry.eval),
                Bounds::Upper => beta = min(beta, entry.eval),
            }
            if alpha >= beta {
                return entry.eval;
            }
        }

        let mut moves = self.move_gen.generate_captures(board);
        sort_moves(board, &mut moves, tt_best_move);

        for mv in moves {
            let new_board = board.clone_with_move(&mv);
            let score = -self.quiescence(&new_board, -beta, -alpha, depth + 1);

            if score >= beta {
                self.transposition_table.store(board_hash, beta,  Some(mv), depth, Bounds::Lower);
                return beta;
            }
            if score > alpha {
                alpha = score;
            }
        }

        self.transposition_table.store(board_hash, alpha,  None, depth, Bounds::Upper);

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
