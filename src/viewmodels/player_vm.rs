use slint::ComponentHandle;

use crate::{
    models::player,
    services::{rt, spotify, ui_weak},
};

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
    rt().spawn(async {
        spotify()
            .on_player_event(|e| {
                handle_player_event(e);
            })
            .await;
    });
    Ok(())
}

fn handle_player_event(event: librespot_playback::player::PlayerEvent) {
    match event {
        librespot_playback::player::PlayerEvent::PlayRequestIdChanged { play_request_id } => {
            println!("Play request id changed to {}", play_request_id);
        }
        librespot_playback::player::PlayerEvent::Stopped { track_id, .. } => {
            println!("Stopping playback of {}", track_id);
            player::pause().unwrap();
        }
        librespot_playback::player::PlayerEvent::Loading {
            track_id,
            position_ms,
            ..
        } => {
            println!("Buffering for {}", track_id);
            player::set_position(position_ms).unwrap();
        }
        librespot_playback::player::PlayerEvent::Preloading { track_id } => {
            println!("Preloading track: {}", track_id);
        }
        librespot_playback::player::PlayerEvent::Playing {
            track_id,
            position_ms,
            ..
        } => {
            println!("Resuming playback of {}", track_id);
            player::set_position(position_ms).unwrap();
            player::play().unwrap();
        }
        librespot_playback::player::PlayerEvent::Paused {
            track_id,
            position_ms,
            ..
        } => {
            println!("Paused playback of {}", track_id);
            player::pause().unwrap();
            player::set_position(position_ms).unwrap();
        }
        librespot_playback::player::PlayerEvent::TimeToPreloadNextTrack { track_id, .. } => {
            println!("Preloading track: {}", track_id);
            spotify().player.preload(track_id);
        }
        librespot_playback::player::PlayerEvent::EndOfTrack { track_id, .. } => {
            println!("Track finished for {}", track_id);
            player::pause().unwrap();
            player::set_position(0).unwrap();
        }
        librespot_playback::player::PlayerEvent::Unavailable { track_id, .. } => {
            println!("Track unavailable: {}", track_id);
        }
        librespot_playback::player::PlayerEvent::PositionCorrection { position_ms, .. } => {
            player::set_position(position_ms).unwrap();
        }
        librespot_playback::player::PlayerEvent::PositionChanged { position_ms, .. } => {
            player::set_position(position_ms).unwrap();
        }
        librespot_playback::player::PlayerEvent::Seeked { position_ms, .. } => {
            player::set_position(position_ms).unwrap();
        }
        librespot_playback::player::PlayerEvent::TrackChanged { audio_item } => {
            println!("{:#?}", audio_item);
            if let Some(url) = audio_item.covers.first() {
                let url = url.url.clone();
                rt().spawn(async move {
                    match spotify().fetch_cover_art(url).await {
                        Ok(img) => player::set_cover_art(img).unwrap(),
                        Err(e) => eprintln!("Failed to fetch cover art: {}", e),
                    }
                });
            }
            player::set_track_details(audio_item).unwrap();
        }
        librespot_playback::player::PlayerEvent::SessionConnected {
            connection_id,
            user_name,
        } => {
            println!("Session connected: {} ({})", user_name, connection_id); // show username in ui later
        }
        librespot_playback::player::PlayerEvent::SessionDisconnected {
            connection_id,
            user_name,
        } => {
            println!("Session disconnected: {} ({})", user_name, connection_id); // remove username from ui later
        }
        librespot_playback::player::PlayerEvent::SessionClientChanged {
            client_id,
            client_name,
            client_brand_name,
            client_model_name,
        } => {
            println!(
                "Session client changed: {} ({}, {}, {})",
                client_id, client_name, client_brand_name, client_model_name
            );
        }
        // librespot_playback::player::PlayerEvent::ShuffleChanged { shuffle } => todo!(),
        // librespot_playback::player::PlayerEvent::RepeatChanged { context, track } => todo!(),
        // librespot_playback::player::PlayerEvent::AutoPlayChanged { auto_play } => todo!(),
        // librespot_playback::player::PlayerEvent::FilterExplicitContentChanged { filter } => todo!(),
        // librespot_playback::player::PlayerEvent::VolumeChanged { volume } => todo!(),
        _ => {
            println!("{:#?}", event);
        }
    }
}
