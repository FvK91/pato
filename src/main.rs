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
const PAWN_WHITE_INITIAL: Bits = 0x000000000000FF00;
const PAWN_BLACK_INITIAL: Bits = 0x00FF000000000000;
const KNIGHT_WHITE_INITIAL: Bits = 0x0000000000000042;
const KNIGHT_BLACK_INITIAL: Bits = 0x4200000000000000;
const BISHOP_WHITE_INITIAL: Bits = 0x0000000000000024;
const BISHOP_BLACK_INITIAL: Bits = 0x2400000000000000;
const ROOK_WHITE_INITIAL: Bits = 0x0000000000000081;
const ROOK_BLACK_INITIAL: Bits = 0x8100000000000000;
const QUEEN_WHITE_INITIAL: Bits = 0x0000000000000010;
const QUEEN_BLACK_INITIAL: Bits = 0x1000000000000000;
const KING_WHITE_INITIAL: Bits = 0x000000000000008;
const KING_BLACK_INITIAL: Bits = 0x800000000000000;

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
        let occupied_squares = self.get_occupied_squares();
        for (square_index, square) in Square::iter().enumerate() {
            if square_index != 0 && square_index % 8 == 0 {
                print!("\n");
            }

            if !Self::has_piece_on_square(occupied_squares, square) {
                print!(". ");
                continue;
            }

            for (piece, color) in PieceType::iter().cartesian_product(Color::iter()) {
                if Self::has_piece_on_square(self.get_bitboard_for(piece, color), square) {
                    print!("x");
                    break;
                }
            }
            print!(" ")
        }
    }

    fn get_bitboard_for(&self, piece: PieceType, color: Color) -> Bits {
        self.bitboards[self.get_index_for(piece, color)]
    }

    fn get_index_for(&self, piece: PieceType, color: Color) -> usize {
        piece as usize + (PieceType::COUNT * color as usize)
    }
    fn get_occupied_squares(&self) -> Bits {
        let mut occupied_squares: Bits = 0;
        for bitboard in self.bitboards.iter() {
            occupied_squares = occupied_squares | bitboard
        }
        occupied_squares
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
