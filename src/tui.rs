use crate::models::Vault;
use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Modifier, Style},
    text::Line,
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Terminal,
};
use std::io;

pub fn run_tui(vault: &Vault) -> io::Result<()> {
    enable_raw_mode()?;

    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let result = render_loop(&mut terminal, vault);

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    result
}

fn render_loop(
    terminal: &mut Terminal<CrosstermBackend<io::Stdout>>,
    vault: &Vault,
) -> io::Result<()> {
    let mut selected: usize = 0;

    loop {
        terminal.draw(|frame| {
            let area = frame.area();

            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Length(3),
                    Constraint::Min(5),
                    Constraint::Length(3),
                ])
                .split(area);

            let header = Paragraph::new("VaultCLI TUI")
                .block(Block::default().borders(Borders::ALL).title("Dashboard"));

            frame.render_widget(header, chunks[0]);

            let items: Vec<ListItem> = vault
                .entries
                .iter()
                .enumerate()
                .map(|(index, entry)| {
                    let marker = if index == selected { ">" } else { " " };

                    let category = entry
                        .category
                        .as_ref()
                        .map(|value| value.as_str())
                        .unwrap_or("uncategorized");

                    ListItem::new(Line::from(format!(
                        "{} {} | {} | {}",
                        marker, entry.title, entry.username, category
                    )))
                })
                .collect();

            let list = List::new(items)
                .block(Block::default().borders(Borders::ALL).title("Entries"))
                .highlight_style(Style::default().add_modifier(Modifier::BOLD));

            frame.render_widget(list, chunks[1]);

            let footer = Paragraph::new("↑/↓ Navigate | q Quit")
                .block(Block::default().borders(Borders::ALL).title("Controls"));

            frame.render_widget(footer, chunks[2]);
        })?;

        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Char('q') => break,
                KeyCode::Down => {
                    if !vault.entries.is_empty() && selected + 1 < vault.entries.len() {
                        selected += 1;
                    }
                }
                KeyCode::Up => {
                    selected = selected.saturating_sub(1);
                }
                _ => {}
            }
        }
    }

    Ok(())
}