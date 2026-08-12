mod utils;
use std::iter::once;

use color_eyre::Result;
use crossterm::event::{self};
use ratatui::{
    DefaultTerminal, Frame,
    layout::{Constraint, Direction::Vertical, Layout, Rect},
    style::{Color, Style, Stylize},
    widgets::{Block, Borders, List, ListItem, Paragraph, Row, Table, TableState},
};

use crate::utils::{
    app::{App, Screen},
    colors::{foreground_color, selection_style},
    library,
};

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
        // term.draw(|f| render(f))
        // term.draw(render)

        // KEYBINDS //
        if let Some(key) = event::read()?.as_key_press_event() {
            app.on_key(key, table_state);
        }

        if app.should_quit {
            return Ok(());
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

    let series_list: Vec<ListItem> = app
        .library
        .series
        .iter()
        .enumerate()
        .map(|(i, s)| ListItem::new(s.title.to_string()).style(selection_style(i, app)))
        .chain(once(
            ListItem::new("Standalone Books").style(selection_style(app.library.series.len(), app)),
        ))
        .collect();

    // renders the series names on the left panel
    frame.render_widget(
        List::new(series_list).style(Style::new().cyan()).block(
            Block::new()
                .bold()
                .fg(foreground_color(Screen::SeriesList, app))
                .borders(Borders::ALL)
                .title("Series"),
        ),
        left_panel[0],
    );

    // TODO: Stats block on the left panel

    // ------- //
    // STATIC //
    // ------- //
    frame.render_widget(
        List::new(vec!["Total Books: -", "Your most read genre is -"]) // give some sample data
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
        Paragraph::new("Who even knows").block(
            Block::new()
                .title("Properties")
                .bold()
                .fg(foreground_color(Screen::Properties, app))
                .borders(Borders::ALL),
        ),
        main[2],
    );
}

fn render_book_table(frame: &mut Frame, app: &mut App, area: Rect, table_state: &mut TableState) {
    let header = Row::new(["Title", "Author", "Pages", "Times Read"])
        .style(Style::new().bold())
        .bottom_margin(1);

    let mut rows = vec![];
    match app.library.series.get(app.selected_series) {
        // returns a reference to the series
        Some(s) => {
            for book in &s.books {
                rows.push(Row::new([
                    book.title.clone(),
                    book.author.clone(),
                    book.total_pages.to_string(),
                    book.times_read.to_string(),
                ]));
            }
        }
        None => {
            if app.selected_series == app.library.series.len() {
                // pull from the 'standalone_books'
                for book in &app.library.standalone_books {
                    rows.push(Row::new([
                        book.title.clone(),
                        book.author.clone(),
                        book.total_pages.to_string(),
                        book.times_read.to_string(),
                    ]));
                }
            } else {
                rows.push(Row::new(["err", "err", "err", "err"]))
            }
        }
    }

    let widths = [
        Constraint::Fill(2),
        Constraint::Fill(1),
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
                .fg(foreground_color(Screen::BookList, app))
                .borders(Borders::ALL),
        );
    frame.render_stateful_widget(table, area, table_state);
}
