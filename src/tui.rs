use crate::models::{Vault, VaultEntry};
use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Paragraph, Wrap},
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
    let mut reveal_password = false;
    let mut search_mode = false;
    let mut search_query = String::new();

    loop {
        let filtered_entries = filter_entries(vault, &search_query);

        if selected >= filtered_entries.len() && !filtered_entries.is_empty() {
            selected = filtered_entries.len() - 1;
        }

        terminal.draw(|frame| {
            let area = frame.area();

            let vertical = Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Length(3),
                    Constraint::Length(3),
                    Constraint::Min(8),
                    Constraint::Length(3),
                ])
                .split(area);

            let body = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([Constraint::Percentage(45), Constraint::Percentage(55)])
                .split(vertical[2]);

            let header = Paragraph::new("VaultCLI TUI")
                .block(Block::default().borders(Borders::ALL).title("Dashboard"));

            frame.render_widget(header, vertical[0]);

            let search_title = if search_mode {
                "Search (typing)"
            } else {
                "Search"
            };

            let search_box = Paragraph::new(search_query.clone())
                .block(Block::default().borders(Borders::ALL).title(search_title));

            frame.render_widget(search_box, vertical[1]);

            let items: Vec<ListItem> = filtered_entries
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

            frame.render_widget(list, body[0]);

            let selected_entry = filtered_entries.get(selected).copied();
            let details = selected_entry_details(selected_entry, reveal_password);

            let details_panel = Paragraph::new(details)
                .block(Block::default().borders(Borders::ALL).title("Details"))
                .wrap(Wrap { trim: true });

            frame.render_widget(details_panel, body[1]);

            let footer_text = if search_mode {
                "Type to search | Backspace Delete | Enter/Esc Exit search | q Quit"
            } else {
                "/ Search | ↑/↓ Navigate | r Reveal/Hide password | q Quit"
            };

            let footer = Paragraph::new(footer_text)
                .block(Block::default().borders(Borders::ALL).title("Controls"));

            frame.render_widget(footer, vertical[3]);
        })?;

        if let Event::Key(key) = event::read()? {
            if search_mode {
                match key.code {
                    KeyCode::Esc | KeyCode::Enter => {
                        search_mode = false;
                        reveal_password = false;
                    }
                    KeyCode::Backspace => {
                        search_query.pop();
                        selected = 0;
                        reveal_password = false;
                    }
                    KeyCode::Char('q') if search_query.is_empty() => break,
                    KeyCode::Char(value) => {
                        search_query.push(value);
                        selected = 0;
                        reveal_password = false;
                    }
                    _ => {}
                }
            } else {
                match key.code {
                    KeyCode::Char('q') => break,
                    KeyCode::Char('/') => {
                        search_mode = true;
                    }
                    KeyCode::Char('r') => {
                        reveal_password = !reveal_password;
                    }
                    KeyCode::Down => {
                        if !filtered_entries.is_empty() && selected + 1 < filtered_entries.len() {
                            selected += 1;
                            reveal_password = false;
                        }
                    }
                    KeyCode::Up => {
                        selected = selected.saturating_sub(1);
                        reveal_password = false;
                    }
                    _ => {}
                }
            }
        }
    }

    Ok(())
}

fn filter_entries<'a>(vault: &'a Vault, query: &str) -> Vec<&'a VaultEntry> {
    if query.trim().is_empty() {
        return vault.entries.iter().collect();
    }

    let query = query.to_lowercase();

    vault
        .entries
        .iter()
        .filter(|entry| {
            entry.title.to_lowercase().contains(&query)
                || entry.username.to_lowercase().contains(&query)
                || entry
                    .website
                    .as_ref()
                    .map(|website| website.to_lowercase().contains(&query))
                    .unwrap_or(false)
                || entry
                    .category
                    .as_ref()
                    .map(|category| category.to_lowercase().contains(&query))
                    .unwrap_or(false)
        })
        .collect()
}

fn selected_entry_details(entry: Option<&VaultEntry>, reveal_password: bool) -> Vec<Line<'static>> {
    match entry {
        Some(entry) => {
            let password = if reveal_password {
                entry.password.clone()
            } else {
                "********".to_string()
            };

            vec![
                Line::from(vec![Span::raw("Title: "), Span::raw(entry.title.clone())]),
                Line::from(vec![
                    Span::raw("Username: "),
                    Span::raw(entry.username.clone()),
                ]),
                Line::from(vec![Span::raw("Password: "), Span::raw(password)]),
                Line::from(vec![
                    Span::raw("Website: "),
                    Span::raw(entry.website.clone().unwrap_or_else(|| "-".to_string())),
                ]),
                Line::from(vec![
                    Span::raw("Category: "),
                    Span::raw(entry.category.clone().unwrap_or_else(|| "-".to_string())),
                ]),
                Line::from(vec![
                    Span::raw("Notes: "),
                    Span::raw(entry.notes.clone().unwrap_or_else(|| "-".to_string())),
                ]),
            ]
        }
        None => vec![Line::from("No entries available.")],
    }
}