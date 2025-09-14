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
            Command::GetUserPlaylists => match spot.get_playlists(10, 0).await {
                Ok(list) => println!("{:#?}", list),
                Err(e) => println!("{}", e),
            },
        }
    }
}

pub(crate) fn handle_event(state: &mut App, msg: Event) {
    match msg {
        Event::Error(e) => {
            state.error = Some(e);
        }
        Event::LoginSuccess(u) => {
            state.authenticating = false;
            state.user = Some(u);
        }
        Event::NotLoggedIn => {
            state.authenticating = false;
        }
    }
}
#[derive(Debug)]
pub(crate) enum Event {
    Error(String),
    LoginSuccess(rspotify::model::PrivateUser),
    NotLoggedIn,
}

pub enum Command {
    AttemptOAuth,
    GetUserPlaylists,
}
