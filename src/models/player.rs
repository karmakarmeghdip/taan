use slint::ComponentHandle;

use crate::services::ui_weak;

pub fn set_track_details(
    track: Box<librespot_metadata::audio::item::AudioItem>,
) -> anyhow::Result<()> {
    ui_weak().upgrade_in_event_loop(move |ui| {
        let app = ui.global::<crate::PlayerState>();
        app.set_song_title(track.name.into());
        app.set_music_duration((track.duration_ms / 1000) as i32);
        match track.unique_fields {
            librespot_metadata::audio::UniqueFields::Track { artists, album, .. } => {
                use librespot_protocol::metadata::artist_with_role::ArtistRole;
                let mut composers = vec![];
                for artist in artists.0 {
                    if artist.role == ArtistRole::ARTIST_ROLE_MAIN_ARTIST {
                        app.set_artist_name(artist.name.into());
                    } else if artist.role == ArtistRole::ARTIST_ROLE_COMPOSER {
                        composers.push(artist.name);
                    }
                }
                let composer_str = composers.join(", ");
                app.set_composer(composer_str.into());
                app.set_album(album.into());
            }
            librespot_metadata::audio::UniqueFields::Episode { .. } => todo!(),
        }
    })?;
    Ok(())
}
pub fn set_cover_art(img: slint::SharedPixelBuffer<slint::Rgba8Pixel>) -> anyhow::Result<()> {
    ui_weak().upgrade_in_event_loop(move |ui| {
        let app = ui.global::<crate::PlayerState>();
        app.set_album_art(slint::Image::from_rgba8(img));
    })?;
    Ok(())
}
pub fn pause() -> anyhow::Result<()> {
    ui_weak().upgrade_in_event_loop(move |ui| {
        let app = ui.global::<crate::PlayerState>();
        app.set_is_playing(false);
    })?;
    Ok(())
}
pub fn play() -> anyhow::Result<()> {
    ui_weak().upgrade_in_event_loop(move |ui| {
        let app = ui.global::<crate::PlayerState>();
        app.set_is_playing(true);
    })?;
    Ok(())
}
pub fn set_position(position_ms: u32) -> anyhow::Result<()> {
    ui_weak().upgrade_in_event_loop(move |ui| {
        let app = ui.global::<crate::PlayerState>();
        app.set_current_time((position_ms / 1000) as i32);
    })?;
    Ok(())
}
