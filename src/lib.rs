use std::sync::Arc;

use ggez::{
    GameError, GameResult, event,
    glam::{self, Vec2},
    graphics,
    winit::dpi::PhysicalSize,
};

use crate::{assets::Assets, chess_game::GameUi};

mod assets;
pub mod chess_game;
pub mod chess_graphics;
pub mod network;
pub mod palette;
mod rect;
pub mod ui;

pub struct MainState {
    game: GameUi,
    // connection: Arc<RefCell<network::GameConnection>>,
    // assets: Arc<Assets>,
}

impl MainState {
    pub fn new(
        ctx: &mut ggez::Context,
        connection: network::GameConnection,
    ) -> GameResult<MainState> {
        let assets = Arc::new(Assets::new(ctx));
        let state = MainState {
            game: GameUi::new(ctx, glam::vec2(10.0, 10.0), &assets, connection)?,
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

    /// Calculates the appropriate offset to keep the `GameUi` struct centered in within the window.
    fn center_offset(&self, ctx: &ggez::Context) -> glam::Vec2 {
        // let window_size = ctx.gfx.window().inner_size().to_logical::<f32>(ctx.gfx.window().scale_factor());
        let window_size = ctx.gfx.window().inner_size().cast::<f32>();

        glam::vec2(window_size.width / 2.0 - GameUi::size().x / 2.0, 0.0)
    }
}

impl event::EventHandler<GameError> for MainState {
    fn update(&mut self, ctx: &mut ggez::Context) -> GameResult<()> {
        let min_size = GameUi::size();
        ctx.gfx
            .window()
            .set_min_inner_size(Some(PhysicalSize::new(min_size.x, min_size.y)));

        self.game.update();

        Ok(())
    }

    fn draw(&mut self, ctx: &mut ggez::Context) -> GameResult<()> {
        let mut canvas = graphics::Canvas::from_frame(ctx, graphics::Color::from_rgb_u32(0x2E2B28));

        self.game.draw(ctx, &mut canvas, self.center_offset(ctx))?;

        canvas.finish(ctx)
    }

    fn quit_event(&mut self, _ctx: &mut ggez::Context) -> Result<bool, GameError> {
        println!("Quiting...");

        self.game.quit_event().unwrap_or_else(|error| {
            println!("Sending quit failed: {}", error);
        });

        Ok(false)
    }

    fn mouse_button_down_event(
        &mut self,
        ctx: &mut ggez::Context,
        button: event::MouseButton,
        x: f32,
        y: f32,
    ) -> GameResult {
        self.mouse_left_button_event(
            ctx,
            button,
            ui::PressState::Pressed,
            x - self.center_offset(ctx).x,
            y,
        );
        Ok(())
    }
    fn mouse_button_up_event(
        &mut self,
        ctx: &mut ggez::Context,
        button: event::MouseButton,
        x: f32,
        y: f32,
    ) -> GameResult {
        self.mouse_left_button_event(
            ctx,
            button,
            ui::PressState::Released,
            x - self.center_offset(ctx).x,
            y,
        );
        Ok(())
    }
    fn mouse_motion_event(
        &mut self,
        ctx: &mut ggez::Context,
        x: f32,
        y: f32,
        _dx: f32,
        _dy: f32,
    ) -> Result<(), GameError> {
        self.game
            .update_with_mouse_position(Vec2::new(x, y) - self.center_offset(ctx));
        Ok(())
    }
}
