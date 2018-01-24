extern crate chess_pgn_parser;

use chess_pgn_parser::game::{Game, GameResult};

#[test]
fn termination() {
    assert_eq!(format!("{}", GameResult::WhiteWon), "1-0");
    assert_eq!(format!("{}", GameResult::BlackWon), "0-1");
    assert_eq!(format!("{}", GameResult::Draw), "1/2-1/2");
    assert_eq!(format!("{}", GameResult::InProgress), "*");
    assert_eq!(format!("{}", GameResult::Abandoned), "*");
    assert_eq!(format!("{}", GameResult::Unknown), "*");
}

#[test]
fn creating_games() {
    assert_eq!(Game::new(), Game {
        event: "".to_string(),
        site: "".to_string(),
        date: None,
        round: 0,
        white: Vec::new(),
        black: Vec::new(),
        result: None,
        moves: Vec::new(),
    });
}
