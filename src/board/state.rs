use crate::board::bitboard::{
    Bitboard, B_B_INIT, B_W_INIT, K_B_INIT, K_W_INIT, N_B_INIT, N_W_INIT, P_B_INIT, P_W_INIT,
    Q_B_INIT, Q_W_INIT, R_B_INIT, R_W_INIT,
};

use strum::EnumCount;
use strum_macros::{EnumCount, EnumIter};

#[derive(EnumIter, Clone, Copy)]
#[repr(i8)]
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
pub struct State {
    bitboards: [Bitboard; PieceType::COUNT * Color::COUNT],
}

impl State {
    pub fn new() -> Self {
        State {
            bitboards: [
                Bitboard::from_squares(&P_W_INIT),
                Bitboard::from_squares(&N_W_INIT),
                Bitboard::from_squares(&B_W_INIT),
                Bitboard::from_squares(&R_W_INIT),
                Bitboard::from_squares(&Q_W_INIT),
                Bitboard::from_squares(&K_W_INIT),
                Bitboard::from_squares(&P_B_INIT),
                Bitboard::from_squares(&N_B_INIT),
                Bitboard::from_squares(&B_B_INIT),
                Bitboard::from_squares(&R_B_INIT),
                Bitboard::from_squares(&Q_B_INIT),
                Bitboard::from_squares(&K_B_INIT),
            ],
        }
    }

    pub fn get_bitboard_for(&self, piece: PieceType, color: Color) -> Bitboard {
        self.bitboards[self.get_index_for(piece, color)]
    }

    fn get_index_for(&self, piece: PieceType, color: Color) -> usize {
        piece as usize + (PieceType::COUNT * color as usize)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::board::state::Color::{Black, White};
    use crate::board::state::PieceType::{Bishop, King, Knight, Pawn, Queen, Rook};
    use Square::*;

    #[test]
    fn test_bitboard_from_square() {
        let bitboard_d7 = Bitboard::from_squares(&[E5]);
        assert!(bitboard_d7.squares_occupied(&[E5]));
    }

    #[test]
    fn test_initial_position_pawns() {
        let state: State = State::new();
        let white_pawn = state.get_bitboard_for(Pawn, White);
        assert!(white_pawn.squares_occupied(&[A2, B2, C2, D2, E2, F2, G2, H2]));

        let black_pawn = state.get_bitboard_for(Pawn, Black);
        assert!(black_pawn.squares_occupied(&[A7, B7, C7, D7, E7, F7, G7, H7]));
    }

    #[test]
    fn test_initial_position_knights() {
        let state: State = State::new();
        let white_knight = state.get_bitboard_for(Knight, White);
        assert!(white_knight.squares_occupied(&[B1, G1]));

        let black_knight = state.get_bitboard_for(Knight, Black);
        assert!(black_knight.squares_occupied(&[B8, G8]));
    }

    #[test]
    fn test_initial_position_white_bishops() {
        let state: State = State::new();
        let white_bishop = state.get_bitboard_for(Bishop, White);
        assert!(white_bishop.squares_occupied(&[C1, F1]));

        let black_bishop = state.get_bitboard_for(Bishop, Black);
        assert!(black_bishop.squares_occupied(&[C8, F8]));
    }

    #[test]
    fn test_initial_position_white_rooks() {
        let state: State = State::new();
        let white_rook = state.get_bitboard_for(Rook, White);
        assert!(white_rook.squares_occupied(&[A1, H1]));

        let black_rook = state.get_bitboard_for(Rook, Black);
        assert!(black_rook.squares_occupied(&[A8, H8]));
    }

    #[test]
    fn test_initial_position_white_queen() {
        let state: State = State::new();
        let white_queen = state.get_bitboard_for(Queen, White);
        assert!(white_queen.square_occupied(D1));
        let black_queen = state.get_bitboard_for(Queen, Black);
        assert!(black_queen.square_occupied(D8));
    }

    #[test]
    fn test_initial_position_white_king() {
        let state: State = State::new();
        let white_king = state.get_bitboard_for(King, White);
        assert!(white_king.square_occupied(E1));
        let black_king = state.get_bitboard_for(King, Black);
        assert!(black_king.square_occupied(E8));
    }
}
