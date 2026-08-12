use ratatui::style::Color;

use crate::utils::app::{self, App, Screen};

pub fn foreground_color(caller: Screen, app: &App) -> Color {
    if caller == app.screen {
        Color::Red
    } else {
        Color::Blue
    }
}
