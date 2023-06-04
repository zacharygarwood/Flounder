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

    pub fn retrieve(&self, key: u64, depth: u8) -> Option<&Entry> {
        let entry = self.table.get(&key);
        if entry.is_some() && entry.unwrap().hash_key == key && entry.unwrap().depth >= depth {
            return entry;
        }
        return None;
    }
}

pub struct Entry {
    pub hash_key: u64,
    pub eval: i32,
    pub best_move: Option<Move>,
    pub depth: u8,
    pub bounds: Bounds,
}

pub enum Bounds {
    Exact,
    Lower,
    Upper,
}