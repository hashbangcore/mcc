//! Color theme helpers (not wired into output yet).
use std::env;

/// Terminal theme preference.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Theme {
    Light,
    Dark,
}

/// Named color palette (20 colors per theme).
#[derive(Clone, Copy, Debug)]
pub struct Palette {
    pub name: &'static str,
    pub colors: [(&'static str, &'static str); 20],
}

/// Output roles mapped to palette colors.
#[derive(Clone, Copy, Debug)]
pub struct Roles {
    pub header: &'static str,
    pub log: &'static str,
    pub debug: &'static str,
    pub response: &'static str,
    pub alert: &'static str,
    pub trace: &'static str,
    pub muted: &'static str,
}

/// Theme configuration selected from environment variables.
#[derive(Clone, Copy, Debug)]
pub struct ThemeConfig {
    pub theme: Theme,
    pub enabled: bool,
    pub palette: Palette,
    pub roles: Roles,
}

/// Returns the theme configuration based on environment variables.
/// NETERO_THEME overrides all other hints.
pub fn resolve_theme() -> ThemeConfig {
    if is_no_color() {
        return ThemeConfig {
            theme: Theme::Dark,
            enabled: false,
            palette: palette_dark(),
            roles: roles_dark(),
        };
    }

    let theme = match env::var("NETERO_THEME").ok().as_deref() {
        Some("light") => Theme::Light,
        Some("dark") => Theme::Dark,
        Some("auto") => detect_theme(),
        Some(_) => detect_theme(),
        None => detect_theme(),
    };

    match theme {
        Theme::Light => ThemeConfig {
            theme,
            enabled: true,
            palette: palette_light(),
            roles: roles_light(),
        },
        Theme::Dark => ThemeConfig {
            theme,
            enabled: true,
            palette: palette_dark(),
            roles: roles_dark(),
        },
    }
}

fn is_no_color() -> bool {
    env::var("NO_COLOR").is_ok()
}

/// Best-effort theme detection from common terminal env vars.
fn detect_theme() -> Theme {
    if let Ok(value) = env::var("COLORFGBG") {
        if let Some(bg) = value.split(';').last() {
            if let Ok(code) = bg.parse::<i32>() {
                if code >= 7 {
                    return Theme::Light;
                }
                return Theme::Dark;
            }
        }
    }

    if let Ok(value) = env::var("TERM") {
        let value = value.to_lowercase();
        if value.contains("light") {
            return Theme::Light;
        }
    }

    Theme::Dark
}

fn palette_light() -> Palette {
    Palette {
        name: "light",
        colors: [
            ("black", "\x1b[30m"),
            ("red", "\x1b[31m"),
            ("green", "\x1b[32m"),
            ("yellow", "\x1b[33m"),
            ("blue", "\x1b[34m"),
            ("magenta", "\x1b[35m"),
            ("cyan", "\x1b[36m"),
            ("white", "\x1b[37m"),
            ("gray", "\x1b[90m"),
            ("bright_red", "\x1b[91m"),
            ("bright_green", "\x1b[92m"),
            ("bright_yellow", "\x1b[93m"),
            ("bright_blue", "\x1b[94m"),
            ("bright_magenta", "\x1b[95m"),
            ("bright_cyan", "\x1b[96m"),
            ("bright_white", "\x1b[97m"),
            ("bold", "\x1b[1m"),
            ("dim", "\x1b[2m"),
            ("underline", "\x1b[4m"),
            ("reset", "\x1b[0m"),
        ],
    }
}

fn palette_dark() -> Palette {
    Palette {
        name: "dark",
        colors: [
            ("black", "\x1b[30m"),
            ("red", "\x1b[31m"),
            ("green", "\x1b[32m"),
            ("yellow", "\x1b[33m"),
            ("blue", "\x1b[34m"),
            ("magenta", "\x1b[35m"),
            ("cyan", "\x1b[36m"),
            ("white", "\x1b[37m"),
            ("gray", "\x1b[90m"),
            ("bright_red", "\x1b[91m"),
            ("bright_green", "\x1b[92m"),
            ("bright_yellow", "\x1b[93m"),
            ("bright_blue", "\x1b[94m"),
            ("bright_magenta", "\x1b[95m"),
            ("bright_cyan", "\x1b[96m"),
            ("bright_white", "\x1b[97m"),
            ("bold", "\x1b[1m"),
            ("dim", "\x1b[2m"),
            ("underline", "\x1b[4m"),
            ("reset", "\x1b[0m"),
        ],
    }
}

fn roles_light() -> Roles {
    Roles {
        header: "blue",
        log: "gray",
        debug: "magenta",
        response: "green",
        alert: "red",
        trace: "cyan",
        muted: "dim",
    }
}

fn roles_dark() -> Roles {
    Roles {
        header: "bright_blue",
        log: "gray",
        debug: "bright_magenta",
        response: "bright_green",
        alert: "bright_red",
        trace: "bright_cyan",
        muted: "dim",
    }
}
