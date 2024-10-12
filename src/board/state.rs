use strum::EnumCount;
use strum_macros::{EnumCount, EnumIter};

#[derive(EnumIter, Clone, Copy)]
#[repr(u8)]
pub enum Square {
    A1,
    A2,
    A3,
    A4,
    A5,
    A6,
    A7,
    A8,
    B1,
    B2,
    B3,
    B4,
    B5,
    B6,
    B7,
    B8,
    C1,
    C2,
    C3,
    C4,
    C5,
    C6,
    C7,
    C8,
    D1,
    D2,
    D3,
    D4,
    D5,
    D6,
    D7,
    D8,
    E1,
    E2,
    E3,
    E4,
    E5,
    E6,
    E7,
    E8,
    F1,
    F2,
    F3,
    F4,
    F5,
    F6,
    F7,
    F8,
    G1,
    G2,
    G3,
    G4,
    G5,
    G6,
    G7,
    G8,
    H1,
    H2,
    H3,
    H4,
    H5,
    H6,
    H7,
    H8,
}

#[derive(EnumIter, EnumCount, Clone, Copy, Debug)]
pub enum PieceType {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

#[derive(EnumIter, EnumCount, Clone, Copy, Debug)]
pub enum Color {
    White,
    Black,
}

type Bits = u64;
type Index = u8;

// Going for Little-Endian Rank-File Mapping now.
// Why? No clue (yet)

// Initial positions
const PAWN_WHITE_INITIAL: Bits = 0x202020202020202;
const PAWN_BLACK_INITIAL: Bits = 0x4040404040404040;
const KNIGHT_WHITE_INITIAL: Bits = 0x1000000000100;
const KNIGHT_BLACK_INITIAL: Bits = 0x80000000008000;
const BISHOP_WHITE_INITIAL: Bits = 0x00010000010000;
const BISHOP_BLACK_INITIAL: Bits = 0x00800000800000;
const ROOK_WHITE_INITIAL: Bits = 0x100000000000001;
const ROOK_BLACK_INITIAL: Bits = 0x8000000000000080;
const QUEEN_WHITE_INITIAL: Bits = 0x0000000001000000;
const QUEEN_BLACK_INITIAL: Bits = 0x0000000080000000;
const KING_WHITE_INITIAL: Bits = 0x0000000100000000;
const KING_BLACK_INITIAL: Bits = 0x0000008000000000;

pub struct State {
    bitboards: [Bits; PieceType::COUNT * Color::COUNT],
}

impl State {
    pub fn new() -> Self {
        State {
            bitboards: [
                PAWN_WHITE_INITIAL,
                KNIGHT_WHITE_INITIAL,
                BISHOP_WHITE_INITIAL,
                ROOK_WHITE_INITIAL,
                QUEEN_WHITE_INITIAL,
                KING_WHITE_INITIAL,
                PAWN_BLACK_INITIAL,
                KNIGHT_BLACK_INITIAL,
                BISHOP_BLACK_INITIAL,
                ROOK_BLACK_INITIAL,
                QUEEN_BLACK_INITIAL,
                KING_BLACK_INITIAL,
            ],
        }
    }

    pub fn has_piece_on_square(bitboard: Bits, square: &Square) -> bool {
        (bitboard & (1 << square.clone() as Index)) != 0
    }

    pub fn get_bitboard_for(&self, piece: PieceType, color: Color) -> Bits {
        self.bitboards[self.get_index_for(piece, color)]
    }

    fn get_index_for(&self, piece: PieceType, color: Color) -> usize {
        piece as usize + (PieceType::COUNT * color as usize)
    }
}