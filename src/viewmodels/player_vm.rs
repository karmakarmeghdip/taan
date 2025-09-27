use slint::ComponentHandle;

use crate::services::spotify::SPOTIFY_SERVICE;

pub fn register_handlers() -> anyhow::Result<()> {
    let ui = crate::UI.get().unwrap().unwrap();
    let app = ui.global::<crate::AppState>();
    app.on_play(|| {
        SPOTIFY_SERVICE.get().unwrap().player.play();
    });
    app.on_pause(|| {
        SPOTIFY_SERVICE.get().unwrap().player.pause();
    });
    app.on_seek(|pos| {
        SPOTIFY_SERVICE
            .get()
            .unwrap()
            .player
            .seek((pos * 1000) as u32);
    });
    Ok(())
}
