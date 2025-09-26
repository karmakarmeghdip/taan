use image::EncodableLayout;
use slint::ComponentHandle;

pub fn play(ui: slint::Weak<crate::MainWindow>, spot: crate::spotify::SpotifyState) {
    let ui = ui.unwrap();
    let app = ui.global::<crate::AppState>();
    app.on_play(move || {
        println!("Playing predefined music for testing");
        spot.player.play();
    });
}

pub fn pause(ui: slint::Weak<crate::MainWindow>, spot: crate::spotify::SpotifyState) {
    let ui = ui.unwrap();
    let app = ui.global::<crate::AppState>();
    app.on_pause(move || {
        println!("Pausing music for testing");
        spot.player.pause();
    });
}

pub fn seek(ui: slint::Weak<crate::MainWindow>, spot: crate::spotify::SpotifyState) {
    let ui = ui.unwrap();
    let app = ui.global::<crate::AppState>();
    app.on_seek(move |pos| {
        println!("Seeking to position {}", pos);
        spot.player.seek((pos * 1000) as u32);
    });
}

// pub fn volume_changed(ui: crate::MainWindow, spot: crate::spotify::SpotifyState) {
//     let app = ui.global::<crate::AppState>();
//     app.on_volume_changed(move |v| {
//         // spot.player
//         //     .emit_volume_changed_event((v * 100.0).floor() as u16);
//         // println!("Volume changed to {}", v);
//         // TODO: Doesn't work for some reason, also need to implement debouncing
//     });
// }

pub fn player_event_handler(
    ui: slint::Weak<crate::MainWindow>,
    spot: crate::spotify::SpotifyState,
    rt: tokio::runtime::Handle,
) {
    let ui_weak = ui;
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
                librespot_playback::player::PlayerEvent::Paused { position_ms, .. } => {
                    ui_weak
                        .upgrade_in_event_loop(move |ui| {
                            let app = ui.global::<crate::AppState>();
                            app.set_is_playing(false);
                            app.set_current_time((position_ms/1000) as i32);
                        })
                        .unwrap();
                }
                librespot_playback::player::PlayerEvent::Playing { position_ms, .. } => {
                    ui_weak
                        .upgrade_in_event_loop(move |ui| {
                            let app = ui.global::<crate::AppState>();
                            app.set_is_playing(true);
                            app.set_current_time((position_ms/1000) as i32);
                        })
                        .unwrap();
                }
                librespot_playback::player::PlayerEvent::Seeked { position_ms, .. } => {
                    ui_weak
                        .upgrade_in_event_loop(move |ui| {
                            let app = ui.global::<crate::AppState>();
                            app.set_current_time((position_ms/1000) as i32);
                        })
                        .unwrap();
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
    app.set_music_duration((track.duration_ms/1000) as i32);
    match track.unique_fields {
        librespot_metadata::audio::UniqueFields::Track { artists, album, .. } => {
            use librespot_protocol::metadata::artist_with_role::ArtistRole;
            let mut composers = vec![];
            for artist in artists.0 {
                if artist.role == ArtistRole::ARTIST_ROLE_MAIN_ARTIST {
                    app.set_artist_name(artist.name.into());
                } else if artist.role == ArtistRole::ARTIST_ROLE_COMPOSER {
                    composers.push(artist.name);
                }
            }
            let composer_str = composers.join(", ");
            app.set_composer(composer_str.into());
            app.set_album(album.into());
        }
        librespot_metadata::audio::UniqueFields::Episode { .. } => todo!(),
    }
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
