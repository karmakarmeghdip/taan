use xilem::{
    WidgetView,
    core::fork,
    tokio::sync::mpsc::UnboundedReceiver,
    view::{Axis, flex, worker},
};

use crate::{spotify::SpotifyState, state::App};

mod spotify_login;

pub fn root(data: &mut App) -> impl WidgetView<App> + use<> {
    fork(
        flex(Axis::Vertical, (spotify_login::login_button(data),)),
        worker(
            |proxy, mut rx: UnboundedReceiver<Command>| async move {
                let spot = SpotifyState::default();
                if spot.is_logged_in() {
                    if let Err(e) = proxy.message(Event::LoginSuccess) {
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
                                if let Err(e) = proxy.message(Event::Error(e.to_string())) {
                                    println!("{}", e);
                                }
                            } else {
                                if let Err(e) = proxy.message(Event::LoginSuccess) {
                                    println!("{}", e);
                                }
                            }
                        }
                    }
                }
            },
            |state: &mut App, tx| {
                state.tx = Some(tx);
            },
            |state, msg: Event| match msg {
                Event::Error(e) => {
                    state.error = Some(e);
                }
                Event::LoginSuccess => {
                    state.authenticating = false;
                    state.logged_in = true;
                }
                Event::NotLoggedIn => {
                    state.authenticating = false;
                    state.logged_in = false;
                }
            },
        ),
    )
}

#[derive(Debug)]
enum Event {
    Error(String),
    LoginSuccess,
    NotLoggedIn,
}

pub enum Command {
    AttemptOAuth,
}
