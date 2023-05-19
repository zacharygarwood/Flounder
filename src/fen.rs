use crate::board::{Board, Position};
use crate::bitboard::{RANKS, FILES};
use crate::pieces::{Piece, Color};

// pub fn fen_to_board(fen: &str) -> Board {
//     let fen_parts: Vec<&str> = fen.split(' ').collect();

//     let piece_placement = parse_piece_placement(fen_parts[0]);
//     let active_color = parse_active_color(fen_parts[1]);
//     // let castling_ability = parse_castling_ability(fen_parts[2]);
//     // let en_passant_target = parse_en_passant_target(fen_parts[3]);
//     // let halfmove_clock = parse_halfmove_clock(fen_parts[4]);
//     // let fullmove_counter = parse_fullmove_counter(fen_parts[5]);
// }

fn parse_piece_placement(piece_placement: &str) -> Position {
    let mut position = Position::new();
    let pieces_placement_rank: Vec<&str> = piece_placement.split('/').collect();

    for (idx, values) in pieces_placement_rank.iter().enumerate() {
        let rank = 7 - idx as u8;
        let mut file = 0;
        for c in values.chars() {
            match c {
                'p' | 'n' | 'b' | 'r' | 'q' | 'k' | 
                'P' | 'N' | 'B' | 'R' | 'Q' | 'K' => {
                    position.add_piece(char_to_color(c), char_to_piece(c), rank, file);
                    file += 1;
                } 
                '1'..='8' => file += c.to_digit(10).unwrap() as u8,
                _ => panic!("Invalid character in fen piece placement")
            };
        }
    }
    position
}

pub fn parse_active_color(active_color: &str) -> Color {
    let c = active_color.chars().next().unwrap();
    match c {
        'w' => Color::White,
        'b' => Color::Black,
        _ => panic!("Invalid character in fen active color")
    }
}

// fn parse_castling_ability(castling_ability: &str) -> Castle {

// }

// fn parse_en_passant_target(en_passant_target: &str) -> Option<Square> {

// }

// fn parse_halfmove_clock(halfmove_clock: &str) -> usize {

// }

// fn parse_fullmove_counter(fullmove_counter: &str) -> usize {

// }

pub fn char_to_piece(c: char) -> Piece {
    match c.to_ascii_lowercase() {
        'p' => Piece::Pawn,
        'n' => Piece::Knight,
        'b' => Piece::Bishop,
        'r' => Piece::Rook,
        'q' => Piece::Queen,
        'k' => Piece::King,
        _ => panic!("Invalid piece character provided by FEN")
    }
}

pub fn char_to_color(c: char) -> Color {
    if c.is_lowercase() {
        Color::Black
    } else {
        Color::White
    }
}