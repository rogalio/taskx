use ratatui::{
    text::Line,
    widgets::Tabs,
};

pub fn get_tab_titles() -> Tabs<'static> {
    Tabs::new(vec![
        Line::from(" Tasks "),
        Line::from(" Calendar "),
        Line::from(" Statistics "),
    ])
}