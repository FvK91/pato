use crate::board::state::Color::White;
use crate::board::state::PieceType::Knight;

mod board;

fn main() {
    let state = board::state::State::new();
    board::display::print_board(&state);
}
