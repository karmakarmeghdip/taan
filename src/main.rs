use xilem::winit::error::EventLoopError;
use xilem::{EventLoop, WindowOptions, Xilem};

mod spotify;
mod state;
mod ui;

fn main() -> Result<(), EventLoopError> {
    let opts = WindowOptions::new("Native Spotify");
    let state = state::App {
        authenticating: true,
        ..Default::default()
    };
    let app = Xilem::new_simple(state, ui::root, opts);
    app.run_in(EventLoop::with_user_event())?;
    Ok(())
}
