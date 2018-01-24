use std::cmp::PartialEq;
use std::fmt;

use chrono::prelude::*;

#[derive(Debug)]
pub enum GameResult { WhiteWon, BlackWon, Draw, InProgress, Abandoned, Unknown }

impl fmt::Display for GameResult {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", match self {
            &GameResult::WhiteWon => "1-0",
            &GameResult::BlackWon => "0-1",
            &GameResult::Draw => "1/2-1/2",
            _ => "*",
        })
    }
}

#[derive(Debug)]
pub enum Color {
    Black,
    White,
}

#[derive(Debug)]
pub enum PieceType {
    Pawn,
    Night,
    Bishop,
    Rook,
    Queen,
    King,
}

impl fmt::Display for PieceType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", match self {
            &PieceType::Pawn => "",
            &PieceType::Night => "N",
            &PieceType::Bishop => "B",
            &PieceType::Rook => "R",
            &PieceType::Queen => "Q",
            &PieceType::King => "K",
        })
    }
}

#[derive(Debug)]
pub struct Piece {
    piece_type: PieceType,
    color: Color,
}

#[derive(Debug)]
pub struct BoardLocation {
    rank: i32,
    file: char,
}

impl fmt::Display for BoardLocation {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}{}", self.file, self.rank)
    }
}

#[derive(Debug)]
pub struct HalfMove {
    piece: Piece,
    from: BoardLocation,
    to: BoardLocation,
}

#[derive(Debug)]
pub struct Move {
    number: i32,
    white: HalfMove,
    black: HalfMove,
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
            moves: Vec::new(),
        }
    }
}

impl PartialEq for Game {
    fn eq(&self, rhs: &Self) -> bool {
        // TODO: fix
        return true;
    }
}
