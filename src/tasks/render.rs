use termimad::MadSkin;

/// Renders markdown to terminal-friendly output.
pub fn render_markdown(response: &str) -> String {
    let skin = MadSkin::default();
    skin.term_text(response).to_string()
}
