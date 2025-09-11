#[derive(Default)]
pub struct App {
    creds: Option<librespot_core::authentication::Credentials>,
    load_creds: bool,
}
