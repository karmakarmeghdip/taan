#[derive(Default)]
pub struct App {
    pub authenticating: bool,
    pub spotify: crate::spotify::SpotifyState,
    pub user: Option<UserData>,
}

pub struct UserData {
    pub username: String,
}
