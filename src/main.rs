mod utils;

use std::iter::once;

use color_eyre::Result;
use crossterm::event::{self, KeyCode};
use ratatui::{
    DefaultTerminal, Frame,
    layout::{Constraint, Direction::Vertical, Layout, Rect},
    style::{Color, Style, Stylize},
    widgets::{Block, Borders, List, Paragraph, Row, Table, TableState},
};

use crate::utils::{app::App, library};

mod content;

fn main() -> Result<()> {
    // TODO: Show error message in UI
    let lib = library::load()?;
    let mut app = App::new(lib);

    color_eyre::install()?;

    let mut table_state = TableState::default();
    table_state.select_first();
    table_state.select_first_column();

    let terminal = ratatui::init();
    let result = run(terminal, &mut app, &mut table_state);
    ratatui::restore();

    result
}

fn run(mut term: DefaultTerminal, app: &mut App, table_state: &mut TableState) -> Result<()> {
    loop {
        term.draw(|f| render(f, app, table_state))?; // render frame while passing in app state

        // KEYBINDS //
        if let Some(key) = event::read()?.as_key_press_event() {
            match key.code {
                KeyCode::Char('q') | KeyCode::Esc => return Ok(()),
                KeyCode::Char('j') | KeyCode::Down => table_state.select_next(),
                KeyCode::Char('k') | KeyCode::Up => table_state.select_previous(),
                KeyCode::Char('l') | KeyCode::Right => table_state.select_next_column(),
                KeyCode::Char('h') | KeyCode::Left => table_state.select_previous_column(),
                KeyCode::Char('g') => table_state.select_first(),
                KeyCode::Char('G') => table_state.select_last(),
                _ => {}
            }
        }
    }
}

fn render(frame: &mut Frame, app: &mut App, table_state: &mut TableState) {
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
    render_book_table(frame, app, main[1], table_state);
    frame.render_widget(
        // this will have two modes: view and edit
        Paragraph::new("Options (change title, author, genre, etc)").block(
            Block::new()
                .title("Properties")
                .bold()
                .fg(Color::Blue)
                .borders(Borders::ALL),
        ),
        main[2],
    );
}

fn render_book_table(frame: &mut Frame, app: &mut App, area: Rect, table_state: &mut TableState) {
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

    let widths = [
        Constraint::Fill(2),
        Constraint::Fill(1),
        Constraint::Fill(1),
    ];

    let table = Table::new(rows, widths)
        .header(header)
        .column_spacing(1)
        .style(Color::White)
        .highlight_symbol("> ")
        .row_highlight_style(Style::new().on_blue().black())
        .block(
            Block::new()
                .title("Books")
                .bold()
                .fg(Color::Blue)
                .borders(Borders::ALL),
        );
    frame.render_stateful_widget(table, area, table_state);
}
