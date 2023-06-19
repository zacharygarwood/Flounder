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

#[cfg(test)]
mod tests {
    use crate::square::{square_to_algebraic, algebraic_to_square, rank_file_to_square, square_to_rank_file};


    #[test]
    fn test_square_to_algebraic() {
        assert_eq!("a1", square_to_algebraic(0));
        assert_eq!("a8", square_to_algebraic(56));
        assert_eq!("h1", square_to_algebraic(7));
        assert_eq!("h8", square_to_algebraic(63));
    }

    #[test]
    fn test_algebraic_to_square() {
        assert_eq!(0, algebraic_to_square("a1"));
        assert_eq!(56, algebraic_to_square("a8"));
        assert_eq!(7, algebraic_to_square("h1"));
        assert_eq!(63, algebraic_to_square("h8"));
    }

    #[test]
    fn test_rank_file_to_square() {
        assert_eq!(0, rank_file_to_square(0, 0));
        assert_eq!(7, rank_file_to_square(0, 7));
        assert_eq!(56, rank_file_to_square(7, 0));
        assert_eq!(63, rank_file_to_square(7, 7));
    }

    #[test]
    fn test_square_to_rank_file() {
        assert_eq!((0,0), square_to_rank_file(0));
        assert_eq!((0,7), square_to_rank_file(7));
        assert_eq!((7,0), square_to_rank_file(56));
        assert_eq!((7,7), square_to_rank_file(63));
    }

    
}