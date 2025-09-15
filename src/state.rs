use rspotify::model::{PlaylistItem, SimplifiedPlaylist};
use xilem::tokio::sync::mpsc::UnboundedSender;

#[derive(Default)]
pub struct App {
    pub authenticating: bool,
    pub error: Option<String>,
    pub user: Option<rspotify::model::PrivateUser>,
    pub playlists: Option<Vec<SimplifiedPlaylist>>,
    pub playlist_item: Option<Vec<PlaylistItem>>,
    pub current_name: Option<String>,
    pub tx: Option<UnboundedSender<crate::spotify::async_loop::Command>>,
    pub theme_state: crate::ui::colors::ThemeState,
}
