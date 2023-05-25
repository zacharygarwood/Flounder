use crate::pieces::{Piece, Color, PIECE_COUNT, COLOR_COUNT};
use crate::square::{Square, A1, D1, F1, G1, H1, A8, D8, F8, G8, H8};
use crate::bitboard::{Bitboard, BitboardOperations, WHITE_QUEEN_SIDE, WHITE_KING_SIDE, BLACK_QUEEN_SIDE, BLACK_KING_SIDE};
use crate::util::print_bitboard;
use crate::fen::fen_to_board;
use crate::moves::{Move, MoveType};

// Represents the chess board using bitboards
#[derive(Copy, Clone, Debug)]
pub struct Board {
    pub position: Position,
    pub active_color: Color,
    pub castling_ability: Castle,
    pub en_passant_target: Option<Square>,
    pub halfmove_clock: u8,
    pub fullmove_counter: u8,
}

impl Board {
    pub fn new(fen: &str) -> Self {
        match fen_to_board(fen) {
            Ok(board) => board,
            Err(err) => {
                println!("Error constructing FEN: {}", err);
                println!("Setting board to default values");
                Self::default()
            },
        }
    }

    // Creates the default board state
    pub fn default() -> Self {
        Self {
            position: Position::default(),
            active_color: Color::White,
            castling_ability: Castle::new(true, true, true, true),
            en_passant_target: None,
            halfmove_clock: 0,
            fullmove_counter: 1,
        }
    }

    // Returns select pieces of a certain color e.g. white pawns
    pub fn bb(&self, color: Color, piece: Piece) -> Bitboard {
        self.position.bb(color, piece)
    }

    // Returns all pieces of a certain color e.g. white pieces
    pub fn bb_color(&self, color: Color) -> Bitboard {
        self.position.bb_color(color)
    }

    // Returns all pieces of a select type e.g. pawns
    pub fn bb_piece(&self, piece: Piece) -> Bitboard {
            self.position.bb_piece(piece)
        }

    // Returns the color of the player to play
    pub fn active_color(&self) -> Color {
        self.active_color
    }

    pub fn castling_ability(&self, color: Color) -> (bool, bool) {
        match color {
            Color::White => (self.castling_ability.white_king, self.castling_ability.white_queen),
            Color::Black => (self.castling_ability.black_king, self.castling_ability.black_queen),
        }
    }

    // Bitboard of all empty spaces
    pub fn bb_empty(&self) -> Bitboard {
        !(self.bb_color(Color::White) | self.bb_color(Color::Black))
    }

    // Bitboard of all pieces
    pub fn bb_all(&self) -> Bitboard {
        self.bb_color(Color::White) | self.bb_color(Color::Black)
    }

    pub fn add_piece(&mut self, color: Color, piece: Piece, square: Square) {
        self.position.add_piece(color, piece, square);
    }

    pub fn remove_piece(&mut self, color: Color, piece: Piece, square: Square) {
        self.position.remove_piece(color, piece, square);
    }

    pub fn make_move(&mut self, mv: &Move) {
        self.reset_en_passant_target();

        if mv.move_type == MoveType::EnPassant {
            self.make_en_passant(mv);
        }

        if mv.move_type == MoveType::Castle {
            self.make_castle(mv);
        }
    }

    fn reset_en_passant_target(&mut self) {
        self.en_passant_target = None;
    }

    fn make_en_passant(&mut self, mv: &Move) {
        let color = self.active_color;
        let offset = match color {
            Color::White => 8,
            Color::Black => -8,
        };

        let captured_square = (mv.to as i8 - offset) as u8;

        self.remove_piece(!color, Piece::Pawn, captured_square);
        self.remove_piece(color, Piece::Pawn, mv.from);
        self.add_piece(color, Piece::Pawn, mv.to)

        // TODO: Might need to modify halfmove clock and fullmove counter
    }

    fn make_castle(&mut self, mv: &Move) {
        let color = self.active_color;

        let is_king_side = match color {
            Color::White => G1 == mv.to,
            Color::Black => G8 == mv.to,
        };

        let (rook_from, rook_to) = match is_king_side {
            true => match color {
                Color::White => (H8, F1),
                Color::Black => (H8, F8),
            },
            false => match color {
                Color::White => (A1, D1),
                Color::Black => (A8, D8),
            }
        };

        self.remove_piece(color, Piece::King, mv.from);
        self.add_piece(color, Piece::King, mv.to);

        self.remove_piece(color, Piece::Rook, rook_from);
        self.add_piece(color, Piece::Rook, rook_to);

        self.remove_castle_rights(color);

    }

    fn remove_castle_rights(&mut self, color: Color) {
        self.castling_ability.remove_rights(color);
    }

    pub fn print(&self) {
        // TODO: Eventually use a 2D array of pieces to display the board instead of bitboards
        println!("White pieces:\n");
        print_bitboard(self.bb_color(Color::White));

        println!("Black pieces:\n");
        print_bitboard(self.bb_color(Color::Black));

        println!("Active color: {}", self.active_color);

        println!(
            "Castling ability: {}{}{}{}",
            if self.castling_ability.white_king { "K" } else { "" },
            if self.castling_ability.white_queen { "Q" } else { "" },
            if self.castling_ability.black_king { "k" } else { "" },
            if self.castling_ability.black_queen { "q" } else { "" }
        );
        match self.en_passant_target {
            Some(square) => println!("En Passant Target: {}", square),
            None => println!("En Passant Target: None"),
        }

        println!("Halfmove clock: {}", self.halfmove_clock);
        println!("Fullmove counter: {}\n", self.fullmove_counter);
    }
}


#[derive(Copy, Clone, Debug)]
pub struct Position {
    pieces: [Bitboard; PIECE_COUNT], // Six bitboards for the pieces
    colors: [Bitboard; COLOR_COUNT], // Two bitboards for the colors
}

impl Position {
    pub fn new() -> Self{
        let mut pieces = [0; PIECE_COUNT];
        let mut colors = [0; COLOR_COUNT];
    
        Self { pieces, colors}
    }

    // Creates the default chess starting position
    pub fn default() -> Self {
        let mut pieces = [0; PIECE_COUNT];
        let mut colors = [0; COLOR_COUNT];

        pieces[Piece::Pawn] = 0x00ff00000000ff00;
        pieces[Piece::Knight] = 0x4200000000000042;
        pieces[Piece::Bishop] = 0x2400000000000024;
        pieces[Piece::Rook] = 0x8100000000000081;
        pieces[Piece::Queen] = 0x0800000000000008;
        pieces[Piece::King] = 0x1000000000000010;
        
        colors[Color::White] = 0x000000000000ffff;
        colors[Color::Black] = 0xffff000000000000;

        Self { pieces, colors}
    }

    // Returns select pieces of a certain color e.g. white pawns
    pub fn bb(&self, color: Color, piece: Piece) -> u64 {
        self.pieces[piece] & self.colors[color]
    }

    // Returns all pieces of a certain color e.g. white pieces
    pub fn bb_color(&self, color: Color) -> u64 {
        self.colors[color]
    }

    // Returns all pieces of a select type e.g. pawns
    pub fn bb_piece(&self, piece: Piece) -> u64 {
        self.pieces[piece]
    }

    pub fn add_piece(&mut self, color: Color, piece: Piece, square: Square) {
        self.colors[color].set_bit(square);
        self.pieces[piece].set_bit(square);
    }

    pub fn remove_piece(&mut self, color: Color, piece: Piece, square: Square) {
        self.colors[color].remove_bit(square);
        self.pieces[piece].remove_bit(square);
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Castle {
    white_king: bool,
    white_queen: bool,
    black_king: bool,
    black_queen: bool,
}

impl Castle {
    pub fn new(white_king: bool, white_queen: bool, black_king: bool, black_queen: bool) -> Self {
        Self {
            white_king,
            white_queen,
            black_king,
            black_queen,
        } 
    }

    pub fn set(&mut self, castle: char, ability: bool) {
        match castle {
            'K' => self.white_king = ability,
            'Q' => self.white_queen = ability,
            'k' => self.black_king = ability,
            'q' => self.black_queen = ability,
            _ => {} // Can ignore any other character
        };
    }

    pub fn remove_rights(&mut self, color: Color) {
        match color {
            Color::White => {
                self.white_king = false;
                self.white_queen = false;
            }
            Color::Black => {
                self.black_king = false;
                self.black_queen = false;
            }
        };
    }
}