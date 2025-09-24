use image::EncodableLayout;
use slint::{ComponentHandle, SharedString};
use std::ops::Div;

pub fn play(ui: crate::MainWindow, spot: crate::spotify::SpotifyState) {
    let app = ui.global::<crate::AppState>();
    let ui = ui.clone_strong();
    app.on_play(move || {
        println!("Playing predefined music for testing");
        spot.player.play();
        let app = ui.global::<crate::AppState>();
        app.set_is_playing(true);
    });
}

pub fn pause(ui: crate::MainWindow, spot: crate::spotify::SpotifyState) {
    let app = ui.global::<crate::AppState>();
    let ui = ui.clone_strong();
    app.on_pause(move || {
        println!("Pausing music for testing");
        spot.player.pause();
        let app = ui.global::<crate::AppState>();
        app.set_is_playing(false);
    });
}

pub fn volume_changed(ui: crate::MainWindow, spot: crate::spotify::SpotifyState) {
    let app = ui.global::<crate::AppState>();
    app.on_volume_changed(move |v| {
        // spot.player
        //     .emit_volume_changed_event((v * 100.0).floor() as u16);
        // println!("Volume changed to {}", v);
        // TODO: Doesn't work for some reason, also need to implement debouncing
    });
}

pub fn player_event_handler(
    ui: crate::MainWindow,
    spot: crate::spotify::SpotifyState,
    rt: tokio::runtime::Handle,
) {
    let ui_weak = ui.as_weak();
    rt.spawn(async move {
        while let Some(e) = spot.player.get_player_event_channel().recv().await {
            match e {
                librespot_playback::player::PlayerEvent::TrackChanged { audio_item: track } => {
                    let cover_url = track.covers.first().map(|c| c.url.clone());
                    ui_weak
                        .upgrade_in_event_loop(move |ui| {
                            let app = ui.global::<crate::AppState>();
                            println!("Track changed to {:#?}", track);
                            set_track(&app, track);
                        })
                        .unwrap_or_else(|e| {
                            eprintln!("Failed to upgrade ui weak reference: {}", e)
                        });
                    if let Some(url) = cover_url {
                        let ui_weak = ui_weak.clone();
                        let url = url.to_string();
                        if let Err(e) = get_cover_image(ui_weak, &url).await {
                            eprintln!("Failed to get cover image: {}", e);
                        }
                    }
                }
                _ => {
                    println!("Player event received: {:#?}", e);
                }
            }
        }
    });
}

fn set_track(app: &crate::AppState, track: Box<librespot_metadata::audio::item::AudioItem>) {
    app.set_song_title(track.name.into());
    let duration = track.duration_ms.div(1000);
    let minutes = duration.div(60);
    let seconds = duration % 60;
    app.set_remaining_time(SharedString::from(format!("{}:{}", minutes, seconds)));
}

async fn get_cover_image(ui_weak: slint::Weak<crate::MainWindow>, url: &str) -> anyhow::Result<()> {
    let img =
        image::load_from_memory(reqwest::get(url).await?.bytes().await?.as_bytes())?.into_rgba8();
    let buf = slint::SharedPixelBuffer::clone_from_slice(img.as_raw(), img.width(), img.height());
    ui_weak.upgrade_in_event_loop(move |ui| {
        let app = ui.global::<crate::AppState>();
        app.set_album_art(slint::Image::from_rgba8(buf));
    })?;
    Ok(())
}
