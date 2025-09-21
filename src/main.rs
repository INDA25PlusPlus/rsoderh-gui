use ggez::event;

use rsoderh_gui::MainState;

pub fn main() -> ggez::GameResult {
    let cb = ggez::ContextBuilder::new("rsoderh_chess_gui", "ggez");
    let (mut ctx, event_loop) = cb.build()?;
    let state = MainState::new(&mut ctx)?;
    event::run(ctx, event_loop, state)
}
