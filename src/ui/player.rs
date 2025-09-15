use xilem::{
    WidgetView,
    style::{Padding, Style},
    view::{button, flex_row, label},
};

use crate::{state::App, ui::colors::ThemeColor};

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
    .must_fill_major_axis(true)
    .main_axis_alignment(xilem::view::MainAxisAlignment::Center)
    .padding(Padding::all(8.0))
    .background_color(ThemeColor::from(state.theme_state.flavor.colors.surface1).0)
}
