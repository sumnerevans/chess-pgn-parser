extern crate chess;

use chess::game::Game;

#[test]
fn parse_pgn() {
    let game = Game::from_pgn("ohea".to_string());
}
