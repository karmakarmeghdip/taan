use xilem::{core::MessageProxy, tokio::sync::mpsc::UnboundedReceiver};

use crate::{
    spotify::SpotifyState,
    state::{App, UserData},
};

pub(crate) async fn run_spotify_loop(
    proxy: MessageProxy<Event>,
    mut rx: UnboundedReceiver<Command>,
) {
    let mut spot = SpotifyState::default();
    let _ = spot.connect().await.inspect_err(|e| println!("{}", e));
    if spot.is_logged_in() {
        if let Err(e) = proxy.message(Event::LoginSuccess(UserData {
            username: spot.get_username(),
        })) {
            println!("{}", e);
        }
    } else {
        if let Err(e) = proxy.message(Event::NotLoggedIn) {
            println!("{}", e);
        }
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
                        .message(Event::LoginSuccess(UserData {
                            username: spot.get_username(),
                        }))
                        .inspect_err(|e| println!("{}", e));
                }
            }
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
            state.logged_in = true;
            state.user = Some(u);
        }
        Event::NotLoggedIn => {
            state.authenticating = false;
            state.logged_in = false;
        }
    }
}
#[derive(Debug)]
pub(crate) enum Event {
    Error(String),
    LoginSuccess(UserData),
    NotLoggedIn,
}

pub enum Command {
    AttemptOAuth,
}
