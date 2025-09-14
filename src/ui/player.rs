use xilem::{
    WidgetView,
    view::{button, flex_row, label},
};

use crate::state::App;

pub fn player(state: &mut App) -> impl WidgetView<App> + use<> {
    let now_playing = state
        .current_name
        .clone()
        .unwrap_or("Nothing playing right now".to_owned());
    flex_row((
        label(now_playing),
        button("Pause", |s: &mut App| {
            s.tx.as_ref()
                .unwrap()
                .send(crate::spotify::async_loop::Command::Pause)
                .unwrap();
        }),
    ))
}
