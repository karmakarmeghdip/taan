use crate::services::ui_weak;
use slint::ComponentHandle;

/**
 * Can be called from any thread
 */
pub fn login_succeeded() -> anyhow::Result<()> {
    ui_weak().upgrade_in_event_loop(|ui| {
        let auth_state = ui.global::<crate::AuthenticationState>();
        auth_state.set_loggedIn(true);
        auth_state.set_loading(false);
        auth_state.set_login_in_progress(false);
    })?;
    Ok(())
}
/**
 * Can be called from any thread
 */
pub fn login_failed(_error: &str) -> anyhow::Result<()> {
    ui_weak().upgrade_in_event_loop(|ui| {
        let auth_state = ui.global::<crate::AuthenticationState>();
        auth_state.set_loggedIn(false);
        auth_state.set_loading(false);
        auth_state.set_login_in_progress(false);
        // TODO: Add error handling in UI
    })?;
    Ok(())
}
/**
 * Can be called from any thread
 */
pub fn login_started() -> anyhow::Result<()> {
    ui_weak().upgrade_in_event_loop(|ui| {
        let auth_state = ui.global::<crate::AuthenticationState>();
        auth_state.set_login_in_progress(true);
    })?;
    Ok(())
}
