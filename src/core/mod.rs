use std::fmt;

#[derive(Debug)]
pub enum Color {
    WHITE,
    BLACK,
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
        write!(
            f,
            "{}",
            match self {
                &PieceType::Pawn => "",
                &PieceType::Night => "N",
                &PieceType::Bishop => "B",
                &PieceType::Rook => "R",
                &PieceType::Queen => "Q",
                &PieceType::King => "K",
            }
        )
    }
}

#[derive(Debug)]
pub struct BoardLocation {
    name: String,
    rank: char,
    file: char,
}

impl BoardLocation {
    pub fn new(name: String) -> Option<BoardLocation> {
        if name.len() != 2 {
            return None;
        }
        let mut chars = name.chars();

        if let (Some(rank), Some(file)) = (chars.nth(0), chars.nth(1)) {
            Some(BoardLocation {
                name: name.clone(),
                rank: rank,
                file: file,
            })
        } else {
            None
        }
    }
}

impl fmt::Display for BoardLocation {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}
