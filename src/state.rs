use rspotify::model::{PlaylistId, PlaylistItem, SimplifiedPlaylist};
use xilem::tokio::sync::mpsc::UnboundedSender;

#[derive(Default)]
pub struct App {
    pub authenticating: bool,
    pub error: Option<String>,
    pub user: Option<rspotify::model::PrivateUser>,
    pub playlists: Option<Vec<SimplifiedPlaylist>>,
    pub open_playlist: Option<PlaylistId<'static>>,
    pub playlist_item: Option<Vec<PlaylistItem>>,
    pub tx: Option<UnboundedSender<crate::spotify::async_loop::Command>>,
}
