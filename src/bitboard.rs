pub type Bitboard = u64;

pub const FILES: usize = 8;
pub const RANKS: usize = 8;

pub const RANK_1: Bitboard = 0x00000000000000FF;
pub const RANK_2: Bitboard = RANK_1 << 8;
pub const RANK_3: Bitboard = RANK_2 << 8;
pub const RANK_4: Bitboard = RANK_3 << 8;
pub const RANK_5: Bitboard = RANK_4 << 8;
pub const RANK_6: Bitboard = RANK_5 << 8;
pub const RANK_7: Bitboard = RANK_6 << 8;
pub const RANK_8: Bitboard = RANK_7 << 8;

pub const FILE_A: Bitboard = 0x8080808080808080;
pub const FILE_B: Bitboard = FILE_A << 1;
pub const FILE_C: Bitboard = FILE_B << 1;
pub const FILE_D: Bitboard = FILE_C << 1;
pub const FILE_E: Bitboard = FILE_D << 1;
pub const FILE_F: Bitboard = FILE_E << 1;
pub const FILE_G: Bitboard = FILE_F << 1;
pub const FILE_H: Bitboard = FILE_G << 1;