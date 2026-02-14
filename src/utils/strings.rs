/// Capitalizes the first character of a string.
pub fn capitalize(s: &str) -> String {
    s.get(0..1).unwrap_or("").to_uppercase() + s.get(1..).unwrap_or("")
}
