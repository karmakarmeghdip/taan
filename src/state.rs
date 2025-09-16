use rspotify::model::{PlaylistItem, SimplifiedPlaylist};

#[derive(Default)]
pub struct App {
    pub authenticating: bool,
    pub error: Option<String>,
    pub user: Option<rspotify::model::PrivateUser>,
    pub playlists: Option<Vec<SimplifiedPlaylist>>,
    pub playlist_item: Option<Vec<PlaylistItem>>,
    pub current_name: Option<String>,
}
