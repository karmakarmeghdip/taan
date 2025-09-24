use i_slint_backend_winit::WinitWindowAccessor;
use slint::ComponentHandle;

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
