use ratatui::style::{Color, Style};
use std::str::FromStr;

pub fn parse_color(hex: &str) -> Color {
    if let Some(hex) = hex.strip_prefix('#') {
        if hex.len() == 6 {
            if let (Ok(r), Ok(g), Ok(b)) = (
                u8::from_str_radix(&hex[0..2], 16),
                u8::from_str_radix(&hex[2..4], 16),
                u8::from_str_radix(&hex[4..6], 16),
            ) {
                return Color::Rgb(r, g, b);
            }
        }
    }

    // Fallback to foreground color if parsing fails
    match hex {
        "red" => Color::Red,
        "green" => Color::Green,
        "blue" => Color::Blue,
        "yellow" => Color::Yellow,
        "magenta" => Color::Magenta,
        "cyan" => Color::Cyan,
        "gray" => Color::Gray,
        "dark_gray" => Color::DarkGray,
        "white" => Color::White,
        "black" => Color::Black,
        _ => Color::Reset,
    }
}

pub fn styled_text(text: &str, color_hex: &str) -> (String, Style) {
    let color = parse_color(color_hex);
    (text.to_string(), Style::default().fg(color))
}

pub fn truncate_string(s: &str, max_len: usize) -> String {
    if s.len() <= max_len {
        s.to_string()
    } else {
        format!("{}…", &s[..max_len.saturating_sub(1)])
    }
}