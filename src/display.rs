use colored::{Color, Colorize};

pub fn colored_string(text: String, color: Color) -> String {
    // termion::color::AnsiValue
    format!("{}", text.color(color))
}
