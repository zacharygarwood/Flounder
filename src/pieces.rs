use std::ops::{Index, IndexMut, Not};
use crate::bitboard::Bitboard;


pub const COLOR_COUNT: usize = 2;
pub const PIECE_COUNT: usize = 6;

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum PieceType {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum Color {
    White,
    Black,
}

pub struct Piece {
    pub piece_type: PieceType,
    pub color: Color,
}

impl Piece {
    pub fn convert_char_to_piece(c: char) -> PieceType {
        match c.to_ascii_lowercase() {
            'p' => PieceType::Pawn,
            'r' => PieceType::Rook,
            'n' => PieceType::Knight,
            'b' => PieceType::Bishop,
            'k' => PieceType::King,
            'q' => PieceType::Queen,
            _ => panic!("Can't convert character: {}", c),
        }
    }

    pub fn convert_char_to_color(c: char) -> Color {
        if c.is_lowercase() {
            Color::Black
        } else {
            Color::White
        }
    }
}

impl Index<PieceType> for [Bitboard; PIECE_COUNT] {
    type Output = Bitboard;

    fn index(&self, piece: PieceType) -> &Self::Output {
        match piece {
            PieceType::Pawn => &self[0],
            PieceType::Knight => &self[1],
            PieceType::Bishop => &self[2],
            PieceType::Rook => &self[3],
            PieceType::Queen => &self[4],
            PieceType::King => &self[5],
        }
    }
}

impl IndexMut<PieceType> for [Bitboard; PIECE_COUNT] {
    fn index_mut(&mut self, piece: PieceType) -> &mut Self::Output {
        match piece {
            PieceType::Pawn => &mut self[0],
            PieceType::Knight => &mut self[1],
            PieceType::Bishop => &mut self[2],
            PieceType::Rook => &mut self[3],
            PieceType::Queen => &mut self[4],
            PieceType::King => &mut self[5],
        }
    }
}

impl Index<Color> for [Bitboard; COLOR_COUNT] {
    type Output = Bitboard;

    fn index(&self, color: Color) -> &Self::Output {
        match color {
            Color::White => &self[0],
            Color::Black => &self[1],
        }
    }
}

impl IndexMut<Color> for [Bitboard; COLOR_COUNT] {
    fn index_mut(&mut self, color: Color) -> &mut Self::Output {
        match color {
            Color::White => &mut self[0],
            Color::Black => &mut self[1],
        }
    }
}

impl std::ops::Not for Color {
    type Output = Color;

    fn not(self) -> Self::Output {
        match self {
            Color::White => Color::Black,
            Color::Black => Color::White,
        }
    }
}