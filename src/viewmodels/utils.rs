use slint::ComponentHandle;

use crate::services::ui_weak;

pub fn register_handlers() -> anyhow::Result<()> {
    ui_weak().upgrade_in_event_loop(|ui| {
        let utils = ui.global::<crate::Utils>();
        utils.on_first_char(|s| {
            let mut i = slint::SharedString::new();
            i.push_str(&s.chars().next().unwrap_or_default().to_string());
            i
        });
        utils.on_ms_to_string(|ms| {
            let seconds = (ms / 1000) % 60;
            let minutes = (ms / 1000) / 60;
            slint::SharedString::from(format!("{:02}:{:02}", minutes, seconds))
        });
    })?;
    Ok(())
}
