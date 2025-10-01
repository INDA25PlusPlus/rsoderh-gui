#[allow(unused_imports)]
use super::*;

// spell-checker: words rnbqkbnr, RNBQKB1R, attäck

#[test]
fn fen_row_iter() {
    assert_eq!(
        FenBoardRowPieces::new("4P2k").collect::<Vec<_>>(),
        vec![
            Ok(None),
            Ok(None),
            Ok(None),
            Ok(None),
            Ok(Some(Piece {
                kind: PieceKind::Pawn,
                color: Color::White
            })),
            Ok(None),
            Ok(None),
            Ok(Some(Piece {
                kind: PieceKind::King,
                color: Color::Black
            })),
        ]
    )
}

#[test]
fn fen_row_invalid_iter() {
    assert_eq!(
        FenBoardRowPieces::new("rNö2").collect::<Vec<_>>(),
        vec![
            Ok(Some(Piece {
                kind: PieceKind::Rook,
                color: Color::Black
            })),
            Ok(Some(Piece {
                kind: PieceKind::Knight,
                color: Color::White
            })),
            Err(BoardParseError::InvalidTileCharacter('ö')),
            Ok(None),
            Ok(None),
        ]
    )
}

#[test]
fn board_parse_standard() {
    assert_eq!(
        "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR".parse::<Board>(),
        Ok(chess::game::game_state::new().into()),
    )
}

#[test]
fn board_parse_too_few_columns() {
    assert_eq!(
        "rnbqkbnr/pp1ppppp/8/2p5/4P3/5N2/PPPP1P/RNBQKB1R".parse::<Board>(),
        Err(BoardParseError::InvalidColumnCount(6)),
    )
}

#[test]
fn board_parse_too_many_columns() {
    assert_eq!(
        "rnbqkbnr/pp1ppppp/8/2p5/4P3/5N2/PPPP2PPP/RNBQKB1R".parse::<Board>(),
        Err(BoardParseError::InvalidColumnCount(9)),
    )
}

#[test]
fn board_parse_too_few_rows() {
    assert_eq!(
        "rnbqkbnr/pp1ppppp/8/2p5/4P3/8/PPPP1PPP".parse::<Board>(),
        Err(BoardParseError::InvalidRowCount(7)),
    )
}

#[test]
fn board_parse_too_many_rows() {
    assert_eq!(
        "rnbqkbnr/pp1ppppp/8/2p5/4P3/5N2/PPPP1PPP/RNBQKB1R/RNBQKB1R".parse::<Board>(),
        Err(BoardParseError::InvalidRowCount(9)),
    )
}

#[test]
fn message_parse_quit() {
    let message_bytes = b"ChessQUIT:I had a panic att\xC3\xA4ck:000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000";

    assert_eq!(
        Message::parse_from(message_bytes),
        Ok(Message::Quit(QuitMessage {
            message: String::from("I had a panic attäck")
        }))
    )
}

#[test]
fn message_parse_quit_empty() {
    let message_bytes = b"ChessQUIT::000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000";

    assert_eq!(
        Message::parse_from(message_bytes),
        Ok(Message::Quit(QuitMessage {
            message: String::from("")
        }))
    )
}

#[test]
fn message_parse_move() {
    let message_bytes = b"ChessMOVE:E2E40:0-0:rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR:0000000000000000000000000000000000000000000000000000000000000000";

    assert_eq!(
        Message::parse_from(message_bytes),
        Ok(Message::Move(MoveMessage {
            source: Position::parse("e2").unwrap(),
            dest: Position::parse("e4").unwrap(),
            promotion: None,
            phase: GamePhase::Ongoing,
            board: chess::game::game_state::new().into()
        }))
    )
}

#[test]
fn message_parse_move_draw() {
    let message_bytes = b"ChessMOVE:E2E40:1-1:rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR:0000000000000000000000000000000000000000000000000000000000000000";

    assert_eq!(
        Message::parse_from(message_bytes),
        Ok(Message::Move(MoveMessage {
            source: Position::parse("e2").unwrap(),
            dest: Position::parse("e4").unwrap(),
            promotion: None,
            phase: GamePhase::Draw,
            board: chess::game::game_state::new().into()
        }))
    )
}

#[test]
fn message_parse_move_black_won() {
    let message_bytes = b"ChessMOVE:E2E40:0-1:rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR:0000000000000000000000000000000000000000000000000000000000000000";

    assert_eq!(
        Message::parse_from(message_bytes),
        Ok(Message::Move(MoveMessage {
            source: Position::parse("e2").unwrap(),
            dest: Position::parse("e4").unwrap(),
            promotion: None,
            phase: GamePhase::Win(Color::Black),
            board: chess::game::game_state::new().into()
        }))
    )
}

#[test]
fn message_parse_move_promotion() {
    let message_bytes = b"ChessMOVE:E2E4k:1-0:rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR:0000000000000000000000000000000000000000000000000000000000000000";

    assert_eq!(
        Message::parse_from(message_bytes),
        Ok(Message::Move(MoveMessage {
            source: Position::parse("e2").unwrap(),
            dest: Position::parse("e4").unwrap(),
            promotion: Some(PieceKind::King),
            phase: GamePhase::Win(Color::White),
            board: chess::game::game_state::new().into()
        }))
    )
}

#[test]
fn message_serialize_move_ongoing() {
    let message = Message::Move(MoveMessage {
        source: Position::parse("e2").unwrap(),
        dest: Position::parse("e4").unwrap(),
        promotion: None,
        phase: GamePhase::Ongoing,
        board: chess::game::game_state::new().into(),
    });

    assert_eq!(Message::parse_from(&message.serialize()), Ok(message),)
}

#[test]
fn message_serialize_move_white_win() {
    let message = Message::Move(MoveMessage {
        source: Position::parse("e2").unwrap(),
        dest: Position::parse("e4").unwrap(),
        promotion: None,
        phase: GamePhase::Win(Color::White),
        board: chess::game::game_state::new().into(),
    });

    assert_eq!(Message::parse_from(&message.serialize()), Ok(message),)
}

#[test]
fn message_serialize_move_black_win() {
    let message = Message::Move(MoveMessage {
        source: Position::parse("e2").unwrap(),
        dest: Position::parse("e4").unwrap(),
        promotion: None,
        phase: GamePhase::Win(Color::Black),
        board: chess::game::game_state::new().into(),
    });

    assert_eq!(Message::parse_from(&message.serialize()), Ok(message),)
}

#[test]
fn message_serialize_move_draw() {
    let message = Message::Move(MoveMessage {
        source: Position::parse("e2").unwrap(),
        dest: Position::parse("e4").unwrap(),
        promotion: None,
        phase: GamePhase::Draw,
        board: chess::game::game_state::new().into(),
    });

    assert_eq!(Message::parse_from(&message.serialize()), Ok(message),)
}

#[test]
fn message_serialize_move_empty_board() {
    let message = Message::Move(MoveMessage {
        source: Position::parse("e2").unwrap(),
        dest: Position::parse("e4").unwrap(),
        promotion: None,
        phase: GamePhase::Draw,
        board: Board::new_empty(),
    });

    assert_eq!(Message::parse_from(&message.serialize()), Ok(message),)
}

#[test]
fn message_serialize_quit_empty() {
    let message = Message::Quit(QuitMessage {
        message: "".to_owned(),
    });

    assert_eq!(Message::parse_from(&message.serialize()), Ok(message),)
}

#[test]
fn message_serialize_quit_unicode() {
    let message = Message::Quit(QuitMessage {
        message: "I need hëlp =( 🇸🇪".to_owned(),
    });

    assert_eq!(Message::parse_from(&message.serialize()), Ok(message),)
}
