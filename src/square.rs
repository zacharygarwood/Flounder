pub type Square = u8;

// Castling squares
pub const A1: Square = 0;
pub const C1: Square = 2;
pub const D1: Square = 3;
pub const E1: Square = 4;
pub const F1: Square = 5;
pub const G1: Square = 6;
pub const H1: Square = 7;

pub const A8: Square = 56;
pub const C8: Square = 58;
pub const D8: Square = 59;
pub const E8: Square = 60;
pub const F8: Square = 61;
pub const G8: Square = 62;
pub const H8: Square = 63;

pub fn rank_file_to_square(rank: u8, file: u8) -> Square {
    rank * 8 + file
}

pub fn square_to_file(s: Square) -> u8 {
    s % 8
}

pub fn square_to_rank(s: Square) -> u8 {
    s / 8
}

pub fn square_to_rank_file(square: Square) -> (u8, u8) {
    let rank = square / 8;
    let file = square % 8;
    (rank, file)
}


pub fn algebraic_to_square(alg: &str) -> Square {
    let mut s = alg.chars();
    let file = s.next().unwrap();
    let rank = s.next().unwrap();
    let file = file as u8 - 'a' as u8;
    let rank = rank as u8 - '1' as u8;
    rank_file_to_square(rank, file)
}

pub fn square_to_algebraic(square: Square) -> String {
    let file = square_to_file(square);
    let rank = square_to_rank(square);
    let file_char = ('a' as u8 + file) as char;
    let rank_char = ('1' as u8 + rank) as char;
    format!("{}{}", file_char, rank_char)
}