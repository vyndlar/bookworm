mod utils;

use color_eyre::Result;
use crossterm::event::{self, Event};
use ratatui::{
    DefaultTerminal, Frame,
    layout::{Constraint, Layout},
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
    let lo = Layout::default()
        .direction(ratatui::layout::Direction::Horizontal)
        .margin(1)
        .constraints(vec![
            Constraint::Percentage(20),
            Constraint::Percentage(60),
            Constraint::Percentage(20),
        ])
        .split(frame.area());

    frame.render_widget(
        Paragraph::new("Series").block(Block::new().bold().fg(Color::Blue).borders(Borders::ALL)),
        lo[0],
    );
    frame.render_widget(
        Paragraph::new("Second").block(Block::new().bold().fg(Color::Green).borders(Borders::ALL)),
        lo[1],
    );
    frame.render_widget(
        Paragraph::new("Third").block(Block::new().bold().fg(Color::Green).borders(Borders::ALL)),
        lo[2],
    );
}
