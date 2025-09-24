use ggez::{conf, event};

use rsoderh_gui::MainState;

pub fn main() -> ggez::GameResult {
    let cb = ggez::ContextBuilder::new("rsoderh_chess_gui", "ggez").window_mode(conf::WindowMode {
        width: 1200.0,
        height: 1000.0,
        ..Default::default()
    });
    let (mut ctx, event_loop) = cb.build()?;
    let state = MainState::new(&mut ctx)?;
    event::run(ctx, event_loop, state)
}
