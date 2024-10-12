use colored::Colorize;
use itertools::Itertools;
use strum::IntoEnumIterator;
use crate::board::state::*;

pub fn print(state: &State) {
    use Square::*;
    print_rank(state, &[A8, B8, C8, D8, E8, F8, G8, H8]);
    print_rank(state, &[A7, B7, C7, D7, E7, F7, G7, H7]);
    print_rank(state, &[A6, B6, C6, D6, E6, F6, G6, H6]);
    print_rank(state, &[A5, B5, C5, D5, E5, F5, G5, H5]);
    print_rank(state, &[A4, B4, C4, D4, E4, F4, G4, H4]);
    print_rank(state, &[A3, B3, C3, D3, E3, F3, G3, H3]);
    print_rank(state, &[A2, B2, C2, D2, E2, F2, G2, H2]);
    print_rank(state, &[A1, B1, C1, D1, E1, F1, G1, H1]);
}

// todo move printing to another struct
fn print_rank(state: &State, squares: &[Square]) {
    for square in squares {
        print_square(state, square);
    }
    print!("\n");
}

fn print_square(state: &State, square: &Square) {
    for (piece, color) in PieceType::iter().cartesian_product(Color::iter()) {
        if State::has_piece_on_square(state.get_bitboard_for(piece, color), square) {
            print_piece(piece, color);
            return;
        }
    }
    print!(". ");
}

fn print_piece(piece: PieceType, color: Color) {
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
