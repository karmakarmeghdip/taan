use xilem::{
    WidgetView,
    view::{flex_row, label},
};

use crate::state::App;

pub fn player(_state: &mut App) -> impl WidgetView<App> + use<> {
    flex_row(label("Music Player"))
}
