use std::{fmt::Debug, sync::Arc};

use ggez::{Context, GameResult, glam, graphics, mint};

use crate::{
    assets::Assets,
    chess_graphics::{BorderRadii, RoundedRectangle, SizedImage},
    palette::PALETTE,
    ui::{self, ButtonSpecialization, PressState},
};

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
    /// use rsoderh_chess::Position;
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

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct Piece {
    pub kind: PieceKind,
    pub color: Color,
}

struct Square {
    position: Position,
    board: Arc<BoardWrapper>,
    assets: Arc<Assets>,
}

impl Square {
    pub fn new(position: Position, board: Arc<BoardWrapper>, assets: Arc<Assets>) -> Self {
        Self {
            position,
            board,
            assets,
        }
    }
}

impl ButtonSpecialization for Square {
    fn draw(
        &self,
        ctx: &mut Context,
        canvas: &mut graphics::Canvas,
        bounds: graphics::Rect,
        press_state: PressState,
        hovered: bool,
    ) -> GameResult {
        static BOARD_CORNER_RADIUS: f32 = 15.0;
        // // let game = self.game.lock().unwrap();
        // // game.at(self.position);
        // // let mesh =
        // // self.game.get_mut();

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

        let mesh = RoundedRectangle::new(
            ctx,
            graphics::DrawMode::fill(),
            bounds,
            corner_radii,
            bg_color,
        )?;
        canvas.draw(&mesh, graphics::DrawParam::new());

        let label_string = match (self.position.column(), self.position.row()) {
            (0, row) => Some(format!("{}", row + 1)),
            (column, 0) => Some(format!(
                "{}",
                char::from_digit(column as u32 + 10, 20).expect("surely my math works")
            )),
            _ => None,
        };

        if let Some(label_string) = label_string {
            let position = if self.position.column() == 0 {
                // Showing row number
                // Note: that last offset is arbitrary
                glam::vec2(bounds.x + 15.0, bounds.y + 15.0 + 6.0)
            } else {
                // Showing column number
                glam::vec2(bounds.right() - 15.0, bounds.bottom() - 15.0)
            };

            let mut text = graphics::Text::new(label_string);
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

        if let Some(piece) = self.board.at(self.position) {
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
        println!("Pressed {:?}", self.position);
    }
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
}

pub struct Game {
    _state: Arc<BoardWrapper>,
    components: [ui::Button; 64],
}

impl Game {
    pub fn new(bounds: graphics::Rect, assets: &Arc<Assets>) -> Self {
        let state = Arc::new(BoardWrapper::new(chess::game::game_state::new()));
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

                    let mut square_bounds = bounds.clone();
                    square_bounds.scale(1.0 / 64.0, 1.0 / 64.0);

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

        Self {
            _state: state,
            components: *components,
        }
    }

    pub fn update_with_press_state(
        &mut self,
        position: glam::Vec2,
        press_state: PressState,
    ) -> bool {
        for button in self.components.iter_mut() {
            if button.update_with_press_state(position, press_state) {
                return true;
            }
        }

        false
    }

    pub fn update_with_mouse_position(&mut self, position: glam::Vec2) {
        for button in self.components.iter_mut() {
            button.update_with_mouse_position(position);
        }
    }

    pub fn draw(&self, ctx: &mut Context, canvas: &mut graphics::Canvas) -> GameResult {
        for component in &self.components {
            component.draw(ctx, canvas)?;
        }
        Ok(())
    }
}
