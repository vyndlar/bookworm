mod utils;

use std::iter::once;

use color_eyre::Result;
use crossterm::event::{self, Event};
use ratatui::{
    DefaultTerminal, Frame,
    layout::{Constraint, Direction::Vertical, Layout, Rect},
    style::{Color, Style, Stylize},
    symbols::block,
    widgets::{Block, Borders, List, Paragraph, Row, TableState},
};

use crate::{
    content::{book::Book, series::Series},
    utils::{app::App, library},
};

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
        .constraints(vec![Constraint::Percentage(75), Constraint::Percentage(25)])
        .split(main[0]);

    let series_list: Vec<&str> = app
        .library
        .series
        .iter()
        .map(|s| s.title.as_str())
        .chain(once("Standalone Books"))
        .collect();

    // renders the series names on the left panel
    frame.render_widget(
        List::new(series_list).cyan().block(
            Block::new()
                .bold()
                .fg(Color::Blue)
                .borders(Borders::ALL)
                .title("Series"),
        ),
        left_panel[0],
    );

    // TODO: Stats block on the left panel
    frame.render_widget(
        List::new(vec!["Total Books: -", "Foo: Bar"]) // give some sample data
            .cyan()
            .block(
                Block::new()
                    .bold()
                    .fg(Color::Blue)
                    .borders(Borders::ALL)
                    .title("Library Stats"),
            ),
        left_panel[1],
    );

    // -------------- //
    // main book view //
    // -------------- //
    frame.render_widget(
        Paragraph::new("Turn ts into a table").block(
            Block::new()
                .title("Books")
                .bold()
                .fg(Color::Blue)
                .borders(Borders::ALL),
        ),
        main[1],
    );
    frame.render_widget(
        Paragraph::new("Options (change title, author, genre, etc)").block(
            Block::new()
                .title("Options")
                .bold()
                .fg(Color::Blue)
                .borders(Borders::ALL),
        ),
        main[2],
    );
}

fn render_book_table(
    frame: &mut Frame,
    app: App,
    area: Rect,
    table_state: &mut TableState,
    series: &Series,
) {
    let header = Row::new(["Title", "Author", "Pages", "isOwned"])
        .style(Style::new().bold())
        .bottom_margin(1);

    let mut rows = vec![];
    match app.library.series.get(app.selected_series) {
        // returns a reference to the series
        Some(s) => {
            for book in &s.books {
                rows.push(Row::new([
                    book.title.clone(),
                    book.author.to_string(),
                    book.total_pages.to_string(),
                    book.is_owned.to_string(),
                ]));
            }
        }
        None => rows.push(Row::new(["err", "err", "err", "err"])),
    }
}
