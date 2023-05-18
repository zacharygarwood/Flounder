pub type Square = u8;

pub fn rank_file_to_square(rank: u8, file: u8) -> Square {
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
    rank_file_to_square(rank, file)
}

pub fn square_to_algebraic(square: Square) -> String {
    let file = square_to_file(square);
    let rank = square_to_rank(square);
    let file_char = ('a' as u8 + file) as char;
    let rank_char = ('1' as u8 + rank) as char;
    format!("{}{}", file_char, rank_char)
}