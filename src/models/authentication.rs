use slint::ComponentHandle;

use crate::UI;

/**
 * Can be called from any thread
 */
pub fn login_succeeded() -> anyhow::Result<()> {
    UI.get().unwrap().upgrade_in_event_loop(|ui| {
        let app_state = ui.global::<crate::AppState>();
        app_state.set_loggedIn(true);
        app_state.set_loading(false);
        app_state.set_login_in_progress(false);
    })?;
    Ok(())
}
/**
 * Can be called from any thread
 */
pub fn login_failed(error: &str) -> anyhow::Result<()> {
    UI.get().unwrap().upgrade_in_event_loop(|ui| {
        let app_state = ui.global::<crate::AppState>();
        app_state.set_loggedIn(false);
        app_state.set_loading(false);
        app_state.set_login_in_progress(false);
        // TODO: Add error handling in UI
    })?;
    Ok(())
}
/**
 * Can be called from any thread
 */
pub fn login_started() -> anyhow::Result<()> {
    UI.get().unwrap().upgrade_in_event_loop(|ui| {
        let app_state = ui.global::<crate::AppState>();
        app_state.set_login_in_progress(true);
    })?;
    Ok(())
}
/**
 * Can be called from any thread
 */
pub fn logout() -> anyhow::Result<()> {
    UI.get().unwrap().upgrade_in_event_loop(|ui| {
        let app_state = ui.global::<crate::AppState>();
        app_state.set_loggedIn(false);
        app_state.set_loading(false);
        app_state.set_login_in_progress(false);
    })?;
    Ok(())
}
