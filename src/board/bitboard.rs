use crate::board::state::Square::{
    A1, A2, A7, A8, B1, B2, B7, B8, C1, C2, C7, C8, D1, D2, D7, D8, E1, E2, E7, E8, F1, F2, F7, F8,
    G1, G2, G7, G8, H1, H2, H7, H8,
};
use crate::board::state::*;

pub type Bits = u64;
pub type Index = u8;

// Initial positions
pub const P_W_INIT: [Square; 8] = [A2, B2, C2, D2, E2, F2, G2, H2];
pub const P_B_INIT: [Square; 8] = [A7, B7, C7, D7, E7, F7, G7, H7];
pub const N_W_INIT: [Square; 2] = [B1, G1];
pub const N_B_INIT: [Square; 2] = [B8, G8];
pub const B_W_INIT: [Square; 2] = [C1, F1];
pub const B_B_INIT: [Square; 2] = [C8, F8];
pub const R_W_INIT: [Square; 2] = [A1, H1];
pub const R_B_INIT: [Square; 2] = [A8, H8];
pub const Q_W_INIT: [Square; 1] = [D1];
pub const Q_B_INIT: [Square; 1] = [D8];
pub const K_W_INIT: [Square; 1] = [E1];
pub const K_B_INIT: [Square; 1] = [E8];

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Bitboard {
    data: Bits,
}
impl Bitboard {
    pub fn from_squares(squares: &[Square]) -> Self {
        let mut data: Bits = 0;
        for square in squares {
            data |= Self::bits_from_square(*square);
        }
        Bitboard { data }
    }

    pub fn from_bits(bits: Bits) -> Self {
        Bitboard { data: bits }
    }

    pub fn get_bits(&self) -> Bits {
        self.data
    }

    pub fn squares_occupied(&self, squares: &[Square]) -> bool {
        for square in squares {
            if !self.square_occupied(*square) {
                false;
            }
        }
        true
    }

    pub fn square_occupied(&self, square: Square) -> bool {
        self.data & Self::bits_from_square(square) != 0
    }

    const fn bits_from_square(square: Square) -> Bits {
        1 << square as Index
    }
}

pub fn attacked_squares_for(piece: PieceType, square: Square) -> Bitboard {
    match piece {
        PieceType::Pawn => attacked_squares_for_pawn(Bitboard::from_squares(&[square]).get_bits()),
        PieceType::Knight => {
            attacked_squares_for_knight(Bitboard::from_squares(&[square]).get_bits())
            // todo create lookup table
        }
        PieceType::King => attacked_squares_for_king(Bitboard::from_squares(&[square]).get_bits()),
        _ => Bitboard::from_squares(&[square]), // todo create lookup table
    }
}

const NOT_FILE_A: Bits = 0xFFFFFFFFFFFFFF00;
const NOT_FILE_B: Bits = 0xFFFFFFFFFFFF00FF;
const NOT_FILE_G: Bits = 0xFF00FFFFFFFFFFFF;
const NOT_FILE_H: Bits = 0x00FFFFFFFFFFFFFF;
const NOT_RANK_1: Bits = 0xFEFEFEFEFEFEFEFE;
const NOT_RANK_2: Bits = 0xFDFDFDFDFDFDFDFD;
const NOT_RANK_7: Bits = 0xBFBFBFBFBFBFBFBF;
const NOT_RANK_8: Bits = 0x7F7F7F7F7F7F7F7F;

fn attacked_squares_for_pawn(pawn: Bits) -> Bitboard {
    Bitboard::from_bits((pawn & NOT_FILE_A) >> 7 | (pawn & NOT_FILE_H) << 9)
}

fn attacked_squares_for_knight(knight: Bits) -> Bitboard {
    Bitboard::from_bits(
        (knight & NOT_FILE_G & NOT_FILE_H & NOT_RANK_8) << 17
            | (knight & NOT_FILE_H & NOT_RANK_7 & NOT_RANK_8) << 10
            | (knight & NOT_FILE_A & NOT_RANK_7 & NOT_RANK_8) >> 6
            | (knight & NOT_FILE_A & NOT_FILE_B & NOT_RANK_8) >> 15
            | (knight & NOT_FILE_A & NOT_FILE_B & NOT_RANK_1) >> 17
            | (knight & NOT_FILE_A & NOT_RANK_1 & NOT_RANK_2) >> 10
            | (knight & NOT_FILE_H & NOT_RANK_1 & NOT_RANK_2) << 6
            | (knight & NOT_FILE_G & NOT_FILE_H & NOT_RANK_1) << 15,
    )
}

fn attacked_squares_for_king(king: Bits) -> Bitboard {
    Bitboard::from_bits(
        (king & NOT_RANK_8) << 1
            | (king & NOT_FILE_H & NOT_RANK_8) << 9
            | (king & NOT_FILE_H) << 8
            | (king & NOT_FILE_H & NOT_RANK_1) << 7
            | (king & NOT_RANK_1) >> 1
            | (king & NOT_FILE_A & NOT_RANK_1) >> 9
            | (king & NOT_FILE_A) >> 8
            | (king & NOT_FILE_A & NOT_RANK_8) >> 7,
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::board::display::print_bitboard;
    use crate::board::state::PieceType::{King, Knight, Pawn};
    use crate::board::state::Square::*;

    #[test]
    fn test_attacked_squares_for_pawn() {
        let from_squares = Bitboard::from_squares;
        assert_eq!(attacked_squares_for(Pawn, A2), from_squares(&[B3]));
        assert_eq!(attacked_squares_for(Pawn, B2), from_squares(&[A3, C3]));
        assert_eq!(attacked_squares_for(Pawn, C2), from_squares(&[B3, D3]));
        assert_eq!(attacked_squares_for(Pawn, D2), from_squares(&[C3, E3]));
        assert_eq!(attacked_squares_for(Pawn, E2), from_squares(&[D3, F3]));
        assert_eq!(attacked_squares_for(Pawn, F2), from_squares(&[E3, G3]));
        assert_eq!(attacked_squares_for(Pawn, G2), from_squares(&[F3, H3]));
        assert_eq!(attacked_squares_for(Pawn, H2), from_squares(&[G3]));
    }

    #[test]
    fn test_attacked_squares_knight() {
        let check = Bitboard::from_squares;
        let test = attacked_squares_for;

        // A-file
        assert_eq!(test(Knight, A1), check(&[B3, C2]));
        assert_eq!(test(Knight, A2), check(&[B4, C1, C3]));
        assert_eq!(test(Knight, A3), check(&[B1, B5, C2, C4]));
        assert_eq!(test(Knight, A4), check(&[B2, B6, C3, C5]));
        assert_eq!(test(Knight, A5), check(&[B3, B7, C4, C6]));
        assert_eq!(test(Knight, A6), check(&[B4, B8, C5, C7]));
        assert_eq!(test(Knight, A7), check(&[B5, C6, C8]));
        assert_eq!(test(Knight, A8), check(&[B6, C7]));

        // B-file
        assert_eq!(test(Knight, B1), check(&[A3, C3, D2]));
        assert_eq!(test(Knight, B2), check(&[A4, C4, D1, D3]));
        assert_eq!(test(Knight, B3), check(&[A1, A5, C1, C5, D2, D4]));
        assert_eq!(test(Knight, B4), check(&[A2, A6, C2, C6, D3, D5]));
        assert_eq!(test(Knight, B5), check(&[A3, A7, C3, C7, D4, D6]));
        assert_eq!(test(Knight, B6), check(&[A4, A8, C4, C8, D5, D7]));
        assert_eq!(test(Knight, B7), check(&[A5, C5, D6, D8]));
        assert_eq!(test(Knight, B8), check(&[A6, C6, D7]));

        // C-file
        assert_eq!(test(Knight, C1), check(&[A2, B3, D3, E2]));
        assert_eq!(test(Knight, C2), check(&[A1, A3, B4, D4, E1, E3]));
        assert_eq!(test(Knight, C3), check(&[A2, A4, B1, B5, D1, D5, E2, E4]));
        assert_eq!(test(Knight, C4), check(&[A3, A5, B2, B6, D2, D6, E3, E5]));
        assert_eq!(test(Knight, C5), check(&[A4, A6, B3, B7, D3, D7, E4, E6]));
        assert_eq!(test(Knight, C6), check(&[A5, A7, B4, B8, D4, D8, E5, E7]));
        assert_eq!(test(Knight, C7), check(&[A6, A8, B5, D5, E6, E8]));
        assert_eq!(test(Knight, C8), check(&[A7, B6, D6, E7]));

        // D-file
        assert_eq!(test(Knight, D1), check(&[B2, C3, E3, F2]));
        assert_eq!(test(Knight, D2), check(&[B1, B3, C4, E4, F1, F3]));
        assert_eq!(test(Knight, D3), check(&[B2, B4, C1, C5, E1, E5, F2, F4]));
        assert_eq!(test(Knight, D4), check(&[B3, B5, C2, C6, E2, E6, F3, F5]));
        assert_eq!(test(Knight, D5), check(&[B4, B6, C3, C7, E3, E7, F4, F6]));
        assert_eq!(test(Knight, D6), check(&[B5, B7, C4, C8, E4, E8, F5, F7]));
        assert_eq!(test(Knight, D7), check(&[B6, B8, C5, E5, F6, F8]));
        assert_eq!(test(Knight, D8), check(&[B7, C6, E6, F7]));

        // E-file
        assert_eq!(test(Knight, E1), check(&[C2, D3, F3, G2]));
        assert_eq!(test(Knight, E2), check(&[C1, C3, D4, F4, G1, G3]));
        assert_eq!(test(Knight, E3), check(&[C2, C4, D1, D5, F1, F5, G2, G4]));
        assert_eq!(test(Knight, E4), check(&[C3, C5, D2, D6, F2, F6, G3, G5]));
        assert_eq!(test(Knight, E5), check(&[C4, C6, D3, D7, F3, F7, G4, G6]));
        assert_eq!(test(Knight, E6), check(&[C5, C7, D4, D8, F4, F8, G5, G7]));
        assert_eq!(test(Knight, E7), check(&[C6, C8, D5, F5, G6, G8]));
        assert_eq!(test(Knight, E8), check(&[C7, D6, F6, G7]));

        // F-file
        assert_eq!(test(Knight, F1), check(&[D2, E3, G3, H2]));
        assert_eq!(test(Knight, F2), check(&[D1, D3, E4, G4, H1, H3]));
        assert_eq!(test(Knight, F3), check(&[D2, D4, E1, E5, G1, G5, H2, H4]));
        assert_eq!(test(Knight, F4), check(&[D3, D5, E2, E6, G2, G6, H3, H5]));
        assert_eq!(test(Knight, F5), check(&[D4, D6, E3, E7, G3, G7, H4, H6]));
        assert_eq!(test(Knight, F6), check(&[D5, D7, E4, E8, G4, G8, H5, H7]));
        assert_eq!(test(Knight, F7), check(&[D6, D8, E5, G5, H6, H8]));
        assert_eq!(test(Knight, F8), check(&[D7, E6, G6, H7]));

        // G-file
        assert_eq!(test(Knight, G1), check(&[E2, F3, H3]));
        assert_eq!(test(Knight, G2), check(&[E1, E3, F4, H4]));
        assert_eq!(test(Knight, G3), check(&[E2, E4, F1, F5, H1, H5]));
        assert_eq!(test(Knight, G4), check(&[E3, E5, F2, F6, H2, H6]));
        assert_eq!(test(Knight, G5), check(&[E4, E6, F3, F7, H3, H7]));
        assert_eq!(test(Knight, G6), check(&[E5, E7, F4, F8, H4, H8]));
        assert_eq!(test(Knight, G7), check(&[E6, E8, F5, H5]));
        assert_eq!(test(Knight, G8), check(&[E7, F6, H6]));

        // H-file
        assert_eq!(test(Knight, H1), check(&[F2, G3]));
        assert_eq!(test(Knight, H2), check(&[F1, F3, G4]));
        assert_eq!(test(Knight, H3), check(&[F2, F4, G1, G5]));
        assert_eq!(test(Knight, H4), check(&[F3, F5, G2, G6]));
        assert_eq!(test(Knight, H5), check(&[F4, F6, G3, G7]));
        assert_eq!(test(Knight, H6), check(&[F5, F7, G4, G8]));
        assert_eq!(test(Knight, H7), check(&[F6, F8, G5]));
        assert_eq!(test(Knight, H8), check(&[F7, G6]));
    }

    #[test]
    fn test_attacked_squares_king() {
        let check = Bitboard::from_squares;
        let test = attacked_squares_for;

        // A-file
        assert_eq!(test(King, A1), check(&[A2, B1, B2]));
        assert_eq!(test(King, A2), check(&[A1, A3, B1, B2, B3]));
        assert_eq!(test(King, A3), check(&[A2, A4, B2, B3, B4]));
        assert_eq!(test(King, A4), check(&[A3, A5, B3, B4, B5]));
        assert_eq!(test(King, A5), check(&[A4, A6, B4, B5, B6]));
        assert_eq!(test(King, A6), check(&[A5, A7, B5, B6, B7]));
        assert_eq!(test(King, A7), check(&[A6, A8, B6, B7, B8]));
        assert_eq!(test(King, A8), check(&[A7, B7, B8]));

        // B-file
        assert_eq!(test(King, B1), check(&[A1, A2, B2, C1, C2]));
        assert_eq!(test(King, B2), check(&[A1, A2, A3, B1, B3, C1, C2, C3]));
        assert_eq!(test(King, B3), check(&[A2, A3, A4, B2, B4, C2, C3, C4]));
        assert_eq!(test(King, B4), check(&[A3, A4, A5, B3, B5, C3, C4, C5]));
        assert_eq!(test(King, B5), check(&[A4, A5, A6, B4, B6, C4, C5, C6]));
        assert_eq!(test(King, B6), check(&[A5, A6, A7, B5, B7, C5, C6, C7]));
        assert_eq!(test(King, B7), check(&[A6, A7, A8, B6, B8, C6, C7, C8]));
        assert_eq!(test(King, B8), check(&[A7, A8, B7, C7, C8]));

        // C-file
        assert_eq!(test(King, C1), check(&[B1, B2, C2, D1, D2]));
        assert_eq!(test(King, C2), check(&[B1, B2, B3, C1, C3, D1, D2, D3]));
        assert_eq!(test(King, C3), check(&[B2, B3, B4, C2, C4, D2, D3, D4]));
        assert_eq!(test(King, C4), check(&[B3, B4, B5, C3, C5, D3, D4, D5]));
        assert_eq!(test(King, C5), check(&[B4, B5, B6, C4, C6, D4, D5, D6]));
        assert_eq!(test(King, C6), check(&[B5, B6, B7, C5, C7, D5, D6, D7]));
        assert_eq!(test(King, C7), check(&[B6, B7, B8, C6, C8, D6, D7, D8]));
        assert_eq!(test(King, C8), check(&[B7, B8, C7, D7, D8]));

        // D-file
        assert_eq!(test(King, D1), check(&[C1, C2, D2, E1, E2]));
        assert_eq!(test(King, D2), check(&[C1, C2, C3, D1, D3, E1, E2, E3]));
        assert_eq!(test(King, D3), check(&[C2, C3, C4, D2, D4, E2, E3, E4]));
        assert_eq!(test(King, D4), check(&[C3, C4, C5, D3, D5, E3, E4, E5]));
        assert_eq!(test(King, D5), check(&[C4, C5, C6, D4, D6, E4, E5, E6]));
        assert_eq!(test(King, D6), check(&[C5, C6, C7, D5, D7, E5, E6, E7]));
        assert_eq!(test(King, D7), check(&[C6, C7, C8, D6, D8, E6, E7, E8]));
        assert_eq!(test(King, D8), check(&[C7, C8, D7, E7, E8]));

        // E-file
        assert_eq!(test(King, E1), check(&[D1, D2, E2, F1, F2]));
        assert_eq!(test(King, E2), check(&[D1, D2, D3, E1, E3, F1, F2, F3]));
        assert_eq!(test(King, E3), check(&[D2, D3, D4, E2, E4, F2, F3, F4]));
        assert_eq!(test(King, E4), check(&[D3, D4, D5, E3, E5, F3, F4, F5]));
        assert_eq!(test(King, E5), check(&[D4, D5, D6, E4, E6, F4, F5, F6]));
        assert_eq!(test(King, E6), check(&[D5, D6, D7, E5, E7, F5, F6, F7]));
        assert_eq!(test(King, E7), check(&[D6, D7, D8, E6, E8, F6, F7, F8]));
        assert_eq!(test(King, E8), check(&[D7, D8, E7, F7, F8]));

        // F-file
        assert_eq!(test(King, F1), check(&[E1, E2, F2, G1, G2]));
        assert_eq!(test(King, F2), check(&[E1, E2, E3, F1, F3, G1, G2, G3]));
        assert_eq!(test(King, F3), check(&[E2, E3, E4, F2, F4, G2, G3, G4]));
        assert_eq!(test(King, F4), check(&[E3, E4, E5, F3, F5, G3, G4, G5]));
        assert_eq!(test(King, F5), check(&[E4, E5, E6, F4, F6, G4, G5, G6]));
        assert_eq!(test(King, F6), check(&[E5, E6, E7, F5, F7, G5, G6, G7]));
        assert_eq!(test(King, F7), check(&[E6, E7, E8, F6, F8, G6, G7, G8]));
        assert_eq!(test(King, F8), check(&[E7, E8, F7, G7, G8]));

        // G-file
        assert_eq!(test(King, G1), check(&[F1, F2, G2, H1, H2]));
        assert_eq!(test(King, G2), check(&[F1, F2, F3, G1, G3, H1, H2, H3]));
        assert_eq!(test(King, G3), check(&[F2, F3, F4, G2, G4, H2, H3, H4]));
        assert_eq!(test(King, G4), check(&[F3, F4, F5, G3, G5, H3, H4, H5]));
        assert_eq!(test(King, G5), check(&[F4, F5, F6, G4, G6, H4, H5, H6]));
        assert_eq!(test(King, G6), check(&[F5, F6, F7, G5, G7, H5, H6, H7]));
        assert_eq!(test(King, G7), check(&[F6, F7, F8, G6, G8, H6, H7, H8]));
        assert_eq!(test(King, G8), check(&[F7, F8, G7, H7, H8]));

        // H-file
        assert_eq!(test(King, H1), check(&[G1, G2, H2]));
        assert_eq!(test(King, H2), check(&[G1, G2, G3, H1, H3]));
        assert_eq!(test(King, H3), check(&[G2, G3, G4, H2, H4]));
        assert_eq!(test(King, H4), check(&[G3, G4, G5, H3, H5]));
        assert_eq!(test(King, H5), check(&[G4, G5, G6, H4, H6]));
        assert_eq!(test(King, H6), check(&[G5, G6, G7, H5, H7]));
        assert_eq!(test(King, H7), check(&[G6, G7, G8, H6, H8]));
        assert_eq!(test(King, H8), check(&[G7, G8, H7]));
    }
}
