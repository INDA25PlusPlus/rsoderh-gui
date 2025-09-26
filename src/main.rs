use ggez::{conf, event};

use rsoderh_gui::{MainState, chess_game::GameUi};

pub fn main() -> ggez::GameResult {
    let min_size = GameUi::size();
    let cb = ggez::ContextBuilder::new("rsoderh_chess_gui", "ggez").window_mode(conf::WindowMode {
        width: min_size.x,
        height: min_size.y,
        resizable: true,
        // resize_on_scale_factor_change: true,
        ..Default::default()
    }).window_setup(conf::WindowSetup {
        title: "Rsoderh Chess".to_owned(),
        ..Default::default()
    });
    let (mut ctx, event_loop) = cb.build()?;
    let state = MainState::new(&mut ctx)?;
    event::run(ctx, event_loop, state)
}
