#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[allow(dead_code)]
pub enum Piece {
    WhitePawn,
    WhiteKnight,
    WhiteBishop,
    WhiteRook,
    WhiteQueen,
    WhiteKing,
    BlackPawn,
    BlackKnight,
    BlackBishop,
    BlackRook,
    BlackQueen,
    BlackKing,
}
pub struct Board {
    //each bitboard contains
    bitboards: [u64; 12],
    white_turn: bool,
    //                           WHITE                         |                          BLACK
    //          QUEEN SIDE         |          KING SIDE        |        QUEEN SIDE         |         KING SIDE
    // [ can castle? | did castle? | can castle? | did castle? | can castle? | did castle? | can castle? | did castle?]
    castling_rights: u8,
}

impl Piece {
    fn get_index(&self) -> usize {
        *self as usize
    }
}

impl Board {
    pub fn new() -> Self {
        Self {
            bitboards: [0; 12],
            castling_rights: 0,
            white_turn: true,
        }
    }
    pub fn get(&self, piece: Piece) -> u64 {
        self.bitboards[piece.get_index()]
    }
    pub fn set(&mut self, piece: Piece, bitboard: u64) {
        self.bitboards[piece.get_index()] = bitboard;
    }
    pub fn set_bit(&mut self, piece: Piece, square: usize) {
        self.bitboards[piece.get_index()] |= 1 << square;
    }
    pub fn clear_bit(&mut self, piece: Piece, square: usize) {
        self.bitboards[piece.get_index()] &= 1 << square;
    }
}
