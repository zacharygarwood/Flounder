#[allow(non_camel_case_types)]
pub enum Pieces { 
    P,
    B,
    N,
    R,
    Q,
    K,
    p,
    b,
    n,
    r,
    q,
    k,
    e,
    o,
}

impl std::fmt::Display for Pieces {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let piece_str = match self {
            Pieces::P => '♟',
            Pieces::B => '♝',
            Pieces::N => '♞',
            Pieces::R => '♜',
            Pieces::Q => '♛',
            Pieces::K => '♚',
            Pieces::p => '♙',
            Pieces::b => '♗',
            Pieces::n => '♘',
            Pieces::r => '♖',
            Pieces::q => '♕',
            Pieces::k => '♔',
            Pieces::e => '.',
            Pieces::o => 'o',
        };
        write!(f, "{}", piece_str)
    }
}

pub enum Color {
    WHITE,
    BLACK,
}