use std::collections::HashMap;
use crate::moves::Move;

pub struct TranspositionTable {
    table: HashMap<u64, Entry>,
}

impl TranspositionTable {
    pub fn new() -> Self {
        Self {
            table: HashMap::new(),
        }
    }

    pub fn store(&mut self, hash_key: u64, eval: i32, best_move: Option<Move>, depth: u8, bounds: Bounds) {
        let entry = Entry {
            hash_key,
            eval,
            best_move,
            depth,
            bounds,
        };

        // Depth-Preferred Replacement
        let prev_entry = self.table.get(&hash_key);
        if prev_entry.is_none() {
            self.table.insert(hash_key, entry);
        } else if prev_entry.is_some() && prev_entry.unwrap().depth <= depth {
            self.table.insert(hash_key, entry);
        }
    }

    pub fn retrieve(&self, key: u64) -> Option<&Entry> {
        let entry = self.table.get(&key);
        if entry.is_some() && entry.unwrap().hash_key == key {
            return entry;
        }
        return None;
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Entry {
    pub hash_key: u64,
    pub eval: i32,
    pub best_move: Option<Move>,
    pub depth: u8,
    pub bounds: Bounds,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Bounds {
    Exact,
    Lower,
    Upper,
}

#[cfg(test)]
mod tests {
    use crate::transposition::{TranspositionTable, Bounds};
    use crate::zobrist::ZobristTable;
    use crate::pieces::Piece;
    use crate::moves::{Move, MoveType};
    use crate::board::Board;

    #[test]
    fn retrieve_position_in_table() {
        let mut tt = TranspositionTable::new();
        let zobrist = ZobristTable::new();
        let board = Board::new("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");

        let stored_eval = 150;
        let stored_mv = Move::new(8, 16, Piece::Pawn, MoveType::Quiet);
        let stored_depth = 6;
        let stored_bounds = Bounds::Exact;

        tt.store(zobrist.hash(&board), stored_eval, Some(stored_mv), stored_depth, stored_bounds);

        let entry = tt.retrieve(zobrist.hash(&board));

        assert_eq!(stored_eval, entry.unwrap().eval);
        assert_eq!(stored_mv, entry.unwrap().best_move.unwrap());
        assert_eq!(stored_depth, entry.unwrap().depth);
        assert_eq!(stored_bounds, entry.unwrap().bounds);
    }

    #[test]
    fn retrieve_position_not_in_table() {
        let tt = TranspositionTable::new();
        let zobrist = ZobristTable::new();
        let board = Board::new("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");

        let entry = tt.retrieve(zobrist.hash(&board));

        assert_eq!(entry, None);
    }

    #[test]
    fn update_entry_with_new_greater_depth() {
        let mut tt = TranspositionTable::new();
        let zobrist = ZobristTable::new();
        let board = Board::new("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");

        let lower_depth_eval = 150;
        let lower_depth_mv = Move::new(8, 16, Piece::Pawn, MoveType::Quiet);
        let lower_depth_depth = 6;
        let lower_depth_bounds = Bounds::Exact;

        let greater_depth_eval = 100;
        let greater_depth_mv = Move::new(9, 17, Piece::Pawn, MoveType::Quiet);
        let greater_depth_depth = 7;
        let greater_depth_bounds = Bounds::Lower;

        // Store entry with lower depth first, then store entry with greater depth
        // lower depth should be replaced with greater depth
        tt.store(zobrist.hash(&board), lower_depth_eval, Some(lower_depth_mv), lower_depth_depth, lower_depth_bounds);
        tt.store(zobrist.hash(&board), greater_depth_eval, Some(greater_depth_mv), greater_depth_depth, greater_depth_bounds);

        let entry = tt.retrieve(zobrist.hash(&board));

        assert_eq!(greater_depth_eval, entry.unwrap().eval);
        assert_eq!(greater_depth_mv, entry.unwrap().best_move.unwrap());
        assert_eq!(greater_depth_depth, entry.unwrap().depth);
        assert_eq!(greater_depth_bounds, entry.unwrap().bounds);
    }

    #[test]
    fn keep_entry_with_old_greater_depth() {
        let mut tt = TranspositionTable::new();
        let zobrist = ZobristTable::new();
        let board = Board::new("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");

        let lower_depth_eval = 150;
        let lower_depth_mv = Move::new(8, 16, Piece::Pawn, MoveType::Quiet);
        let lower_depth_depth = 6;
        let lower_depth_bounds = Bounds::Exact;

        let greater_depth_eval = 100;
        let greater_depth_mv = Move::new(9, 17, Piece::Pawn, MoveType::Quiet);
        let greater_depth_depth = 7;
        let greater_depth_bounds = Bounds::Lower;

        // Store entry with greater depth first, then store entry with lower depth
        // greater depth should not be replaced with lower depth
        tt.store(zobrist.hash(&board), greater_depth_eval, Some(greater_depth_mv), greater_depth_depth, greater_depth_bounds);
        tt.store(zobrist.hash(&board), lower_depth_eval, Some(lower_depth_mv), lower_depth_depth, lower_depth_bounds);

        let entry = tt.retrieve(zobrist.hash(&board));

        assert_eq!(greater_depth_eval, entry.unwrap().eval);
        assert_eq!(greater_depth_mv, entry.unwrap().best_move.unwrap());
        assert_eq!(greater_depth_depth, entry.unwrap().depth);
        assert_eq!(greater_depth_bounds, entry.unwrap().bounds);
    }
    
}