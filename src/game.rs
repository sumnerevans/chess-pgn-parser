use std::cmp::PartialEq;
use std::fmt;

use chrono::prelude::*;
use core::{BoardLocation, Color, PieceType};

#[derive(Debug)]
pub enum GameResult {
    WhiteWon,
    BlackWon,
    Draw,
    InProgress,
    Abandoned,
    Unknown,
}

impl fmt::Display for GameResult {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                &GameResult::WhiteWon => "1-0",
                &GameResult::BlackWon => "0-1",
                &GameResult::Draw => "1/2-1/2",
                _ => "*",
            }
        )
    }
}

#[derive(Debug)]
pub struct Piece {
    pub piece_type: PieceType,
    pub color: Color,
}

#[derive(Debug)]
pub struct HalfMove {
    pub piece: Piece,
    pub from: BoardLocation,
    pub to: BoardLocation,
}

#[derive(Debug)]
pub struct Move {
    pub number: i32,
    pub white: HalfMove,
    pub black: HalfMove,
}

#[derive(Debug)]
pub struct Tag {
    pub name: String,
    pub value: String,
}

#[derive(Debug)]
pub struct Game {
    // Seven-Tag Roster (STR)
    pub event: String,
    pub site: String,
    pub date: Option<Date<Local>>,
    pub round: i32,
    pub white: Vec<String>,
    pub black: Vec<String>,
    pub result: Option<GameResult>,

    pub tags: Vec<Tag>,

    // Move Data
    pub moves: Vec<Move>,
}

impl Game {
    pub fn new() -> Game {
        Game {
            event: "".to_string(),
            site: "".to_string(),
            date: None,
            round: 0,
            white: Vec::new(),
            black: Vec::new(),
            result: None,
            tags: Vec::new(),
            moves: Vec::new(),
        }
    }

    pub fn from_pgn(pgn_text: String) -> Game {
        println!("{}", pgn_text);
        Game::new()
    }
}

impl PartialEq for Game {
    fn eq(&self, rhs: &Self) -> bool {
        // TODO: fix
        return true;
    }
}
