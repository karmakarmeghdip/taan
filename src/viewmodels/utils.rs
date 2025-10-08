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
    })?;
    Ok(())
}
