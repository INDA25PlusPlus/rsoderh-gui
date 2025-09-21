use ggez::{GameError, GameResult, event, glam::Vec2, graphics};

use crate::ui::MouseColors;

pub mod ui;

pub struct MainState {
    buttons: Vec<ui::Button>,
}

impl MainState {
    pub fn new() -> GameResult<MainState> {
        let mut state = MainState {
            buttons: Vec::new(),
        };

        state.add_button(ui::Button::new(
            graphics::Rect {
                x: 10.0,
                y: 10.0,
                w: 150.0,
                h: 100.0,
            },
            ui::RoundedButton::new(
                10.0,
                MouseColors::new(
                    graphics::Color::from_rgb_u32(0x22211E),
                    graphics::Color::from_rgb_u32(0x393734),
                    graphics::Color::from_rgb_u32(0x1b1a18),
                ),
                || println!("Clicked!"),
            ),
        ));

        Ok(state)
    }

    pub fn add_button(&mut self, button: ui::Button) {
        self.buttons.push(button);
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

        for button in self.buttons.iter_mut().rev() {
            if button.update_with_press_state(Vec2::new(x, y), press_state) {
                break;
            }
        }
    }
}

impl event::EventHandler<GameError> for MainState {
    fn update(&mut self, _ctx: &mut ggez::Context) -> GameResult<()> {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut ggez::Context) -> GameResult<()> {
        let mut canvas = graphics::Canvas::from_frame(ctx, graphics::Color::from_rgb_u32(0x2E2B28));

        for button in &mut self.buttons {
            button.draw(ctx, &mut canvas)?;
        }

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
        for button in &mut self.buttons {
            button.update_with_mouse_position(Vec2::new(x, y));
        }
        Ok(())
    }
}
