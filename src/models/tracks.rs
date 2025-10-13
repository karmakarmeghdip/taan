use crate::services::ui_weak;
use slint::{ComponentHandle, Model, ToSharedString};

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

pub fn add_tracks(l: Vec<rspotify::model::PlaylistItem>) -> anyhow::Result<()> {
    ui_weak().upgrade_in_event_loop(move |ui| {
        let tracks = ui.global::<crate::TracksState>().get_tracks();
        let list = tracks
            .as_any()
            .downcast_ref::<slint::VecModel<crate::Track>>()
            .unwrap();
        for item in l {
            if let Some(t) = item.track {
                match t {
                    rspotify::model::PlayableItem::Track(track) => {
                        // Create SharedPixelBuffer and load the image from the URL
                        let t = crate::Track {
                            id: track.id.unwrap().to_shared_string(),
                            title: track.name.into(),
                            duration: track.duration.num_milliseconds() as i32,
                            album: track.album.name.into(),
                            artist: track
                                .artists
                                .iter()
                                .map(|a| a.name.clone())
                                .collect::<Vec<String>>()
                                .join(", ")
                                .into(),
                            cover_art: slint::Image::default(),
                        };
                        list.push(t);
                    }
                    rspotify::model::PlayableItem::Episode(full_episode) => {
                        log::info!("{:#?}", full_episode);
                    }
                    rspotify::model::PlayableItem::Unknown(value) => {
                        log::warn!("Unknown playable item: {:#?}", value);
                    }
                }
            }
        }
    })?;
    Ok(())
}
