mod auth;
mod player;
mod ui;
#[cfg(not(target_os = "android"))]
mod window;

pub fn setup(
    ui: slint::Weak<crate::MainWindow>,
    spot: crate::spotify::SpotifyState,
    rt: tokio::runtime::Handle,
) {
    initialize_app(&ui, spot.clone(), &rt);
    #[cfg(not(target_os = "android"))]
    {
        window::drag_window(&ui);
        window::close_window(&ui);
    }
    auth::start_oauth_login(&ui, spot.clone(), rt.clone());
    player::play(&ui, spot.clone());
    player::pause(&ui, spot.clone());
    player::seek(&ui, spot.clone());
    player::player_event_handler(&ui, spot.clone(), rt.clone());
}

pub fn initialize_app(
    ui: &slint::Weak<crate::MainWindow>,
    spot: crate::spotify::SpotifyState,
    rt: &tokio::runtime::Handle,
) {
    let ui = ui.clone();
    rt.spawn(async move {
        spot.init()
            .await
            .unwrap_or_else(|e| eprintln!("Failed to init spotify client: {}", e));
        ui::set_loading(&ui, false);
        if let Err(e) = spot.web_auth().await {
            eprintln!("Failed to init spotify web api: {}", e)
        } else {
            ui::set_logged_in(&ui, true);
            // Play some music for testing
            // Load testing track just to have something to play/pause
            spot.load_track("spotify:track:30aPCMAtkH6Cf5ejzY4cE4".to_string())
                .unwrap_or_else(|e| eprintln!("Failed to load testing track: {}", e));
        }
    });
}
