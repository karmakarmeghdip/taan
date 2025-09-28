use i_slint_backend_winit::WinitWindowAccessor;
use slint::ComponentHandle;

use crate::services::ui_weak;

pub fn register_handlers() -> anyhow::Result<()> {
    let ui = ui_weak().unwrap();
    let app = ui.global::<crate::AppState>();

    app.on_close_window(move || {
        close().unwrap();
    });

    app.on_start_drag(move || {
        drag_window().unwrap();
    });
    Ok(())
}
pub fn close() -> anyhow::Result<()> {
    ui_weak().unwrap().hide()?;
    Ok(())
}
pub fn drag_window() -> anyhow::Result<()> {
    ui_weak().unwrap().window().with_winit_window(|win| {
        win.drag_window()
            .unwrap_or_else(|e| eprintln!("Failed to drag window: {}", e));
    });
    Ok(())
}
