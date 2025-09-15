use xilem::{
    WidgetView,
    core::fork,
    style::Style as _,
    view::{flex_col, worker},
};

use crate::{state::App, ui::colors::ThemeColor};

pub(crate) mod colors;
mod layout;
mod player;
mod spotify_login;

pub fn root(data: &mut App) -> impl WidgetView<App> + use<> {
    fork(
        flex_col((
            layout::header(data),
            layout::main_area(data),
            layout::footer(data),
        ))
        .must_fill_major_axis(true)
        .main_axis_alignment(xilem::view::MainAxisAlignment::SpaceBetween)
        .background_color(ThemeColor::from(data.theme_state.flavor.colors.base).0),
        worker(
            crate::spotify::async_loop::run_spotify_loop,
            |state: &mut App, tx| {
                state.tx = Some(tx);
            },
            crate::spotify::async_loop::handle_event,
        ),
    )
}
