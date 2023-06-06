use std::collections::HashMap;

pub struct RepetitionTable {
    pub position_counts: HashMap<u64, u32>,
}

impl RepetitionTable {
    pub fn new() -> Self {
        Self {
            position_counts: HashMap::new(),
        }
    }

    pub fn is_threefold_repetition(&self, hash_key: u64) -> bool {
        let count = self.position_counts.get(&hash_key).unwrap_or(&0);
        *count >= 3
    }

    pub fn increment_position_count(&mut self, hash_key: u64) {
        let count = self.position_counts.entry(hash_key).or_insert(0);
        *count += 1;
    }

    pub fn decrement_position_count(&mut self, hash_key: u64) {
        let count = self.position_counts.get_mut(&hash_key).unwrap();
        *count -= 1;
    }

}