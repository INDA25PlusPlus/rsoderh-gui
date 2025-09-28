use std::{cell::RefCell, fmt::Debug, sync::Arc};

use ggez::{Context, GameResult, glam, graphics, mint};

use crate::{
    assets::Assets,
    chess_graphics::{BorderRadii, RoundedRectangle, SizedImage},
    palette::PALETTE,
    rect::RectUtils,
    ui::{self, ButtonSpecialization, PressState},
};

/// Corner radius of the board.
static BOARD_CORNER_RADIUS: f32 = 15.0;
/// Distance from the board top edge to the screen edge.
static BOARD_Y_MARGIN: f32 = 40.0;

/// Represents a coordinate on a chess board. Wrapper around u8 guaranteed to be within 0..8
/// (exclusive).
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct PositionIndex(u8);

impl PositionIndex {
    pub fn new(value: u8) -> Option<Self> {
        match value {
            0..8 => Some(Self(value)),
            _ => None,
        }
    }

    pub fn parse(string: &str) -> Option<Self> {
        let char_ = string.chars().next()?;
        match char_ {
            column_char @ ('a'..='h' | 'A'..='H') => {
                Self::new((column_char.to_digit(18)? - 10) as u8)
            }
            row_char @ ('1'..='8') => Self::new((row_char.to_digit(10)? - 1) as u8),
            _ => None,
        }
    }

    pub fn get(&self) -> u8 {
        self.0
    }
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Position {
    pub column: PositionIndex,
    pub row: PositionIndex,
}

impl Position {
    pub fn new(column: u8, row: u8) -> Option<Self> {
        Self::from_pair((column, row))
    }
    pub fn from_pair(pair: (u8, u8)) -> Option<Self> {
        Some(Position {
            column: PositionIndex::new(pair.0)?,
            row: PositionIndex::new(pair.1)?,
        })
    }
    pub fn as_other_color(self) -> Self {
        Self {
            row: PositionIndex::new(7 - self.row.0).unwrap(),
            ..self
        }
    }
    pub fn column(self) -> u8 {
        self.column.0
    }
    pub fn row(self) -> u8 {
        self.row.0
    }
    pub fn as_pair(self) -> (u8, u8) {
        (self.column(), self.row())
    }
    pub fn translated(self, translation: (i8, i8)) -> Option<Self> {
        Self::new(
            (self.column.0 as i8 + translation.0) as u8,
            (self.row.0 as i8 + translation.1) as u8,
        )
    }

    /// Parse Position from string like "a1". Is case insensitive.
    /// ```
    /// use rsoderh_gui::chess_game::Position;
    ///
    /// assert_eq!(Position::parse("a1").unwrap(), Position::new(0, 0).unwrap());
    /// ```
    pub fn parse(string: &str) -> Option<Self> {
        let chars: Vec<char> = string.chars().collect();
        match chars[..] {
            [
                column_char @ ('a'..='h' | 'A'..='H'),
                row_char @ ('1'..='8'),
            ] => Self::new(
                (column_char.to_digit(18)? - 10) as u8,
                (row_char.to_digit(10)? - 1) as u8,
            ),
            _ => None,
        }
    }
}

impl From<Position> for glam::Vec2 {
    fn from(value: Position) -> Self {
        Self::new(value.column() as f32, value.row() as f32)
    }
}

impl From<Position> for mint::Vector2<f32> {
    fn from(value: Position) -> Self {
        Self {
            x: value.column() as f32,
            y: value.row() as f32,
        }
    }
}

impl Debug for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let column = char::from_digit((self.column.0 as u32) + 10, 18)
            .expect("Position to hold value in valid range");
        let row = char::from_digit(self.row.0 as u32 + 1, 10)
            .expect("Position to hold value in valid range");

        write!(f, "{}{}", column, row)
    }
}

#[derive(Clone, Copy, Eq, PartialEq, Hash, Debug)]
pub enum Color {
    White,
    Black,
}

impl Color {
    pub fn opposite(&self) -> Self {
        // crate::span::Span
        match self {
            Color::White => Color::Black,
            Color::Black => Color::White,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum PieceKind {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Piece {
    pub kind: PieceKind,
    pub color: Color,
}

struct Square {
    position: Position,
    state: Arc<RefCell<GameState>>,
    assets: Arc<Assets>,
}

impl Square {
    pub fn new(position: Position, state: Arc<RefCell<GameState>>, assets: Arc<Assets>) -> Self {
        Self {
            position,
            state,
            assets,
        }
    }
}

impl ButtonSpecialization for Square {
    fn draw(
        &self,
        ctx: &mut Context,
        canvas: &mut graphics::Canvas,
        offset: glam::Vec2,
        mut bounds: graphics::Rect,
        press_state: PressState,
        hovered: bool,
    ) -> GameResult {
        // // let game = self.game.lock().unwrap();
        // // game.at(self.position);
        // // let mesh =
        // // self.game.get_mut();
        bounds.translate(offset);

        let to_actual_color = move |square_color: Color| match square_color {
            Color::White => match press_state {
                PressState::Pressed => PALETTE.board_square_white_active,
                PressState::Released if hovered => PALETTE.board_square_white_hover,
                PressState::Released => PALETTE.board_square_white,
            },
            Color::Black => match press_state {
                PressState::Pressed => PALETTE.board_square_black_active,
                PressState::Released if hovered => PALETTE.board_square_black_hover,
                PressState::Released => PALETTE.board_square_black,
            },
        };

        let square_color = if (self.position.column() + self.position.row()) % 2 == 0 {
            Color::White
        } else {
            Color::Black
        };

        let bg_color = to_actual_color(square_color);

        let corner_radii = match (self.position.column(), self.position.row()) {
            (0, 7) => BorderRadii {
                top_left: BOARD_CORNER_RADIUS,
                ..BorderRadii::zero()
            },
            (7, 7) => BorderRadii {
                top_right: BOARD_CORNER_RADIUS,
                ..BorderRadii::zero()
            },
            (0, 0) => BorderRadii {
                bottom_left: BOARD_CORNER_RADIUS,
                ..BorderRadii::zero()
            },
            (7, 0) => BorderRadii {
                bottom_right: BOARD_CORNER_RADIUS,
                ..BorderRadii::zero()
            },
            _ => BorderRadii::zero(),
        };

        // Draw bg tile.
        let mesh = RoundedRectangle::new(
            ctx,
            graphics::DrawMode::fill(),
            bounds,
            corner_radii,
            bg_color,
        )?;
        canvas.draw(&mesh, graphics::DrawParam::new());

        // Draw label text.
        if self.position.column() == 0 {
            // Showing row number
            let string = format!("{}", self.position.row() + 1);

            // Note: that last offset is arbitrary
            let position = glam::vec2(bounds.x + 15.0, bounds.y + 15.0 + 6.0);

            let mut text = graphics::Text::new(string);
            // TODO: Choose font. See ggez text example for how to load it.
            text.set_scale(30.0)
                .set_bounds(glam::vec2(30.0, 30.0))
                .set_layout(graphics::TextLayout::center());

            canvas.draw(
                &text,
                graphics::DrawParam::new()
                    .dest(position)
                    .color(to_actual_color(square_color.opposite())),
            );
        }
        if self.position.row() == 0 {
            // Showing column number
            let string = format!(
                "{}",
                char::from_digit(self.position.column() as u32 + 10, 20)
                    .expect("surely my math works")
            );

            let position = glam::vec2(bounds.right() - 15.0, bounds.bottom() - 15.0);

            let mut text = graphics::Text::new(string);
            // TODO: Choose font. See ggez text example for how to load it.
            text.set_scale(30.0)
                .set_bounds(glam::vec2(30.0, 30.0))
                .set_layout(graphics::TextLayout::center());

            canvas.draw(
                &text,
                graphics::DrawParam::new()
                    .dest(position)
                    .color(to_actual_color(square_color.opposite())),
            );
        }

        // Draw highlight if selected.
        if self
            .state
            .borrow()
            .last_move
            .into_iter()
            .map(|(source, dest)| [source, dest])
            .flatten()
            .chain(match self.state.borrow().phase {
                GamePhase::SelectDest(source) => Some(source),
                _ => None,
            })
            .any(|square| self.position == square)
        {
            let mut color = PALETTE.board_square_selected;
            color.a = 0.7;
            let mesh = RoundedRectangle::new(
                ctx,
                graphics::DrawMode::fill(),
                bounds,
                corner_radii,
                color,
            )?;
            canvas.draw(&mesh, graphics::DrawParam::new());
        }

        // Draw potential destination highlight.
        let phase = self.state.borrow().phase;
        let is_potential_dest = phase.source_square().is_some_and(|source| {
            self.state
                .borrow_mut()
                .board
                .valid_moves(source)
                .any(|dest| self.position == dest)
        });
        if is_potential_dest {
            let mesh = if self.state.borrow().board.at(self.position).is_some() {
                // Square is occupied.
                graphics::Mesh::new_circle(
                    ctx,
                    graphics::DrawMode::stroke(10.0),
                    bounds.center(),
                    bounds.w / 2.0 - 5.0,
                    0.001,
                    PALETTE.highlight_subtle_overlay,
                )?
            } else {
                // Square is empty.
                graphics::Mesh::new_circle(
                    ctx,
                    graphics::DrawMode::fill(),
                    bounds.center(),
                    20.0,
                    0.001,
                    PALETTE.highlight_subtle_overlay,
                )?
            };
            canvas.draw(&mesh, graphics::DrawParam::new());
        }

        // Draw piece graphic.
        if let Some(piece) = self.state.borrow().board.at(self.position) {
            static PIECE_SCALE: f32 = 0.9;

            let mut piece_bounds = bounds.clone();
            piece_bounds.scale(PIECE_SCALE, PIECE_SCALE);
            let size: glam::Vec2 = bounds.size().into();
            piece_bounds.translate((size - size * PIECE_SCALE) / 2.0);
            let image = SizedImage::new(
                self.assets.piece(piece.color, piece.kind),
                piece_bounds.size().into(),
            );
            canvas.draw(&image, piece_bounds.point());
        }

        Ok(())
    }

    fn on_press(&mut self) {
        self.state.borrow_mut().select_square(self.position);
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum MoveOutcome {
    Valid,
    Check,
    Checkmate,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum MoveError {
    BadCoordinates,
    WrongPlayer,
    Invalid,
    Checked,
}

/// Wrapper around `chess::chess::game::game_state`.
pub struct BoardWrapper(chess::game::game_state);

impl BoardWrapper {
    pub fn new(game_state: chess::game::game_state) -> Self {
        Self(game_state)
    }

    pub fn at(&self, position: Position) -> Option<Piece> {
        if self
            .0
            .empty(position.row() as i32 + 1, position.column() as i32 + 1)
        {
            return None;
        }

        let piece_str = self
            .0
            .get_piece(position.row() as i32 + 1, position.column() as i32 + 1);
        let color_str = self
            .0
            .get_player(position.row() as i32 + 1, position.column() as i32 + 1);

        Some(Piece {
            kind: match piece_str.as_str() {
                "pawn" => PieceKind::Pawn,
                "knight" => PieceKind::Knight,
                "bishop" => PieceKind::Bishop,
                "rook" => PieceKind::Rook,
                "king" => PieceKind::King,
                "queen" => PieceKind::Queen,
                _ => unreachable!(),
            },
            color: match color_str {
                'w' => Color::White,
                'b' => Color::Black,
                _ => unreachable!(),
            },
        })
    }

    pub fn turn(&self) -> Color {
        match self.0.turn {
            'w' => Color::White,
            'b' => Color::Black,
            _ => unreachable!(),
        }
    }

    pub fn valid_moves(&mut self, square: Position) -> impl Iterator<Item = Position> {
        self.0
            .valid_moves(square.row() as i32 + 1, square.column() as i32 + 1)
            .into_iter()
            .map(|(row, column)| {
                Position::new(column as u8 - 1, row as u8 - 1)
                    .expect("library returns valid positions")
            })
    }

    pub fn make_move(
        &mut self,
        source: Position,
        dest: Position,
    ) -> Result<MoveOutcome, MoveError> {
        use chess::outcome::Outcome;
        match self.0.make_move(
            source.row() as i32 + 1,
            source.column() as i32 + 1,
            dest.row() as i32 + 1,
            dest.column() as i32 + 1,
        ) {
            Outcome::Valid => Ok(MoveOutcome::Valid),
            Outcome::Check => Ok(MoveOutcome::Check),
            Outcome::Checkmate => Ok(MoveOutcome::Checkmate),
            Outcome::Bad_coordinates => Err(MoveError::BadCoordinates),
            Outcome::Wrong_player => Err(MoveError::WrongPlayer),
            Outcome::Invalid => Err(MoveError::Invalid),
            Outcome::Checked => Err(MoveError::Checked),
        }
    }
}

/// The phase of an ongoing turn, or if the game isn't active (TODO: implement game over state).
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum GamePhase {
    SelectSource,
    /// When a piece is being moved and a source square has been selected.
    SelectDest(Position),
}

impl GamePhase {
    pub fn source_square(&self) -> Option<Position> {
        match self {
            Self::SelectDest(square) => Some(*square),
            _ => None,
        }
    }
}

struct GameState {
    pub board: BoardWrapper,
    /// When a piece is being moved, this contains the coordinates of the square which was selected
    /// as the source.
    /// Information about the last half move. Contains the source and destination squares.
    pub last_move: Option<(Position, Position)>,
    pub phase: GamePhase,
}

impl GameState {
    pub fn new(board: BoardWrapper) -> Self {
        Self {
            board,
            last_move: None,
            phase: GamePhase::SelectSource,
        }
    }

    pub fn select_square(&mut self, square: Position) {
        match self.phase {
            GamePhase::SelectSource => {
                if self
                    .board
                    .at(square)
                    .is_none_or(|piece| piece.color != self.board.turn())
                {
                    return;
                }

                self.phase = GamePhase::SelectDest(square);
            }
            GamePhase::SelectDest(source) => {
                match self.board.make_move(source, square) {
                    Err(error) => {
                        // Interrpret as canceling the move.
                        println!("Invalid move: {:?}", error);

                        self.phase = GamePhase::SelectSource;
                    }
                    Ok(outcome) => {
                        dbg!(outcome);

                        self.last_move = Some((source, square));
                        self.phase = GamePhase::SelectSource;
                    }
                }
            }
        }
    }
}

pub struct GameUi {
    state: Arc<RefCell<GameState>>,
    board_bounds: graphics::Rect,
    square_buttons: [ui::Button; 64],
    white_label: graphics::Text,
    black_label: graphics::Text,
}

impl GameUi {
    pub fn new(_ctx: &mut Context, top_left: glam::Vec2, assets: &Arc<Assets>) -> GameResult<Self> {
        let state = Arc::new(RefCell::new(GameState::new(BoardWrapper::new(
            chess::game::game_state::new(),
        ))));
        let board_bounds = graphics::Rect {
            x: top_left.x,
            y: top_left.y + BOARD_Y_MARGIN,
            w: 100.0 * 8.0,
            h: 100.0 * 8.0,
        };
        let state_ref = &state;
        let components: Box<[_; 64]> = std::iter::repeat_n(0..8, 8)
            .enumerate()
            .flat_map(|(row_index, column_indices)| {
                column_indices.map(move |column_index| {
                    let position = Position::new(column_index as u8, row_index as u8)
                        .expect("indices are < 8");

                    let position_indices: glam::Vec2 = position.into();
                    let position_indices =
                        (glam::vec2(0.0, 7.0) - position_indices) * glam::vec2(-1.0, 1.0);

                    let mut square_bounds = board_bounds.clone();
                    square_bounds.scale(1.0 / 8.0, 1.0 / 8.0);

                    let square_size: glam::Vec2 = square_bounds.size().into();
                    square_bounds.translate(position_indices * square_size);

                    ui::Button::new(
                        square_bounds,
                        Square::new(position, state_ref.clone(), assets.clone()),
                    )
                })
            })
            .collect::<Box<[_]>>()
            .try_into()
            .ok()
            .expect("there are 64 position");

        let mut white_label = graphics::Text::new("White");
        white_label.set_scale(graphics::PxScale::from(35.0));
        // .set_bounds(glam::vec2(board_bounds.w, BOARD_Y_MARGIN))
        // .set_layout(graphics::TextLayout {
        //     h_align: graphics::TextAlign::Begin,
        //     v_align: graphics::TextAlign::Middle,
        // });
        let mut black_label = graphics::Text::new("Black");
        black_label.set_scale(graphics::PxScale::from(35.0));
        // .set_bounds(glam::vec2(board_bounds.w, BOARD_Y_MARGIN))
        // .set_layout(graphics::TextLayout {
        //     h_align: graphics::TextAlign::Begin,
        //     v_align: graphics::TextAlign::Middle,
        // });
        // let white_label = TextLabel::new(
        //     ctx,
        //     "White",
        //     glam::vec2(
        //         bounds.left(),
        //         bounds.top() + BOARD_Y_OFFSET - BOARD_Y_OFFSET / 2.0,
        //     ),
        //     bounds.w,
        //     10.0,
        //     BOARD_CORNER_RADIUS,
        //     PALETTE.button,
        //     graphics::Color::WHITE,
        // )?;
        // let black_label = TextLabel::new(
        //     ctx,
        //     "Black",
        //     glam::vec2(
        //         bounds.left(),
        //         bounds.bottom() + BOARD_Y_OFFSET + BOARD_Y_OFFSET / 2.0,
        //     ),
        //     bounds.w,
        //     10.0,
        //     BOARD_CORNER_RADIUS,
        //     PALETTE.button,
        //     graphics::Color::WHITE,
        // )?;

        Ok(Self {
            state,
            square_buttons: *components,
            white_label,
            black_label,
            board_bounds,
        })
    }

    pub fn update_with_press_state(
        &mut self,
        position: glam::Vec2,
        press_state: PressState,
    ) -> bool {
        for button in self.square_buttons.iter_mut() {
            if button.update_with_press_state(position, press_state) {
                return true;
            }
        }

        false
    }

    pub fn update_with_mouse_position(&mut self, position: glam::Vec2) {
        for button in self.square_buttons.iter_mut() {
            button.update_with_mouse_position(position);
        }
    }

    pub fn draw(
        &self,
        ctx: &mut Context,
        canvas: &mut graphics::Canvas,
        offset: glam::Vec2,
    ) -> GameResult {
        // Draw board squares.
        for component in &self.square_buttons {
            component.draw(ctx, canvas, offset)?;
        }

        // ctx.gfx.window().scale_factor()

        Self::size();

        // Draw player labels.
        canvas.draw(
            &self.black_label,
            graphics::DrawParam::new()
                .dest(self.board_bounds.top_left() + glam::vec2(10.0, -35.0 - 5.0) + offset)
                .color(PALETTE.board_square_black),
        );
        canvas.draw(
            &self.white_label,
            graphics::DrawParam::new()
                .dest(self.board_bounds.bottom_left() + glam::vec2(10.0, 5.0 + 3.0) + offset)
                .color(PALETTE.board_square_white),
        );

        let side_bar_bounds = graphics::Rect {
            x: self.board_bounds.right(),
            y: self.board_bounds.top(),
            w: 300.0,
            h: self.board_bounds.h,
        };
        static SIDE_BAR_TOP_MARGIN: f32 = 30.0;

        // Draw turn display.
        let (turn_str, turn_color) = match self.state.borrow().board.turn() {
            Color::White => ("White", PALETTE.board_square_white),
            Color::Black => ("Black", PALETTE.board_square_black),
        };
        let mut turn_text = graphics::Text::new(turn_str);
        turn_text
            .set_scale(graphics::PxScale::from(80.0))
            .set_layout(graphics::TextLayout {
                h_align: graphics::TextAlign::Middle,
                v_align: graphics::TextAlign::Begin,
            });

        canvas.draw(
            &turn_text,
            graphics::DrawParam::new().color(turn_color).dest(
                glam::vec2(
                    side_bar_bounds.center().x,
                    side_bar_bounds.top() + SIDE_BAR_TOP_MARGIN,
                ) + offset,
            ),
        );

        let mut subtitle_text = graphics::Text::new("to move");
        subtitle_text
            .set_scale(graphics::PxScale::from(30.0))
            .set_layout(graphics::TextLayout {
                h_align: graphics::TextAlign::Middle,
                v_align: graphics::TextAlign::Begin,
            });

        canvas.draw(
            &subtitle_text,
            graphics::DrawParam::new().color(PALETTE.text_subtle).dest(
                glam::vec2(
                    side_bar_bounds.center().x,
                    side_bar_bounds.top() + SIDE_BAR_TOP_MARGIN + 80.0,
                ) + offset,
            ),
        );

        Ok(())
    }

    pub fn size() -> glam::Vec2 {
        /// If I don't add this to the height the text at the bottom is cut off slightly, and I
        /// can't be bothered to fix it.
        static MAGIC_EXTRA_HEIGHT: f32 = 20.0;
        glam::vec2(
            800.0 + 300.0,
            800.0 + BOARD_Y_MARGIN * 2.0 + MAGIC_EXTRA_HEIGHT,
        )
    }
}
