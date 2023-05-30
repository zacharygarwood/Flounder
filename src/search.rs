use crate::move_gen::MoveGenerator;
use crate::eval::{evaluate, self};
use crate::board::Board;
use crate::moves::Move;

// Using i32 MIN and MAX to separate out mating moves
// There was an issue where the engine would not play the move that leads to mate
// as the move values were the same 
// const INITIAL_ALPHA: isize = (std::i32::MIN) as isize + 1;
// const INITIAL_BETA: isize = (std::i32::MAX) as isize - 1;

const INITIAL_ALPHA: isize = std::isize::MIN + 1;
const INITIAL_BETA: isize = std::isize::MAX - 1;

const MATE_VALUE: isize = std::isize::MIN + 1;

pub struct Searcher {
    move_gen: MoveGenerator
}

impl Searcher {
    pub fn new() -> Self {
        Self {
            move_gen: MoveGenerator::new(),
        }
    }

    pub fn best_move(&self, board: &Board, depth: usize) -> (isize, Option<Move>) {
        let moves = self.move_gen.generate_moves(board);
        let mut best_move = None;
        let mut best_score = std::isize::MIN + 1;

        for mv in moves {
            let new_board = board.clone_with_move(&mv);
            let score = -self.negamax_alpha_beta(&new_board, INITIAL_ALPHA, INITIAL_BETA, depth);
            if score > best_score {
                best_move = Some(mv);
                best_score = score;
            }
        }

        (best_score, best_move)
    }
    fn negamax_alpha_beta(&self, board: &Board, mut alpha: isize, beta: isize, depth: usize) -> isize {
        if depth == 0 {
            return evaluate(board);
        }

        let moves = self.move_gen.generate_moves(board);

        if moves.len() == 0 {
            if self.move_gen.attacks_to(board, self.move_gen.king_square(board)) != 0 {
                return MATE_VALUE;
            } else {
                return 0;
            }
        }

        for mv in moves {
            let new_board = board.clone_with_move(&mv);
            let score = -self.negamax_alpha_beta(&new_board, -beta, -alpha, depth - 1);
            if score >= beta {
                return beta;
            }
            if score > alpha {
                alpha = score;
            }
        }
        alpha
    }
}
