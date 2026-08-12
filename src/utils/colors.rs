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
    // basically: if selected :)
    // series pane
    if (caller == app.library.series.len() && app.selected_series == app.library.series.len())
        || caller == app.selected_series
    {
        if app.screen == Screen::SeriesList {
            Style::new().black().on_cyan()
        } else {
            Style::new().black().on_blue()
        }
    } else {
        // if not selected
        Style::new().blue().on_black()
    }
}

pub fn table_style(app: &App) -> Style {
    if app.screen == Screen::BookList {
        // pane is active
        Style::new().black().on_cyan()
    } else {
        // pane is not active
        Style::new().black().on_blue()
    }
}
