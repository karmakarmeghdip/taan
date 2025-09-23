use i_slint_backend_winit::WinitWindowAccessor;
use slint::ComponentHandle;

pub fn setup(
    ui: crate::MainWindow,
    spot: crate::spotify::SpotifyState,
    rt: tokio::runtime::Handle,
) {
    initialize_app(ui.clone_strong(), spot.clone(), rt.clone());
    drag_window(ui.clone_strong());
    close_window(ui.clone_strong());
    start_oauth_login(ui.clone_strong(), spot.clone(), rt.clone());
    play(ui.clone_strong(), spot.clone());
    pause(ui.clone_strong(), spot.clone());
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

pub fn drag_window(ui: crate::MainWindow) {
    let ui_weak = ui.as_weak();
    let app = ui.global::<crate::AppState>();
    app.on_start_drag(move || {
        ui_weak.unwrap().window().with_winit_window(|win| {
            win.drag_window()
                .unwrap_or_else(|e| eprintln!("Failed to drag window: {}", e));
        });
    });
}

pub fn close_window(ui: crate::MainWindow) {
    let app = ui.global::<crate::AppState>();
    app.on_close_window(move || {
        // ui_weak.unwrap().hide().unwrap();
        slint::quit_event_loop().unwrap();
    });
}
pub fn start_oauth_login(
    ui: crate::MainWindow,
    spot: crate::spotify::SpotifyState,
    rt: tokio::runtime::Handle,
) {
    let app = ui.global::<crate::AppState>();
    let ui_weak = ui.as_weak();
    app.on_login_clicked(move || {
        let spot = spot.clone();
        let ui_weak = ui_weak.clone();
        rt.spawn(async move {
            ui_weak
                .upgrade_in_event_loop(|ui| {
                    let app = ui.global::<crate::AppState>();
                    app.set_login_in_progress(true);
                })
                .unwrap();
            if let Err(e) = spot.auth().await {
                eprintln!("Failed to login: {}", e);
            } else {
                ui_weak
                    .upgrade_in_event_loop(move |ui| {
                        let app = ui.global::<crate::AppState>();
                        app.set_loggedIn(true);
                        println!("Logged in slint event loop");
                    })
                    .unwrap();
            }
        });
    });
}
pub fn play(ui: crate::MainWindow, spot: crate::spotify::SpotifyState) {
    let app = ui.global::<crate::AppState>();
    let ui = ui.clone_strong();
    app.on_play(move || {
        println!("Playing predefined music for testing");
        spot.player.play();
        let app = ui.global::<crate::AppState>();
        app.set_is_playing(true);
    });
}

pub fn pause(ui: crate::MainWindow, spot: crate::spotify::SpotifyState) {
    let app = ui.global::<crate::AppState>();
    let ui = ui.clone_strong();
    app.on_pause(move || {
        println!("Pausing music for testing");
        spot.player.pause();
        let app = ui.global::<crate::AppState>();
        app.set_is_playing(false);
    });
}
