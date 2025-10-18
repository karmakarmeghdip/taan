use crate::services::ui_weak;
use rspotify::model::{FullTrack, PlayableItem, PlaylistItem};
use rspotify::prelude::*;
use slint::{ComponentHandle, Model};

pub fn set_current_track(uri: String) -> anyhow::Result<()> {
    ui_weak().upgrade_in_event_loop(move |ui| {
        let tracks = ui.global::<crate::TracksState>();
        tracks.set_current_track_id(uri.into());
    })?;
    Ok(())
}

pub fn set_fetching_tracks(x: bool) -> anyhow::Result<()> {
    ui_weak().upgrade_in_event_loop(move |ui| {
        let tracks = ui.global::<crate::TracksState>();
        tracks.set_fetching_tracks(x);
    })?;
    Ok(())
}

pub fn add_tracks(l: Vec<FullTrack>) -> anyhow::Result<()> {
    ui_weak().upgrade_in_event_loop(move |ui| {
        let tracks = ui.global::<crate::TracksState>().get_tracks();
        let list = tracks
            .as_any()
            .downcast_ref::<slint::VecModel<crate::Track>>()
            .unwrap();
        for track in l {
            // Create SharedPixelBuffer and load the image from the URL
            let t = crate::Track {
                id: track.id.unwrap().id().into(),
                title: track.name.into(),
                duration: track.duration.num_milliseconds() as i32,
                album: track.album.name.into(),
                artist: track
                    .artists
                    .into_iter()
                    .map(|a| a.name)
                    .collect::<Vec<String>>()
                    .join(", ")
                    .into(),
                cover_art: slint::Image::default(),
            };
            list.push(t);
        }
    })?;
    Ok(())
}
