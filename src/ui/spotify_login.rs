use xilem::{
    WidgetView,
    core::{fork, one_of::OneOf3},
    tokio::sync::mpsc::UnboundedReceiver,
    view::{button, label, worker},
};

use crate::{App, spotify};

pub fn login_button(data: &mut App) -> impl WidgetView<App> + use<> {
    if !data.load_creds {
        if data.creds.is_none() {
            OneOf3::A(button("Login with Spotify", |s: &mut App| {
                s.load_creds = true;
            }))
        } else {
            OneOf3::B(label("Hello, User!!")) // Add support to get user name and display it
        }
    } else {
        OneOf3::C(fork(
            button("Logging in, click to cancel", |s: &mut App| {
                s.load_creds = false
            }),
            worker(
                |proxy, _: UnboundedReceiver<()>| async move {
                    let cred = spotify::auth().await.ok(); // Find a way to terminate this when unloaded
                    let res = proxy.message(cred);
                    if let Err(e) = res {
                        println!("Error sending creds to UI: {}", e);
                    }
                },
                |_, _| {},
                |state: &mut App, c| {
                    state.creds = c;
                    state.load_creds = false;
                },
            ),
        ))
    }
}
