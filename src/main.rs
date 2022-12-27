use ggez::event;
use ggez::GameResult;
use std::env;
use std::path;

mod ball;
mod constants;
mod paddle;

mod game;
use game::MainState;

pub fn main() -> GameResult {
    let resource_dir = if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
        let mut path = path::PathBuf::from(manifest_dir);
        path.push("resources");
        path
    } else {
        path::PathBuf::from("./resources")
    };

    let cb = ggez::ContextBuilder::new("Pong!", "leleneme")
        .window_setup(ggez::conf::WindowSetup::default().title("Pong!"))
        .add_resource_path(resource_dir);

    let (ctx, event_loop) = cb.build()?;
    let state = MainState::new(&ctx)?;

    event::run(ctx, event_loop, state)
}
