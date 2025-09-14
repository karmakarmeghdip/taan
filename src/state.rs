use xilem::tokio::sync::mpsc::UnboundedSender;

#[derive(Default)]
pub struct App {
    pub authenticating: bool,
    pub error: Option<String>,
    pub user: Option<rspotify::model::PrivateUser>,
    pub tx: Option<UnboundedSender<crate::spotify::async_loop::Command>>,
}
