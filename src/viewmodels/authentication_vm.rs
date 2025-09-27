use slint::ComponentHandle;

use crate::models::authentication;

pub fn register_handlers() -> anyhow::Result<()> {
    let ui = crate::UI.get().unwrap().unwrap();
    let app = ui.global::<crate::AppState>();
    app.on_login_clicked(move || {
        handle_login();
    });
    Ok(())
}

pub fn init() {
    let rt = crate::RT.get().unwrap();
    rt.spawn(async {
        let spot = crate::services::spotify::SPOTIFY_SERVICE.get().unwrap();
        spot.init()
            .await
            .unwrap_or_else(|e| eprintln!("Failed to init spotify client: {}", e));
        if let Err(e) = spot.web_auth().await {
            eprintln!("Failed to init spotify web api: {}", e);
            authentication::login_failed("Auto Login Failed").unwrap();
        } else {
            println!("Successfuly logged in");
            authentication::login_succeeded().unwrap();
        }
    });
}

pub fn handle_login() {
    let rt = crate::RT.get().unwrap();
    rt.spawn(async move {
        let spot = crate::services::spotify::SPOTIFY_SERVICE.get().unwrap();
        authentication::login_started().unwrap();
        if let Err(e) = spot.auth().await {
            eprintln!("Failed to login: {}", e);
            authentication::login_failed("Failed to login").unwrap();
        } else {
            println!("Successfuly logged in");
            authentication::login_succeeded().unwrap();
        }
    });
}
