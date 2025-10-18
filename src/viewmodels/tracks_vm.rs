use crate::services::{rt, spotify, ui_weak};
use slint::ComponentHandle;
use crate::models::tracks::add_tracks;

pub fn register_handlers() -> anyhow::Result<()> {
    ui_weak().upgrade_in_event_loop(|ui| {
        let tracks = ui.global::<crate::TracksState>();
        tracks.on_track_clicked(|track| {
            log::info!("Track clicked: {}", track);
            spotify().load_track(track.into()).unwrap_or_else(|e| log::error!("Failed to load track: {}", e));
        });
        tracks.on_fetch_tracks(|plist| {
            log::info!("Fetch tracks: {}", plist);
        });
        tracks.on_fetch_saved_tracks(|| {
            rt().spawn(async {
                let user_tracks = spotify().get_saved_tracks().await.expect("Spotify client not initialized, fatal");
                let tracks: Vec<_> = user_tracks.into_iter().map(|item| item.track).collect();
                add_tracks(tracks).unwrap_or_else(|e| log::error!("Failed to add tracks: {}", e))
            });
        });
    })?;

    Ok(())
}
