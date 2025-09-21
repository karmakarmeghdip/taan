use slint::ComponentHandle;

pub fn initialize_app(
    ui: crate::MainWindow,
    spot: crate::spotify::SpotifyState,
    rt: tokio::runtime::Handle,
) {
    let ui_weak = ui.as_weak();
    rt.spawn(async move {
        spot.init().await;
        let r = spot
            .web_auth()
            .await
            .inspect_err(|e| eprintln!("Failed to init spotify web api: {}", e));
        if r.is_ok() {
            ui_weak
                .upgrade_in_event_loop(|ui| {
                    let app = ui.global::<crate::AppState>();
                    app.set_is_logged_in(true);
                    app.set_current_view("player".into());
                    println!("Logged in slint event loop");
                })
                .unwrap();
        }
    });
}
