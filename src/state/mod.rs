use slint::ComponentHandle;

mod app_state;

pub fn setup(
    ui: crate::MainWindow,
    spot: crate::spotify::SpotifyState,
    rt: tokio::runtime::Handle,
) {
    app_state::initialize_app(ui.clone_strong(), spot.clone(), rt.clone());
    let api = ui.global::<crate::SpotifyAPI>();
    let ui_weak = ui.as_weak();
    api.on_start_oauth_login(move || {
        let spot = spot.clone();
        let ui_weak = ui_weak.clone();
        rt.spawn(async move {
            if let Err(e) = spot.auth().await {
                eprintln!("Failed to login: {}", e);
            } else {
                ui_weak
                    .upgrade_in_event_loop(move |ui| {
                        let app = ui.global::<crate::AppState>();
                        app.set_is_logged_in(true);
                        app.set_current_view("player".into());
                        println!("Logged in slint event loop");
                    })
                    .unwrap();
            }
        });
        true
    });
}
