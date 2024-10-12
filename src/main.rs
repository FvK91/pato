mod board;
fn main() {
    let state = board::state::State::new();
    board::display::print(&state);
}
