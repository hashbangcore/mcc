use chrono::Local;
use std::env;
use std::io::{self, IsTerminal, Read};
use termimad::MadSkin;

pub fn get_stdin() -> String {
    let mut input = String::new();

    if !io::stdin().is_terminal() {
        io::stdin().read_to_string(&mut input).unwrap();
    }
    return input;
}

pub fn stdin_is_piped() -> bool {
    !io::stdin().is_terminal()
}

pub fn get_user() -> String {
    let user = env::var("USER").unwrap_or_else(|_| "user".to_string());
    capitalize(&user)
}

pub fn get_user_lang() -> String {
    for key in ["LC_ALL", "LC_MESSAGES", "LANG"] {
        if let Ok(value) = env::var(key) {
            let trimmed = value.trim();
            if !trimmed.is_empty() {
                return trimmed.to_string();
            }
        }
    }
    "unknown".to_string()
}

pub fn normalize_lang_tag(input: &str) -> String {
    let trimmed = input.trim();
    if trimmed.is_empty() {
        return "unknown".to_string();
    }

    let base = trimmed
        .split('.')
        .next()
        .unwrap_or(trimmed)
        .split('@')
        .next()
        .unwrap_or(trimmed)
        .replace('_', "-");

    let mut parts = base.splitn(2, '-');
    let lang = parts.next().unwrap_or("").to_ascii_lowercase();
    if lang.is_empty() {
        return "unknown".to_string();
    }

    let _region = parts.next().unwrap_or("");
    lang
}

pub fn lang_display_name(tag: &str) -> String {
    match tag.to_ascii_lowercase().as_str() {
        "en" => "English".to_string(),
        "zh" => "Chinese".to_string(),
        "hi" => "Hindi".to_string(),
        "es" => "Spanish".to_string(),
        "fr" => "French".to_string(),
        "ar" => "Arabic".to_string(),
        "bn" => "Bengali".to_string(),
        "pt" => "Portuguese".to_string(),
        "ru" => "Russian".to_string(),
        "ur" => "Urdu".to_string(),
        "id" => "Indonesian".to_string(),
        "de" => "German".to_string(),
        "ja" => "Japanese".to_string(),
        "sw" => "Swahili".to_string(),
        "mr" => "Marathi".to_string(),
        "te" => "Telugu".to_string(),
        "tr" => "Turkish".to_string(),
        "ta" => "Tamil".to_string(),
        "vi" => "Vietnamese".to_string(),
        "it" => "Italian".to_string(),
        "eo" => "Esperanto".to_string(),
        "io" => "Ido".to_string(),
        "ia" => "Interlingua".to_string(),
        "ie" => "Interlingue".to_string(),
        "vo" => "Volapuk".to_string(),
        "jbo" => "Lojban".to_string(),
        "tlh" => "Klingon".to_string(),
        "tok" => "Toki Pona".to_string(),
        "lfn" => "Lingua Franca Nova".to_string(),
        "nov" => "Novial".to_string(),
        _ => tag.to_string(),
    }
}

pub fn current_datetime() -> String {
    Local::now().format("%Y-%m-%d %H:%M:%S").to_string()
}

pub fn capitalize(s: &str) -> String {
    s.get(0..1).unwrap_or("").to_uppercase() + s.get(1..).unwrap_or("")
}

pub fn render_markdown(response: &str) -> String {
    let skin = MadSkin::default();
    skin.term_text(response).to_string()
}
