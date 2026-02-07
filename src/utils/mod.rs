use chrono::Local;
use std::io::{self, IsTerminal, Read};

pub fn get_stdin() -> String {
    let mut input = String::new();

    if !io::stdin().is_terminal() {
        io::stdin().read_to_string(&mut input).unwrap();
    }
    return input;
}

pub fn current_datetime() -> String {
    Local::now().format("%Y-%m-%d %H:%M:%S").to_string()
}

pub fn capitalize(s: &str) -> String {
    s.get(0..1).unwrap_or("").to_uppercase() + s.get(1..).unwrap_or("")
}
