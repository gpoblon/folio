/// Application theme mode.
#[derive(Default, Clone, Copy, PartialEq, Eq, Debug)]
pub enum ThemeMode {
    #[default]
    Dark,
    Light,
}
impl ThemeMode {
    pub const fn as_str(&self) -> &'static str {
        match self {
            ThemeMode::Light => "light",
            ThemeMode::Dark => "dark",
        }
    }
    pub const fn toggle(self) -> Self {
        match self {
            ThemeMode::Light => ThemeMode::Dark,
            ThemeMode::Dark => ThemeMode::Light,
        }
    }
}
