use xilem::masonry::peniko::color::{AlphaColor, Srgb};

pub(crate) struct ThemeState {
    pub flavor: catppuccin::Flavor,
}

pub struct ThemeColor(pub AlphaColor<Srgb>);

impl std::ops::Deref for ThemeColor {
    type Target = AlphaColor<Srgb>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Default for ThemeState {
    fn default() -> Self {
        Self {
            flavor: catppuccin::PALETTE.mocha,
        }
    }
}

impl From<catppuccin::Color> for ThemeColor {
    fn from(value: catppuccin::Color) -> Self {
        let color = AlphaColor::from_rgb8(value.rgb.r, value.rgb.g, value.rgb.b);
        Self(color)
    }
}
