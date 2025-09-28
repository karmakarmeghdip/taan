use slint::ComponentHandle;

use crate::services::{spotify, ui_weak};

pub fn register_handlers() -> anyhow::Result<()> {
    let ui = ui_weak().unwrap();
    let app = ui.global::<crate::AppState>();
    app.on_play(|| {
        spotify().player.play();
    });
    app.on_pause(|| {
        spotify().player.pause();
    });
    app.on_seek(|pos| {
        spotify().player.seek((pos * 1000) as u32);
    });
    Ok(())
}
