mod app;
mod data;
mod ui;
mod filter;
mod column_selection;

use anyhow::{Context, Result};
use app::App;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyModifiers},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::CrosstermBackend, Terminal};
use std::io;
use std::path::PathBuf;

fn main() -> Result<()> {
    // Parse command line arguments
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <file.parquet|file.csv|file.db>", args[0]);
        eprintln!("\nSupported formats:");
        eprintln!("  - Parquet (.parquet)");
        eprintln!("  - CSV (.csv)");
        eprintln!("  - SQLite (.db, .sqlite, .sqlite3)");
        std::process::exit(1);
    }

    let file_path = PathBuf::from(&args[1]);
    if !file_path.exists() {
        eprintln!("Error: File '{}' does not exist", file_path.display());
        std::process::exit(1);
    }

    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Show loading screen
    terminal.draw(|f| {
        use ratatui::widgets::{Block, Borders, Paragraph};
        use ratatui::layout::{Layout, Constraint, Direction};
        use ratatui::style::{Color, Style, Modifier};

        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
            .split(f.area());

        let loading_text = format!("Loading file: {}\n\nPlease wait...", file_path.display());
        let paragraph = Paragraph::new(loading_text)
            .block(Block::default().title(" Loading ").borders(Borders::ALL))
            .style(Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD));

        f.render_widget(paragraph, chunks[0]);
    })?;

    // Create app and run
    let result = run_app(&mut terminal, file_path);

    // Restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(e) = result {
        eprintln!("Error: {:?}", e);
        std::process::exit(1);
    }

    Ok(())
}

fn run_app(terminal: &mut Terminal<CrosstermBackend<io::Stdout>>, file_path: PathBuf) -> Result<()> {
    let mut app = App::new(file_path).context("Failed to load data file")?;

    loop {
        terminal.draw(|f| {
            if app.column_selection_mode {
                column_selection::render_column_selection(f, &app, f.area());
            } else {
                ui::render(f, &app);
            }
        })?;

        if app.should_quit {
            break;
        }

        if let Event::Key(key) = event::read()? {
            handle_key_event(&mut app, key.code, key.modifiers);
        }
    }

    Ok(())
}

fn handle_key_event(app: &mut App, key: KeyCode, modifiers: KeyModifiers) {
    if app.column_selection_mode {
        // Column selection mode
        match key {
            KeyCode::Up | KeyCode::Char('k') => {
                app.column_selection_up();
            }
            KeyCode::Down | KeyCode::Char('j') => {
                app.column_selection_down();
            }
            KeyCode::Char(' ') => {
                app.toggle_column_at_cursor();
            }
            KeyCode::Char('a') | KeyCode::Char('A') => {
                // Reset to show all columns
                app.reset_column_selection();
            }
            KeyCode::Enter => {
                app.exit_column_selection_mode();
            }
            KeyCode::Esc => {
                app.exit_column_selection_mode();
            }
            _ => {}
        }
    } else if app.filter_mode {
        match key {
            KeyCode::Char(c) => {
                app.push_filter_char(c);
            }
            KeyCode::Backspace => {
                app.pop_filter_char();
            }
            KeyCode::Delete => {
                app.delete_filter_char();
            }
            KeyCode::Left => {
                app.filter_cursor_left();
            }
            KeyCode::Right => {
                app.filter_cursor_right();
            }
            KeyCode::Home => {
                app.filter_cursor_home();
            }
            KeyCode::End => {
                app.filter_cursor_end();
            }
            KeyCode::Enter => {
                app.apply_filter();
                app.exit_filter_mode();
            }
            KeyCode::Esc => {
                app.exit_filter_mode();
            }
            _ => {}
        }
    } else {
        match key {
            KeyCode::Char('q') | KeyCode::Char('Q') => {
                app.quit();
            }
            KeyCode::Char('c') if modifiers.contains(KeyModifiers::CONTROL) => {
                app.quit();
            }
            KeyCode::Char('/') => {
                app.enter_filter_mode();
            }
            KeyCode::Esc => {
                app.clear_filter();
            }
            KeyCode::Down | KeyCode::Char('j') => {
                app.scroll_down_one();
            }
            KeyCode::Up | KeyCode::Char('k') => {
                app.scroll_up_one();
            }
            KeyCode::Left | KeyCode::Char('h') => {
                app.scroll_left();
            }
            KeyCode::Right | KeyCode::Char('l') => {
                app.scroll_right();
            }
            KeyCode::PageDown => {
                app.scroll_down();
            }
            KeyCode::PageUp => {
                app.scroll_up();
            }
            KeyCode::Home | KeyCode::Char('g') => {
                app.scroll_to_top();
            }
            KeyCode::End | KeyCode::Char('G') => {
                app.scroll_to_bottom();
            }
            KeyCode::Char('n') | KeyCode::Char('N') => {
                app.toggle_line_numbers();
            }
            KeyCode::Char('c') | KeyCode::Char('C') => {
                app.enter_column_selection_mode();
            }
            _ => {}
        }
    }
}
