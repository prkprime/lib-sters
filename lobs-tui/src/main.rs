pub mod lib;
use crossterm::{
    event::{self, Event as CEvent},
    terminal::enable_raw_mode,
};
use lib::{events::Event, menu::MenuItem};
use std::{
    io,
    sync::mpsc,
    thread,
    time::{Duration, Instant},
};
use tui::Terminal;
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
    terminal.clear();

    let menu_titles: Vec<&str> = vec!["Hottest", "Newest", "Saved", "Preferance", "Quit"];
    let mut active_menu_item: MenuItem = MenuItem::Hottest;

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

                let boundary: Block = Block::default()
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(Color::White))
                    .border_type(BorderType::Thick);

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
                    .block(Block::default().title("Menu").borders(Borders::BOTTOM))
                    .style(Style::default().fg(Color::White))
                    .highlight_style(Style::default().fg(Color::Green))
                    .divider(Span::raw("|"));
                match active_menu_item {
                    MenuItem::Hottest => {}
                    MenuItem::Newest => {}
                    MenuItem::Saved => {}
                    MenuItem::Preference => {}
                    MenuItem::Quit => {}
                }

                let footer: Paragraph = Paragraph::new("Footer goes brrr")
                    .alignment(Alignment::Center)
                    .style(Style::default().fg(Color::Cyan))
                    .block(
                        Block::default()
                            .borders(Borders::TOP)
                            .style(Style::default().fg(Color::White))
                            .border_type(BorderType::Plain),
                    );

                rect.render_widget(boundary, size);
                rect.render_widget(tabs, chunks[0]);
                rect.render_widget(footer, chunks[2]);
            })
            .unwrap();
    }
}
