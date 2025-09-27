use slint::ComponentHandle;

pub fn set_loading(ui: &slint::Weak<crate::MainWindow>, loading: bool) {
    ui.upgrade_in_event_loop(move |ui| {
        let app = ui.global::<crate::AppState>();
        app.set_loading(loading);
    })
    .unwrap();
}

pub fn set_logged_in(ui: &slint::Weak<crate::MainWindow>, logged_in: bool) {
    ui.upgrade_in_event_loop(move |ui| {
        let app = ui.global::<crate::AppState>();
        app.set_loggedIn(logged_in);
        println!("Logged in slint event loop");
    })
    .unwrap();
}
