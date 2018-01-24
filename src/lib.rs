extern crate chrono;

use std::cmp::PartialEq;
use std::fmt;

use chrono::prelude::*;

#[derive(Debug)]
pub enum Result { WHITE_WON, BLACK_WON, DRAW, IN_PROGRESS, ABANDONED, UNKNOWN }

impl fmt::Display for Result {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", match self {
            &Result::WHITE_WON => "1-0",
            &Result::BLACK_WON => "0-1",
            &Result::DRAW => "1/2-1/2",
            _ => "*",
        })
    }
}

#[derive(Debug)]
pub struct Move {}

#[derive(Debug)]
pub struct Game {
    // Seven-Tag Roster (STR)
    pub event: String,
    pub site: String,
    pub date: DateTime<Local>,
    pub round: i32,
    pub white: Vec<String>,
    pub black: Vec<String>,
    pub result: Option<Result>,

    // Move Data
    pub moves: Vec<Move>,
}

impl Game {
    pub fn new() -> Game {
        Game {
            result: None,
        }
    }
}

impl PartialEq for Game {
    fn eq(&self, rhs: &Self) -> bool {
        return true;
    }
}
