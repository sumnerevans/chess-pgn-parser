extern crate chess_pgn_parser;

use chess_pgn_parser::{Game, Result};

#[test]
fn termination() {
    assert_eq!(format!("{}", Result::WHITE_WON), "1-0");
    assert_eq!(format!("{}", Result::BLACK_WON), "0-1");
    assert_eq!(format!("{}", Result::DRAW), "1/2-1/2");
    assert_eq!(format!("{}", Result::IN_PROGRESS), "*");
    assert_eq!(format!("{}", Result::ABANDONED), "*");
    assert_eq!(format!("{}", Result::UNKNOWN), "*");
}

#[test]
fn creating_games() {
    assert_eq!(Game::new(), Game {
        result: None,
    });
}
