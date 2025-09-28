// TODO: remove
#![allow(dead_code)]
// spell-checker: words PNBRQKpnbrqk0-9

use core::str;
use std::str::{FromStr, Utf8Error};

use crate::chess_game::{Color, Piece, PieceKind, Position};

mod tests;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum BoardParseError {
    /// Invalid character (i.e. not `[PNBRQKpnbrqk0-9]`) found which was expected to represent a
    /// tile.
    InvalidTileCharacter(char),
    InvalidRowCount(usize),
    InvalidColumnCount(usize),
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum ParseError {
    /// The parsed buffer didn't contain valid UTF-8.
    Utf8Error(Utf8Error),
    /// The message string had an invalid message identifier. Contains that identifier.
    InvalidMessageId(String),
    /// The message string contained too few parts for it's message type. Contains the number of
    /// parts found including the message identifier.
    TooFewParts(usize),
    /// The message's move part didn't match the format. Contains the entire part (i.e. all
    /// characters between the two surrounding ':').
    InvalidMove(String),
    /// The message's game phase (AKA game state) part didn't match the format. Contains the entire
    /// part (i.e. all characters between the two surrounding ':').
    InvalidGamePhase(String),
    InvalidBoard(String, BoardParseError),
}

impl From<Utf8Error> for ParseError {
    fn from(value: Utf8Error) -> Self {
        Self::Utf8Error(value)
    }
}

// #[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct RawMessage<'a> {
    identifier: &'a str,
    rest: &'a str,
}

fn split_message(buffer: &'_ [u8; 128]) -> Result<RawMessage<'_>, ParseError> {
    let message: &str = str::from_utf8(buffer)?;
    let Some((identifier, rest)) = message.split_once(":") else {
        return Err(ParseError::TooFewParts(1));
    };

    Ok(RawMessage { identifier, rest })
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum GamePhase {
    Ongoing,
    WhiteWin,
    BlackWin,
    Draw,
}

pub struct FenBoardRowPieces<'a> {
    row: &'a str,
    /// How many tiles have been returned for the current first character.
    returned_tiles: u32,
}

impl<'a> FenBoardRowPieces<'a> {
    pub fn new(row: &'a str) -> Self {
        Self {
            row,
            returned_tiles: 0,
        }
    }
}

impl<'a> Iterator for FenBoardRowPieces<'a> {
    type Item = Result<Option<Piece>, BoardParseError>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.row.chars().next()? {
            empty_tiles_char @ '1'..='8' => {
                let empty_tiles_total = empty_tiles_char
                    .to_digit(10)
                    .expect("'1'..='8' is a digit in base 10");

                if self.returned_tiles + 1 >= empty_tiles_total {
                    let mut row_chars = self.row.chars();
                    row_chars.next();
                    self.row = row_chars.as_str();
                    self.returned_tiles = 0;
                } else {
                    self.returned_tiles += 1;
                }

                Some(Ok(None))
            }

            piece_char
            @ ('p' | 'n' | 'b' | 'r' | 'q' | 'k' | 'P' | 'N' | 'B' | 'R' | 'Q' | 'K') => {
                let color = if piece_char.is_ascii_uppercase() {
                    Color::White
                } else {
                    Color::Black
                };
                let kind = match piece_char.to_ascii_lowercase() {
                    'p' => PieceKind::Pawn,
                    'n' => PieceKind::Knight,
                    'b' => PieceKind::Bishop,
                    'r' => PieceKind::Rook,
                    'q' => PieceKind::Queen,
                    'k' => PieceKind::King,
                    _ => unreachable!(),
                };

                let mut row_chars = self.row.chars();
                row_chars.next();
                self.row = row_chars.as_str();

                Some(Ok(Some(Piece { kind, color })))
            }

            invalid_char => {
                let mut row_chars = self.row.chars();
                row_chars.next();
                self.row = row_chars.as_str();
                Some(Err(BoardParseError::InvalidTileCharacter(invalid_char)))
            }
        }
    }
}

/// Type representing a board position. It is structured in the same way as
/// `chess::game::game_state`.
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Board {
    pub board: Vec<Vec<String>>,
    pub player: Vec<Vec<char>>,
}

impl Board {
    pub fn new_empty() -> Self {
        let board = vec![vec!["empty".to_string(); 9]; 9];
        let player = vec![vec![' '; 9]; 9];
        Self { board, player }
    }

    pub fn tile(&self, tile: Position) -> Option<Piece> {
        let kind =
            match self.board[(tile.row() + 1) as usize][(tile.column() + 1) as usize].as_str() {
                "pawn" => PieceKind::Pawn,
                "knight" => PieceKind::Knight,
                "bishop" => PieceKind::Bishop,
                "rook" => PieceKind::Rook,
                "queen" => PieceKind::Queen,
                "king" => PieceKind::King,
                "empty" => return None,
                _ => unreachable!(),
            };

        let color = match self.player[(tile.row() + 1) as usize][(tile.column() + 1) as usize] {
            'w' => Color::White,
            'b' => Color::Black,
            ' ' => return None,
            _ => unreachable!(),
        };

        return Some(Piece { kind, color });
    }
    pub fn set_tile(&mut self, tile: Position, piece: Option<Piece>) {
        self.board[(tile.row() + 1) as usize][(tile.column() + 1) as usize] =
            match piece.map(|piece| piece.kind) {
                Some(PieceKind::Pawn) => "pawn".to_owned(),
                Some(PieceKind::Knight) => "knight".to_owned(),
                Some(PieceKind::Bishop) => "bishop".to_owned(),
                Some(PieceKind::Rook) => "rook".to_owned(),
                Some(PieceKind::Queen) => "queen".to_owned(),
                Some(PieceKind::King) => "king".to_owned(),
                None => "empty".to_owned(),
            };
        self.player[(tile.row() + 1) as usize][(tile.column() + 1) as usize] =
            match piece.map(|piece| piece.color) {
                Some(Color::White) => 'w',
                Some(Color::Black) => 'b',
                None => ' ',
            };
    }

    pub fn update_game(self, game: &mut chess::game::game_state) {
        game.board = self.board;
        game.player = self.player;
    }
}

impl FromStr for Board {
    type Err = BoardParseError;

    /// Parse from "piece placement data" part of FEN position notatation.
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.matches("/").count() + 1 {
            8 => {}
            rows => return Err(BoardParseError::InvalidRowCount(rows)),
        }

        let piece_rows = s
            .split("/")
            .enumerate()
            .map(|(i, row_str)| {
                let row_index = 7 - i;
                let pieces = FenBoardRowPieces::new(row_str)
                    .enumerate()
                    .map(move |(column_index, piece)| ((column_index, row_index), piece))
                    .collect::<Box<[_]>>();

                match pieces.len() {
                    8 => Ok(pieces),
                    rows => Err(BoardParseError::InvalidColumnCount(rows)),
                }
            })
            .collect::<Result<Box<[_]>, _>>()?;

        match piece_rows.len() {
            8 => {}
            rows => return Err(BoardParseError::InvalidRowCount(rows)),
        }

        let mut board = Self::new_empty();

        for ((column_index, row_index), piece) in piece_rows.into_iter().flatten() {
            let position = Position::new(column_index as u8, row_index as u8)
                .expect("columns and rows have been checked to be in 0..8");

            board.set_tile(position, piece?);
        }

        Ok(board)
    }
}

impl From<chess::game::game_state> for Board {
    fn from(value: chess::game::game_state) -> Self {
        Self {
            board: value.board,
            player: value.player,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct MoveMessage {
    source: Position,
    dest: Position,
    promotion: Option<PieceKind>,
    phase: GamePhase,
    board: Board,
}

impl FromStr for MoveMessage {
    type Err = ParseError;

    /// Parse from string, excluding the message identifier and first separator.
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split(":");

        let move_str = parts.next().expect("Split returns at least one element");
        let Some(phase_str) = parts.next() else {
            return Err(ParseError::TooFewParts(2));
        };
        let Some(board_str) = parts.next() else {
            return Err(ParseError::TooFewParts(3));
        };
        let Some(_padding_str) = parts.next() else {
            return Err(ParseError::TooFewParts(4));
        };

        // Parse move
        let (source, dest, promotion) = match *move_str.chars().collect::<Box<[char]>>() {
            [
                source_column_char @ 'A'..='H',
                source_row_char @ '1'..='9',
                dest_column_char @ 'A'..='H',
                dest_row_char @ '1'..='9',
                promotion_char,
            ] => {
                let Some(source) =
                    Position::parse(&format!("{}{}", source_column_char, source_row_char))
                else {
                    return Err(ParseError::InvalidMove(move_str.to_owned()));
                };
                let Some(dest) = Position::parse(&format!("{}{}", dest_column_char, dest_row_char))
                else {
                    return Err(ParseError::InvalidMove(move_str.to_owned()));
                };
                
                let promotion = match promotion_char.to_ascii_lowercase() {
                    '0' => None,
                    'p' => Some(PieceKind::Pawn),
                    'n' => Some(PieceKind::Knight),
                    'b' => Some(PieceKind::Bishop),
                    'r' => Some(PieceKind::Rook),
                    'q' => Some(PieceKind::Queen),
                    'k' => Some(PieceKind::King),
                    _ => return Err(ParseError::InvalidMove(move_str.to_owned())),
                };
                

                (source, dest, promotion)
            }
            _ => return Err(ParseError::InvalidMove(move_str.to_owned())),
        };

        // Parse game phase
        let phase = match phase_str {
            "0-0" => GamePhase::Ongoing,
            "1-0" => GamePhase::WhiteWin,
            "0-1" => GamePhase::BlackWin,
            "1-1" => GamePhase::Draw,
            _ => return Err(ParseError::InvalidGamePhase(phase_str.to_owned())),
        };

        let board = match board_str.parse::<Board>() {
            Ok(board) => board,
            Err(error) => return Err(ParseError::InvalidBoard(board_str.to_owned(), error)),
        };

        Ok(Self {
            source,
            dest,
            promotion,
            phase,
            board,
        })
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct QuitMessage {
    message: String,
}

impl FromStr for QuitMessage {
    type Err = ParseError;

    /// Parse from string, excluding the message identifier and first separator.
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split(":");

        let message = parts
            .next()
            .expect("Split returns at least one element")
            .to_owned();
        let Some(_padding_str) = parts.next() else {
            return Err(ParseError::TooFewParts(2));
        };

        Ok(Self { message })
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
enum Message {
    Move(MoveMessage),
    Quit(QuitMessage),
}

impl Message {
    pub fn parse_from(buffer: &[u8; 128]) -> Result<Self, ParseError> {
        let message = split_message(buffer)?;

        match message.identifier {
            "ChessMOVE" => Ok(Self::Move(message.rest.parse()?)),
            "ChessQUIT" => Ok(Self::Quit(message.rest.parse()?)),
            _ => Err(ParseError::InvalidMessageId(message.identifier.to_owned())),
        }
    }
}
