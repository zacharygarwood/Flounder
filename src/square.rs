use std::f32::consts::SQRT_2;

pub type Square = u8;
pub enum SquareIndex {
    A1 = 0,
    B1,
    C1,
    D1,
    E1,
    F1,
    G1,
    H1,
    A2,
    B2,
    C2,
    D2,
    E2,
    F2,
    G2,
    H2,
    A3,
    B3,
    C3,
    D3,
    E3,
    F3,
    G3,
    H3,
    A4,
    B4,
    C4,
    D4,
    E4,
    F4,
    G4,
    H4,
    A5,
    B5,
    C5,
    D5,
    E5,
    F5,
    G5,
    H5,
    A6,
    B6,
    C6,
    D6,
    E6,
    F6,
    G6,
    H6,
    A7,
    B7,
    C7,
    D7,
    E7,
    F7,
    G7,
    H7,
    A8,
    B8,
    C8,
    D8,
    E8,
    F8,
    G8,
    H8,
}

pub fn rank_file_to_index(rank: u8, file: u8) -> Square {
    rank * 8 + file
}

pub fn square_to_file(s: Square) -> u8 {
    s % 8
}

pub fn square_to_rank(s: Square) -> u8 {
    s / 8
}

pub fn algebraic_to_square(alg: &str) -> Square {
    let mut s = alg.chars();
    let file = s.next().unwrap();
    let rank = s.next().unwrap();
    let file = file as u8 - 'a' as u8;
    let rank = rank as u8 - '1' as u8;
    rank_file_to_index(rank, file)
}

pub fn square_to_algebraic(square: Square) -> String {
    let file = square_to_file(square);
    let rank = square_to_rank(square);
    let file_char = ('a' as u8 + file) as char;
    let rank_char = ('1' as u8 + rank) as char;
    format!("{}{}", file_char, rank_char)
}