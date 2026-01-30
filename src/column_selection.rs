use crate::app::App;

/// Render column selection UI
pub fn render_column_selection(f: &mut ratatui::Frame, app: &App, area: ratatui::layout::Rect) {
    use ratatui::{
        layout::{Constraint, Direction, Layout},
        style::{Color, Modifier, Style},
        text::{Line, Span},
        widgets::{Block, Borders, List, ListItem, Paragraph},
    };

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),  // Header
            Constraint::Min(0),     // Column list
            Constraint::Length(3),  // Footer
        ])
        .split(area);

    // Header
    let header_block = Block::default()
        .title(" Column Selection ")
        .borders(Borders::ALL)
        .style(Style::default().fg(Color::Cyan));

    let header_text = vec![Line::from(vec![
        Span::styled("↑↓", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
        Span::raw(" navigate | "),
        Span::styled("Space", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
        Span::raw(" toggle | "),
        Span::styled("a", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
        Span::raw(" all | "),
        Span::styled("Enter", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
        Span::raw(" done | "),
        Span::styled("Esc", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
        Span::raw(" cancel"),
    ])];

    let header = Paragraph::new(header_text).block(header_block);
    f.render_widget(header, chunks[0]);

    // Column list
    let all_columns = app.all_columns();
    let items: Vec<ListItem> = all_columns
        .iter()
        .enumerate()
        .map(|(i, col)| {
            let is_selected = app.is_column_selected(col);
            let checkbox = if is_selected { "[✓]" } else { "[ ]" };
            let content = format!("{} {}", checkbox, col);

            let style = if i == app.column_selection_cursor {
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD)
                    .bg(Color::DarkGray)
            } else if is_selected {
                Style::default().fg(Color::Green)
            } else {
                Style::default().fg(Color::DarkGray)
            };

            ListItem::new(content).style(style)
        })
        .collect();

    let list = List::new(items)
        .block(Block::default().title(" Columns ").borders(Borders::ALL))
        .style(Style::default().fg(Color::White));

    f.render_widget(list, chunks[1]);

    // Footer with stats
    let selected_count = all_columns
        .iter()
        .filter(|col| app.is_column_selected(col))
        .count();

    let footer_block = Block::default()
        .title(" Info ")
        .borders(Borders::ALL);

    let footer_text = format!(
        "Selected: {} / {} columns",
        selected_count,
        all_columns.len()
    );

    let footer = Paragraph::new(footer_text).block(footer_block);
    f.render_widget(footer, chunks[2]);
}
