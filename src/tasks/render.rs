use std::io::IsTerminal;
use termimad::MadSkin;

/// Renders markdown to terminal-friendly output.
pub fn render_markdown(response: &str) -> String {
    if !std::io::stdout().is_terminal() {
        return response.to_string();
    }
    let skin = MadSkin::default();
    skin.term_text(response).to_string()
}
