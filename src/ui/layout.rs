use rspotify::model::PlayableItem;
use xilem::{
    WidgetView,
    core::one_of::OneOf2,
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

pub fn playlist_picker(state: &mut App) -> impl WidgetView<App> + use<> {
    if let Some(playlists) = state.playlists.as_ref() {
        let playlists = playlists
            .into_iter()
            .map(|p| {
                let pid = p.id.clone();
                button(p.name.clone(), move |s: &mut App| {
                    s.tx.as_ref()
                        .expect("Sender unavailable, fatal")
                        .send(crate::spotify::async_loop::Command::GetPlaylistTracks(
                            pid.clone_static(),
                        ))
                        .expect("Failed to send command, fatal");
                })
            })
            .collect::<Vec<_>>();
        OneOf2::A(flex_col(playlists))
    } else {
        OneOf2::B(flex_col((
            label("No Playlists available"),
            button("Fetch playlists", |s: &mut App| {
                s.tx.clone().inspect(|t| {
                    t.send(crate::spotify::async_loop::Command::GetUserPlaylists)
                        .unwrap();
                });
            }),
        )))
    }
}
pub fn playlist_view(state: &mut App) -> impl WidgetView<App> + use<> {
    let songs = if let Some(list) = state.playlist_item.as_ref() {
        list.iter()
            .map(|i| {
                if let Some(track) = &i.track {
                    if let PlayableItem::Track(t) = track {
                        let name = t.name.clone();
                        let id = t.id.clone();
                        OneOf2::A(button(format!("{:?}", name), move |s: &mut App| {
                            let id = id.as_ref().expect("Missing track id");
                            println!("Playing {:#?}", id.to_string());
                            s.tx.as_ref()
                                .unwrap()
                                .send(crate::spotify::async_loop::Command::PlayTrack(
                                    id.to_string(),
                                ))
                                .unwrap();
                        }))
                    } else {
                        OneOf2::B(label("Unknown playlist item"))
                    }
                } else {
                    OneOf2::B(label("Unknown playlist item"))
                }
            })
            .collect()
    } else {
        Vec::new()
    };
    flex_col(songs)
}
