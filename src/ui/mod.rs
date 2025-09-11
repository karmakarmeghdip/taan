use xilem::{
    WidgetView,
    view::{Axis, flex},
};

use crate::state::App;

mod spotify_login;

pub fn root(data: &mut App) -> impl WidgetView<App> + use<> {
    flex(Axis::Vertical, (spotify_login::login_button(data),))
}
