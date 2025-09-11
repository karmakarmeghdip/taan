use librespot_core::cache::Cache;
use xilem::{
    WidgetView,
    core::{fork, one_of::OneOf3},
    tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender},
    view::{button, label, worker},
};

use crate::{spotify, state::App};

pub fn login_button(data: &mut App) -> impl WidgetView<App> + use<> {
    if !data.authenticating {
        if !data.spotify.is_logged_in() {
            OneOf3::A(button("Login with Spotify", |s: &mut App| {
                s.authenticating = true;
            }))
        } else {
            OneOf3::B(label(format!(
                "Hello, {}!!",
                data.spotify.get_username().unwrap_or("User".to_string())
            ))) // Add support to get user name and display it
        }
    } else {
        OneOf3::C(fork(
            button("Logging in, click to cancel", |s: &mut App| {
                s.authenticating = false
            }),
            worker(
                |proxy, mut rx: UnboundedReceiver<Cache>| async move {
                    let cache = rx.recv().await;
                    let cred = spotify::auth(cache).await.ok(); // Find a way to terminate this when unloaded
                    let res = proxy.message(cred);
                    if let Err(e) = res {
                        println!("Error sending creds to UI: {}", e);
                    }
                },
                |state: &mut App, tx: UnboundedSender<Cache>| {
                    if let Some(cache) = state.spotify.get_cache() {
                        let res = tx.send(cache.clone());
                        if let Err(e) = res {
                            println!("Couldn't send cache: {}", e)
                        }
                    }
                },
                |state: &mut App, c| {
                    state.spotify.set_creds(c);
                    state.authenticating = false;
                },
            ),
        ))
    }
}
