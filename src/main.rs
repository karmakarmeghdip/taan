use xilem::winit::error::EventLoopError;
use xilem::{EventLoop, WindowOptions, Xilem};

mod spotify;
mod state;
mod ui;

fn main() -> Result<(), EventLoopError> {
    let opts = WindowOptions::new("Native Spotify");
    let mut state = state::App::default();
    if let Err(e) = state.spotify.init() {
        println!("Failed to init: {}", e);
    }
    let app = Xilem::new_simple(state, ui::root, opts);
    app.run_in(EventLoop::with_user_event())?;
    Ok(())
}
