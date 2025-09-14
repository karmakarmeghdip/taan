use rspotify::model::SimplifiedPlaylist;
use xilem::{core::MessageProxy, tokio::sync::mpsc::UnboundedReceiver};

use crate::{spotify::SpotifyState, state::App};

pub(crate) async fn run_spotify_loop(
    proxy: MessageProxy<Event>,
    mut rx: UnboundedReceiver<Command>,
) {
    let mut spot = SpotifyState::default();
    let _ = spot.connect().await.inspect_err(|e| println!("{}", e));
    if spot.is_logged_in() {
        proxy
            .message(Event::LoginSuccess(
                spot.get_me().await.expect("Auth token, fatal"),
            ))
            .expect("IPC error, fatal");
    } else {
        proxy.message(Event::NotLoggedIn).expect("IPC error, fatal");
    }
    while let Some(cmd) = rx.recv().await {
        match cmd {
            Command::AttemptOAuth => {
                if let Err(e) = spot.auth().await {
                    let _ = proxy
                        .message(Event::Error(e.to_string()))
                        .inspect_err(|e| println!("{}", e));
                } else {
                    let _ = proxy
                        .message(Event::LoginSuccess(
                            spot.get_me().await.expect("Auth token, fatal"),
                        ))
                        .inspect_err(|e| println!("{}", e));
                }
            }
            Command::GetUserPlaylists => match spot.get_user_playlists(10, 0).await {
                Ok(list) => proxy
                    .message(Event::UserPlaylists(list))
                    .expect("IPC error, fatal"),
                Err(e) => proxy
                    .message(Event::Error(e.to_string()))
                    .expect("IPC error, fatal"),
            },
            Command::GetPlaylistTracks(playlist_id) => {
                match spot.get_playlist(playlist_id, 10, 0).await {
                    Ok(list) => proxy
                        .message(Event::PlaylistItems(list))
                        .expect("IPC error, fatal"),
                    Err(e) => proxy
                        .message(Event::Error(e.to_string()))
                        .expect("IPC error, fatal"),
                }
            }
            Command::PlayTrack(i) => {
                let _ = spot.play_track(i).inspect_err(|e| {
                    proxy
                        .message(Event::Error(e.to_string()))
                        .expect("IPC error, fatal")
                });
            }
            Command::Pause => {
                spot.pause();
            }
        }
    }
}

pub(crate) fn handle_event(state: &mut App, msg: Event) {
    match msg {
        Event::Error(e) => {
            println!("An error occured: {}", e);
            state.error = Some(e);
        }
        Event::LoginSuccess(u) => {
            state.authenticating = false;
            state.user = Some(u);
        }
        Event::NotLoggedIn => {
            state.authenticating = false;
        }
        Event::UserPlaylists(playlists) => {
            state.playlists = Some(playlists);
        }
        Event::PlaylistItems(items) => {
            state.playlist_item = Some(items);
        }
    }
}
#[derive(Debug)]
pub(crate) enum Event {
    Error(String),
    LoginSuccess(rspotify::model::PrivateUser),
    UserPlaylists(Vec<SimplifiedPlaylist>),
    PlaylistItems(Vec<rspotify::model::PlaylistItem>),
    NotLoggedIn,
}

pub enum Command {
    AttemptOAuth,
    GetUserPlaylists,
    GetPlaylistTracks(rspotify::model::PlaylistId<'static>),
    PlayTrack(String),
    Pause,
}
