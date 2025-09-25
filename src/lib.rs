use std::sync::Arc;

use ggez::{GameError, GameResult, event, glam::Vec2, graphics};

use crate::{assets::Assets, chess_game::GameUi};

mod assets;
mod chess_game;
pub mod chess_graphics;
pub mod palette;
pub mod ui;

pub struct MainState {
    game: GameUi,
    // assets: Arc<Assets>,
}

impl MainState {
    pub fn new(ctx: &mut ggez::Context) -> GameResult<MainState> {
        let assets = Arc::new(Assets::new(ctx));
        let state = MainState {
            game: GameUi::new(
                graphics::Rect {
                    x: 10.0,
                    y: 10.0,
                    w: 100.0 * 64.0,
                    h: 100.0 * 64.0,
                },
                &assets,
            ),
            // assets,
        };

        Ok(state)
    }

    fn mouse_left_button_event(
        &mut self,
        _ctx: &mut ggez::Context,
        button: event::MouseButton,
        press_state: ui::PressState,
        x: f32,
        y: f32,
    ) {
        if button != event::MouseButton::Left {
            return;
        }

        self.game
            .update_with_press_state(Vec2::new(x, y), press_state);
    }
}

impl event::EventHandler<GameError> for MainState {
    fn update(&mut self, _ctx: &mut ggez::Context) -> GameResult<()> {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut ggez::Context) -> GameResult<()> {
        let mut canvas = graphics::Canvas::from_frame(ctx, graphics::Color::from_rgb_u32(0x2E2B28));

        self.game.draw(ctx, &mut canvas)?;

        canvas.finish(ctx)
    }

    fn mouse_button_down_event(
        &mut self,
        ctx: &mut ggez::Context,
        button: event::MouseButton,
        x: f32,
        y: f32,
    ) -> GameResult {
        self.mouse_left_button_event(ctx, button, ui::PressState::Pressed, x, y);
        Ok(())
    }
    fn mouse_button_up_event(
        &mut self,
        ctx: &mut ggez::Context,
        button: event::MouseButton,
        x: f32,
        y: f32,
    ) -> GameResult {
        self.mouse_left_button_event(ctx, button, ui::PressState::Released, x, y);
        Ok(())
    }
    fn mouse_motion_event(
        &mut self,
        _ctx: &mut ggez::Context,
        x: f32,
        y: f32,
        _dx: f32,
        _dy: f32,
    ) -> Result<(), GameError> {
        self.game.update_with_mouse_position(Vec2::new(x, y));
        Ok(())
    }
}
