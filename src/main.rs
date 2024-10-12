use colored::*;
use itertools::Itertools;
use strum::{EnumCount, IntoEnumIterator};
use strum_macros::{EnumCount, EnumIter};

type Bits = u64;
type Index = u8;

// Going for Little-Endian Rank-File Mapping now.
// Why? No clue (yet)
#[derive(EnumIter, Clone, Copy)]
#[repr(u8)]
enum Square {
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
enum PieceType {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

#[derive(EnumIter, EnumCount, Clone, Copy, Debug)]
enum Color {
    White,
    Black,
}

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

// ToDo: Print board
// ToDo: Implement generating possible moves

struct State {
    bitboards: [Bits; PieceType::COUNT * Color::COUNT],
}

impl State {
    fn new() -> Self {
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

    pub fn print(&self) {
        use Square::*;
        self.print_rank(&[A8, B8, C8, D8, E8, F8, G8, H8]);
        self.print_rank(&[A7, B7, C7, D7, E7, F7, G7, H7]);
        self.print_rank(&[A6, B6, C6, D6, E6, F6, G6, H6]);
        self.print_rank(&[A5, B5, C5, D5, E5, F5, G5, H5]);
        self.print_rank(&[A4, B4, C4, D4, E4, F4, G4, H4]);
        self.print_rank(&[A3, B3, C3, D3, E3, F3, G3, H3]);
        self.print_rank(&[A2, B2, C2, D2, E2, F2, G2, H2]);
        self.print_rank(&[A1, B1, C1, D1, E1, F1, G1, H1]);
    }

    // todo move printing to another struct
    fn print_rank(&self, squares: &[Square]) {
        for square in squares {
            self.print_square(*square);
        }
        print!("\n");
    }

    fn print_square(&self, square: Square) {
        for (piece, color) in PieceType::iter().cartesian_product(Color::iter()) {
            if Self::has_piece_on_square(self.get_bitboard_for(piece, color), square) {
                self.print_piece(piece, color);
                return;
            }
        }
        print!(". ");
    }

    fn print_piece(&self, piece: PieceType, color: Color) {
        use PieceType::*;
        let piece_char = match piece {
            Pawn => "P",
            Knight => "N",
            Bishop => "B",
            Rook => "R",
            Queen => "Q",
            King => "K",
        };

        let print_color = match color {
            Color::White => "green",
            _ => "red",
        };

        print!("{} ", piece_char.color(print_color));
    }

    fn get_bitboard_for(&self, piece: PieceType, color: Color) -> Bits {
        self.bitboards[self.get_index_for(piece, color)]
    }

    fn get_index_for(&self, piece: PieceType, color: Color) -> usize {
        piece as usize + (PieceType::COUNT * color as usize)
    }

    fn has_piece_on_square(bitboard: Bits, square: Square) -> bool {
        (bitboard & (1 << square as Index)) != 0
    }
}

fn main() {
    println!("Hello, pato!");
    let state = State::new();
    state.print();
}
