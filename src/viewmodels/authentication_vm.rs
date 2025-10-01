use slint::ComponentHandle;

use crate::{
    models::authentication,
    services::{rt, spotify, ui_weak},
};

pub fn register_handlers() -> anyhow::Result<()> {
    let ui = ui_weak().unwrap();
    let app = ui.global::<crate::AppState>();
    app.on_login_clicked(move || {
        handle_login();
    });
    Ok(())
}

pub fn init() {
    rt().spawn(async {
        spotify()
            .init()
            .await
            .unwrap_or_else(|e| log::error!("Failed to init spotify client: {}", e));
        if let Err(e) = spotify().web_auth().await {
            log::error!("Failed to init spotify web api: {}", e);
            authentication::login_failed("Auto Login Failed").unwrap();
        } else {
            log::info!("Successfuly logged in");
            authentication::login_succeeded().unwrap();
        }
    });
}

pub fn handle_login() {
    rt().spawn(async move {
        authentication::login_started().unwrap();
        if let Err(e) = spotify().auth().await {
            log::error!("Failed to login: {}", e);
            authentication::login_failed("Failed to login").unwrap();
        } else {
            log::info!("Successfuly logged in");
            authentication::login_succeeded().unwrap();
        }
    });
}
