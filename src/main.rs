mod utils;

use color_eyre::Result;
use crossterm::event::{self, Event};
use ratatui::{
    DefaultTerminal, Frame,
    layout::{Constraint, Direction::Vertical, Layout},
    style::{Color, Stylize},
    widgets::{Block, Borders, Paragraph},
};

use crate::utils::{app::App, library};

mod content;

fn main() -> Result<()> {
    // TODO: Show error message in UI
    let lib = library::load()?;
    let app = App::new(lib);

    color_eyre::install()?;

    let terminal = ratatui::init();
    let result = run(terminal, &app);
    ratatui::restore();

    result
}

fn run(mut term: DefaultTerminal, app: &App) -> Result<()> {
    loop {
        term.draw(|f| render(f, app))?; // render frame while passing in app state
        if matches!(event::read()?, Event::Key(_)) {
            break Ok(());
        }
    }
}

fn render(frame: &mut Frame, app: &App) {
    let main = Layout::default()
        .direction(ratatui::layout::Direction::Horizontal)
        .margin(1)
        .constraints(vec![
            Constraint::Percentage(20),
            Constraint::Percentage(60),
            Constraint::Percentage(20),
        ])
        .split(frame.area());

    let left_panel = Layout::default()
        .direction(Vertical)
        .constraints(vec![Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(main[0]);

    let series_list: Vec<&str> = app
        .library
        .series
        .iter()
        .map(|s| s.title.as_str())
        .collect();

    frame.render_widget(
        List::new().block::new()
            .bold()
            .fg(Color::Blue)
            .borders(Borders::ALL)
            .title("Series"),
        left_panel[0],
    );

    frame.render_widget(
        Block::new()
            .bold()
            .fg(Color::Blue)
            .borders(Borders::ALL)
            .title("Standalone Books"),
        left_panel[1],
    );

    frame.render_widget(
        Paragraph::new("Second").block(Block::new().bold().fg(Color::Green).borders(Borders::ALL)),
        main[1],
    );
    frame.render_widget(
        Paragraph::new("Third").block(Block::new().bold().fg(Color::Green).borders(Borders::ALL)),
        main[2],
    );
}
