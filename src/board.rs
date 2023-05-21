use crate::pieces::{Piece, Color, PIECE_COUNT, COLOR_COUNT};
use crate::square::Square;
use crate::bitboard::{Bitboard, BitboardOperations};
use crate::util::print_bitboard;
use crate::fen::fen_to_board;

// Represents the chess board using bitboards
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

    // Bitboard of all empty spaces
    pub fn bb_empty(&self) -> Bitboard {
        !(self.bb_color(Color::White) | self.bb_color(Color::Black))
    }

    // Bitboard of all pieces
    pub fn bb_all(&self) -> Bitboard {
        self.bb_color(Color::White) | self.bb_color(Color::Black)
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

    pub fn add_piece(&mut self, color: Color, piece: Piece, rank: u8, file: u8) {
        self.colors[color].set_bit(rank, file);
        self.pieces[piece].set_bit(rank, file);
    }
}

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
}