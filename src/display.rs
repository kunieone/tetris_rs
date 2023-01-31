use colored::*;

pub fn colored_string(text: String, color: Color) -> String {
    format!("{}", text.color(color))
}
