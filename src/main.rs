mod utils;

use color_eyre::Result;
use crossterm::event::{self, Event};
use ratatui::{
    DefaultTerminal, Frame,
    layout::{Constraint, Layout},
    style::{Color, Stylize},
    widgets::{Block, Borders, Paragraph},
};

use crate::utils::library;

mod content;

fn main() -> Result<()> {
    color_eyre::install()?;
    let terminal = ratatui::init();
    let result = run(terminal);
    ratatui::restore();
    let lib = library::load();
    result
}

fn run(mut term: DefaultTerminal) -> Result<()> {
    loop {
        term.draw(render)?;
        if matches!(event::read()?, Event::Key(_)) {
            break Ok(());
        }
    }
}

fn render(frame: &mut Frame) {
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
        Paragraph::new("Buecher").block(Block::new().bold().fg(Color::Blue).borders(Borders::ALL)),
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
