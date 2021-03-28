pub mod lib;
use crossterm::{
    event::{self, Event as CEvent, KeyCode},
    terminal::{disable_raw_mode, enable_raw_mode},
};
use lib::{events::Event, menu::MenuItem};
use lib_sters::{get_posts, models::Post, LobstersPath};
use std::{
    io,
    sync::mpsc,
    thread,
    time::{Duration, Instant},
};
use tui::{backend::CrosstermBackend, layout::Rect};
use tui::{
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Style},
    widgets::{BorderType, Paragraph},
};
use tui::{
    style::Modifier,
    text::{Span, Spans},
    widgets::{Block, Borders, Tabs},
};
use tui::{
    widgets::{Cell, Row, Table},
    Terminal,
};

fn main() -> Result<(), io::Error> {
    let (tx, rx) = mpsc::channel();
    let tick_rate = Duration::from_millis(200);
    thread::spawn(move || {
        let mut last_tick = Instant::now();
        loop {
            let timeout = tick_rate
                .checked_sub(last_tick.elapsed())
                .unwrap_or_else(|| Duration::from_secs(0));

            if event::poll(timeout).expect("poll works") {
                if let CEvent::Key(key) = event::read().expect("can read key events") {
                    tx.send(Event::Input(key)).expect("can send key events");
                }
            }

            if last_tick.elapsed() >= tick_rate {
                if let Ok(_) = tx.send(Event::Tick) {
                    last_tick = Instant::now();
                }
            }
        }
    });

    enable_raw_mode().expect("can run raw mode");
    let stdout = io::stdout();
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend).unwrap();
    terminal.clear().unwrap();

    let menu_titles: Vec<&str> = vec!["Hottest", "Newest", "Saved", "Preferance", "Quit"];
    let mut active_menu_item: MenuItem = MenuItem::Hottest;

    let hottest_posts: Vec<Post> = get_posts(LobstersPath::Hottest, None).unwrap();
    let newest_posts: Vec<Post> = get_posts(LobstersPath::Newest, None).unwrap();
    let hottest_table: Table = generate_table(MenuItem::Hottest, &hottest_posts);
    let newest_table: Table = generate_table(MenuItem::Newest, &newest_posts);
    let empty_table: Table = Table::new(vec![]);
    loop {
        terminal
            .draw(|rect| {
                let size = rect.size();

                let chunks: Vec<Rect> = Layout::default()
                    .direction(Direction::Vertical)
                    .margin(1)
                    .constraints(
                        [
                            Constraint::Length(3),
                            Constraint::Min(2),
                            Constraint::Length(3),
                        ]
                        .as_ref(),
                    )
                    .split(size);

                // let boundary: Block = Block::default()
                //     .borders(Borders::ALL)
                //     .border_style(Style::default().fg(Color::White))
                //     .border_type(BorderType::Thick);

                let menu: Vec<Spans> = menu_titles
                    .iter()
                    .map(|t| {
                        let (first, rest) = t.split_at(1);
                        Spans::from(vec![
                            Span::styled(
                                first,
                                Style::default()
                                    .fg(Color::Green)
                                    .add_modifier(Modifier::UNDERLINED),
                            ),
                            Span::styled(rest, Style::default().fg(Color::White)),
                        ])
                    })
                    .collect();

                let tabs = Tabs::new(menu)
                    .select(active_menu_item.into())
                    .block(Block::default().title("Menu").borders(Borders::ALL))
                    .style(Style::default().fg(Color::White))
                    .highlight_style(Style::default().fg(Color::Green))
                    .divider(Span::raw("|"));
                match active_menu_item {
                    MenuItem::Hottest => {
                        rect.render_widget(hottest_table.clone(), chunks[1]);
                    }
                    MenuItem::Newest => {
                        rect.render_widget(newest_table.clone(), chunks[1]);
                    }
                    MenuItem::Saved => {
                        rect.render_widget(empty_table.clone(), chunks[1]);
                    }
                    MenuItem::Preference => {
                        rect.render_widget(empty_table.clone(), chunks[1]);
                    }
                    MenuItem::Quit => {
                        rect.render_widget(empty_table.clone(), chunks[1]);
                    }
                }

                let footer: Paragraph = Paragraph::new("Footer goes brrr")
                    .alignment(Alignment::Center)
                    .style(Style::default().fg(Color::Cyan))
                    .block(
                        Block::default()
                            .borders(Borders::ALL)
                            .style(Style::default().fg(Color::White))
                            .border_type(BorderType::Plain),
                    );

                // rect.render_widget(boundary, size);
                rect.render_widget(tabs, chunks[0]);
                rect.render_widget(footer, chunks[2]);
            })
            .unwrap();
        match rx.recv().unwrap() {
            Event::Input(event) => match event.code {
                KeyCode::Char('h') => active_menu_item = MenuItem::Hottest,
                KeyCode::Char('n') => active_menu_item = MenuItem::Newest,
                KeyCode::Char('s') => active_menu_item = MenuItem::Saved,
                KeyCode::Char('p') => active_menu_item = MenuItem::Preference,
                KeyCode::Char('q') => {
                    disable_raw_mode().unwrap();
                    terminal.show_cursor().unwrap();
                    terminal.clear().unwrap();
                    break;
                }
                _ => {}
            },
            Event::Tick => {}
        }
    }
    Ok(())
}

fn generate_table(menu_item: MenuItem, posts: &Vec<Post>) -> Table {
    let mut post_rows: Vec<Row> = Vec::new();
    for (index, post) in posts.iter().enumerate() {
        let row: Row = Row::new(vec![
            Cell::from(Span::raw("❯")),
            Cell::from(Span::raw(index.to_string())),
            Cell::from(Span::raw(&post.title)),
        ]);
        post_rows.push(row)
    }
    let posts_table: Table = Table::new(post_rows)
        .header(Row::new(vec![
            Cell::from(Span::styled("", Style::default())),
            Cell::from(Span::styled(
                "No.",
                Style::default().add_modifier(Modifier::BOLD),
            )),
            Cell::from(Span::styled(
                "Posts",
                Style::default().add_modifier(Modifier::BOLD),
            )),
        ]))
        .block(
            Block::default()
                .title(match menu_item {
                    MenuItem::Hottest => "Hottest",
                    MenuItem::Newest => "Newest",
                    _ => "",
                })
                .borders(Borders::ALL)
                .style(Style::default().fg(Color::White))
                .border_type(BorderType::Plain),
        )
        .widths(&[
            Constraint::Length(1),
            Constraint::Length(3),
            Constraint::Min(5),
        ]);
    posts_table
}
