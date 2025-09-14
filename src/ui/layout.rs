use xilem::{
    WidgetView,
    view::{button, flex_col, flex_row, label},
};

use crate::state::App;

pub fn header(state: &mut App) -> impl WidgetView<App> + use<> {
    flex_row((label("Spotify"), super::spotify_login::login_button(state)))
        .must_fill_major_axis(true)
        .main_axis_alignment(xilem::view::MainAxisAlignment::SpaceBetween)
}

pub fn main_area(state: &mut App) -> impl WidgetView<App> + use<> {
    flex_row((playlist_picker(state), playlist_view(state)))
        .must_fill_major_axis(true)
        .main_axis_alignment(xilem::view::MainAxisAlignment::SpaceAround)
}

pub fn footer(state: &mut App) -> impl WidgetView<App> + use<> {
    super::player::player(state)
}

pub fn playlist_picker(_state: &mut App) -> impl WidgetView<App> + use<> {
    label("Playlist Picker")
}
pub fn playlist_view(_state: &mut App) -> impl WidgetView<App> + use<> {
    flex_col((
        label("Playlist Songs"),
        button("Fetch 16 playlists", |s: &mut App| {
            s.tx.clone().inspect(|t| {
                t.send(crate::spotify::async_loop::Command::GetUserPlaylists)
                    .unwrap();
            });
        }),
    ))
}
