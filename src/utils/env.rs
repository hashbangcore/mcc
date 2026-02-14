use std::env;

use super::strings::capitalize;

/// Returns the current OS user name, with a capitalized first letter.
pub fn get_user() -> String {
    let user = env::var("USER").unwrap_or_else(|_| "user".to_string());
    capitalize(&user)
}

/// Returns the preferred user language from common locale variables.
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
