use xilem::core::fork;
use xilem::core::one_of::OneOf2;
use xilem::tokio::sync::mpsc::UnboundedReceiver;
use xilem::view::{Axis, button, flex, label, worker};
use xilem::winit::error::EventLoopError;
use xilem::{EventLoop, WidgetView, WindowOptions, Xilem};

mod spotify;

#[derive(Default)]
struct App {
    creds: Option<librespot_core::authentication::Credentials>,
    load_creds: bool,
}

fn app_logic(data: &mut App) -> impl WidgetView<App> + use<> {
    let user_id = match data.creds.as_ref() {
        Some(c) => c.username.clone(),
        None => None,
    };
    println!("User auth state: {}", data.creds.is_some());
    flex(
        Axis::Vertical,
        (
            label(match user_id {
                Some(n) => format!("{}", n),
                None => "Not Logged in".to_string(),
            }),
            if data.load_creds {
                OneOf2::A(fork(
                    button("Logging in, click to cancel", |s: &mut App| {
                        s.load_creds = false
                    }),
                    worker(
                        |proxy, _: UnboundedReceiver<()>| async move {
                            let cred = spotify::auth().await.ok();
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
            } else {
                OneOf2::B(button("Login with Spotify", |s: &mut App| {
                    s.load_creds = true;
                }))
            },
        ),
    )
}

fn main() -> Result<(), EventLoopError> {
    let app = Xilem::new_simple(App::default(), app_logic, WindowOptions::new("Counter app"));
    app.run_in(EventLoop::with_user_event())?;
    Ok(())
}
