extern crate chess;

use chess::core::{BoardLocation, PieceType};
use chess::game::{Game, GameResult};

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
fn piece_rep() {
    assert_eq!(format!("{}", PieceType::Pawn), "");
    assert_eq!(format!("{}", PieceType::Night), "N");
    assert_eq!(format!("{}", PieceType::Bishop), "B");
    assert_eq!(format!("{}", PieceType::Rook), "R");
    assert_eq!(format!("{}", PieceType::Queen), "Q");
    assert_eq!(format!("{}", PieceType::King), "K");
}

#[test]
fn board_location_rep() {
    let a4 = BoardLocation::new("a4".to_string()).unwrap();
    let e8 = BoardLocation::new("e8".to_string()).unwrap();
    assert_eq!(format!("{}", a4), "a4");
    assert_eq!(format!("{}", e8), "e8");
}
