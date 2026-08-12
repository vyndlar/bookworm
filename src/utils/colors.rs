use ratatui::style::{Color, Style};

use crate::utils::app::{App, Screen};

pub fn foreground_color(caller: Screen, app: &App) -> Color {
    if caller == app.screen {
        Color::Red
    } else {
        Color::Blue
    }
}

// style for selected option (book, series) to highlight
// To improve readability
pub fn selection_style(caller: usize, app: &App) -> Style {
    if caller == app.selected_book {
        Style::new().cyan().on_red()
    } else {
        Style::new().blue().on_black()
    }
}
