use std::collections::HashMap;
use crate::moves::Move;

pub struct TranspositionTable {
    table: HashMap<u64, Entry>,
}

impl TranspositionTable {
    fn new() -> Self {
        Self {
            table: HashMap::new(),
        }
    }

    fn store(&mut self, key: u64, entry: Entry) {
        self.table.insert(key, entry);
    }

    fn retrieve(&self, key: u64) -> Option<&Entry> {
        self.table.get(&key)
    }
}

struct Entry {
    hash_key: u64,
    eval: i32,
    best_move: Option<Move>,
    depth: u8,
    bounds: Bounds,
}

enum Bounds {
    Exact,
    LowerBound,
    UpperBound,
}