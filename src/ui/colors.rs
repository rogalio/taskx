use ratatui::style::Color;

// Linear-like colors
pub const BLUE: Color = Color::Rgb(59, 130, 246);
pub const GREEN: Color = Color::Rgb(16, 185, 129);
pub const YELLOW: Color = Color::Rgb(251, 191, 36);
pub const RED: Color = Color::Rgb(239, 68, 68);
pub const PURPLE: Color = Color::Rgb(139, 92, 246);
pub const GRAY: Color = Color::Rgb(156, 163, 175);
pub const DARK_GRAY: Color = Color::Rgb(75, 85, 99);

// Status colors
pub const TODO_COLOR: Color = BLUE;
pub const IN_PROGRESS_COLOR: Color = GREEN;
pub const DONE_COLOR: Color = GRAY;

// Priority colors
pub const LOW_PRIORITY_COLOR: Color = BLUE;
pub const MEDIUM_PRIORITY_COLOR: Color = GREEN;
pub const HIGH_PRIORITY_COLOR: Color = YELLOW;
pub const CRITICAL_PRIORITY_COLOR: Color = RED;