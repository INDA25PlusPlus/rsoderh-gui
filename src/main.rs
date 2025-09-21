use ggez::event;

use rsoderh_gui::MainState;

pub fn main() -> ggez::GameResult {
    let cb = ggez::ContextBuilder::new("rsoderh_chess_gui", "ggez");
    let (ctx, event_loop) = cb.build()?;
    let state = MainState::new()?;
    event::run(ctx, event_loop, state)
}
