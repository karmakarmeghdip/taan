use slint::ComponentHandle;

pub fn start_oauth_login(
    ui: slint::Weak<crate::MainWindow>,
    spot: crate::spotify::SpotifyState,
    rt: tokio::runtime::Handle,
) {
    let ui_weak = ui.clone();
    let ui = ui.unwrap();
    let app = ui.global::<crate::AppState>();
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
