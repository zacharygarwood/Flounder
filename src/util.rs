use crate::bitboard::Bitboard;

pub fn print_board(num: u64) {
    use crate::bitboard::{RANKS, FILES};

    for rank in (0..RANKS).rev() {
        print!(" {} ", rank+1);
        for file in 0..FILES {
            let square = rank * 8 + file;
            let bit = (num >> square) & 1;
            print!(" {} ", bit);
        }
        println!();
    }
    println!("    a  b  c  d  e  f  g  h");
    println!();
}

pub fn board_to_hex(bb: Bitboard) -> String {
    format!("{:016x}", bb)
}