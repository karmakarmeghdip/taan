use xilem::{
    WidgetView,
    core::one_of::OneOf3,
    view::{button, label},
};

use crate::state::App;

pub fn login_button(data: &mut App) -> impl WidgetView<App> + use<> {
    if !data.authenticating {
        if !data.logged_in {
            OneOf3::A(button("Login with Spotify", |s: &mut App| {
                let res =
                    s.tx.as_ref()
                        .map(|t| t.send(super::Command::AttemptOAuth).ok());
                s.authenticating = res.is_some();
            }))
        } else {
            OneOf3::B(label(format!(
                "Hello, {}!!",
                data.user
                    .as_ref()
                    .map(|u| u.username.clone())
                    .unwrap_or("User".to_string())
            )))
        }
    } else {
        OneOf3::C(label("Logging in, complete the auth flow"))
    }
}
