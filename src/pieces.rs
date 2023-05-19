use std::ops::{Index, IndexMut, Not};
use crate::bitboard::Bitboard;


pub const COLOR_COUNT: usize = 2;
pub const PIECE_COUNT: usize = 6;

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum Piece {
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

impl std::fmt::Display for Piece {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let piece_str = match self {
            Piece::Pawn => "Pawn",
            Piece::Knight => "Knight",
            Piece::Bishop => "Bishop",
            Piece::Rook => "Rook",
            Piece::Queen => "Queen",
            Piece::King => "King",
        };
        write!(f, "{}", piece_str)
    }
}

#[derive(Copy, Clone, Debug)]
pub struct PromotionPieceIterator {
    current_piece: Piece,
}

impl PromotionPieceIterator {
    pub fn new() -> Self {
        PromotionPieceIterator {
            current_piece: Piece::Pawn,
        }
    }
}

impl Iterator for PromotionPieceIterator {
    type Item = Piece;

    fn next(&mut self) -> Option<Self::Item> {
        let next_piece = match self.current_piece {
            Piece::Pawn => Piece::Knight,
            Piece::Knight => Piece::Bishop,
            Piece::Bishop => Piece::Rook,
            Piece::Rook => Piece::Queen,
            Piece::Queen => return None,
            _ => unreachable!(),
        };

        self.current_piece = next_piece;
        Some(next_piece)
    }
}

impl Index<Piece> for [Bitboard; PIECE_COUNT] {
    type Output = Bitboard;

    fn index(&self, piece: Piece) -> &Self::Output {
        match piece {
            Piece::Pawn => &self[0],
            Piece::Knight => &self[1],
            Piece::Bishop => &self[2],
            Piece::Rook => &self[3],
            Piece::Queen => &self[4],
            Piece::King => &self[5],
        }
    }
}

impl IndexMut<Piece> for [Bitboard; PIECE_COUNT] {
    fn index_mut(&mut self, piece: Piece) -> &mut Self::Output {
        match piece {
            Piece::Pawn => &mut self[0],
            Piece::Knight => &mut self[1],
            Piece::Bishop => &mut self[2],
            Piece::Rook => &mut self[3],
            Piece::Queen => &mut self[4],
            Piece::King => &mut self[5],
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