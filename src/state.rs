use xilem::tokio::sync::mpsc::UnboundedSender;

#[derive(Default)]
pub struct App {
    pub authenticating: bool,
    pub error: Option<String>,
    pub user: Option<UserData>,
    pub logged_in: bool,
    pub tx: Option<UnboundedSender<crate::ui::Command>>,
}

#[derive(Debug)]
pub struct UserData {
    pub username: String,
}
