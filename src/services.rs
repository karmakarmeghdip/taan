pub mod spotify;

struct Services {
    spotify: spotify::SpotifyService,
    rt: tokio::runtime::Handle,
    ui: slint::Weak<crate::MainWindow>,
}

static SERVICES: std::sync::OnceLock<Services> = std::sync::OnceLock::new();

pub fn init(
    spotify: spotify::SpotifyService,
    rt: tokio::runtime::Handle,
    ui: slint::Weak<crate::MainWindow>,
) {
    SERVICES
        .set(Services { spotify, rt, ui })
        .unwrap_or_else(|_| {
            log::error!("Init must be called only once");
        });
}

pub fn spotify() -> &'static spotify::SpotifyService {
    &SERVICES.get().unwrap().spotify
}
pub fn rt() -> &'static tokio::runtime::Handle {
    &SERVICES.get().unwrap().rt
}
pub fn ui_weak() -> &'static slint::Weak<crate::MainWindow> {
    &SERVICES.get().unwrap().ui
}
