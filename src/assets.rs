use ratatui::text::Text;

// Logo
pub static LOGO: &str = include_str!("../assets/logo.txt");
pub static LOGO_TEXT: std::sync::LazyLock<Text<'static>> =
    std::sync::LazyLock::new(|| Text::raw(LOGO));
