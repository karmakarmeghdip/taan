use slint::ComponentHandle;

mod auth;
mod player;
mod window;

pub fn setup(
    ui: crate::MainWindow,
    spot: crate::spotify::SpotifyState,
    rt: tokio::runtime::Handle,
) {
    initialize_app(ui.clone_strong(), spot.clone(), rt.clone());
    window::drag_window(ui.clone_strong());
    window::close_window(ui.clone_strong());
    auth::start_oauth_login(ui.clone_strong(), spot.clone(), rt.clone());
    player::play(ui.clone_strong(), spot.clone());
    player::pause(ui.clone_strong(), spot.clone());
    player::volume_changed(ui.clone_strong(), spot.clone());
    player::player_event_handler(ui.clone_strong(), spot.clone(), rt.clone());
}

pub fn initialize_app(
    ui: crate::MainWindow,
    spot: crate::spotify::SpotifyState,
    rt: tokio::runtime::Handle,
) {
    let ui_weak = ui.as_weak();
    rt.spawn(async move {
        let _ = spot
            .init()
            .await
            .inspect_err(|e| eprintln!("Failed to init spotify client: {}", e));
        ui_weak
            .upgrade_in_event_loop(|ui| {
                let app = ui.global::<crate::AppState>();
                app.set_loading(false);
                println!("Initialized slint event loop");
            })
            .unwrap();
        let r = spot
            .web_auth()
            .await
            .inspect_err(|e| eprintln!("Failed to init spotify web api: {}", e));
        if r.is_ok() {
            ui_weak
                .upgrade_in_event_loop(|ui| {
                    let app = ui.global::<crate::AppState>();
                    app.set_loggedIn(true);
                    println!("Logged in slint event loop");
                })
                .unwrap();
            // Play some music for testing
            // Load testing track just to have something to play/pause
            spot.load_track("spotify:track:4fnskJdNDDh27vBhsvXChn".to_string())
                .unwrap_or_else(|e| eprintln!("Failed to load testing track: {}", e));
        }
    });
}
