use crate::app::App;
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Cell, Paragraph, Row, Table},
    Frame,
};

/// Render the UI
pub fn render(f: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // Header
            Constraint::Min(0),     // Table
            Constraint::Length(3), // Footer/Filter
            Constraint::Length(1), // Status bar
        ])
        .split(f.area());

    render_header(f, app, chunks[0]);
    render_table(f, app, chunks[1]);
    render_footer(f, app, chunks[2]);
    render_status(f, app, chunks[3]);
}

fn render_header(f: &mut Frame, app: &App, area: Rect) {
    let title = format!(" {} ", app.file_path.display());
    let block = Block::default()
        .title(title)
        .borders(Borders::ALL)
        .style(Style::default().fg(Color::Cyan));

    let text = vec![
        Line::from(vec![
            Span::styled("/", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
            Span::raw(" filter | "),
            Span::styled("c", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
            Span::raw(" columns | "),
            Span::styled("n", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
            Span::raw(" line# | "),
            Span::styled("↑↓←→", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
            Span::raw(" navigate | "),
            Span::styled("q", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
            Span::raw(" quit"),
        ]),
    ];

    let paragraph = Paragraph::new(text).block(block);
    f.render_widget(paragraph, area);
}

fn render_table(f: &mut Frame, app: &App, area: Rect) {
    let table_data = app.current_page();

    if table_data.num_columns() == 0 {
        let block = Block::default()
            .title(" No Data ")
            .borders(Borders::ALL);
        let paragraph = Paragraph::new("No data to display").block(block);
        f.render_widget(paragraph, area);
        return;
    }

    // Build headers with optional line number column
    let mut all_headers = Vec::new();
    if app.show_line_numbers {
        all_headers.push("#".to_string());
    }

    // Apply column offset for horizontal scrolling
    let visible_data_headers: Vec<String> = table_data.headers
        .iter()
        .skip(app.column_offset)
        .take(10) // Show up to 10 columns at a time
        .cloned()
        .collect();

    all_headers.extend(visible_data_headers);

    let header_cells = all_headers
        .iter()
        .map(|h| {
            Cell::from(h.as_str())
                .style(Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD))
        });

    let header = Row::new(header_cells)
        .height(1)
        .bottom_margin(1);

    let rows = table_data.rows.iter().enumerate().map(|(idx, row)| {
        let mut cells = Vec::new();

        // Add line number if enabled
        if app.show_line_numbers {
            let line_num = app.scroll_offset + idx + 1;
            cells.push(Cell::from(format!("{}", line_num))
                .style(Style::default().fg(Color::DarkGray)));
        }

        // Add data cells
        let data_cells: Vec<Cell> = row
            .iter()
            .skip(app.column_offset)
            .take(10)
            .map(|cell| {
                // Truncate long cells
                let display = if cell.len() > 50 {
                    format!("{}...", &cell[..47])
                } else {
                    cell.clone()
                };
                Cell::from(display)
            })
            .collect();

        cells.extend(data_cells);
        Row::new(cells).height(1)
    });

    // Calculate column widths dynamically
    let num_visible_cols = all_headers.len();
    let widths: Vec<Constraint> = if app.show_line_numbers && num_visible_cols > 1 {
        // Give line number column fixed width, split rest evenly
        let mut w = vec![Constraint::Length(6)]; // Line number column
        let remaining_cols = num_visible_cols - 1;
        if remaining_cols > 0 {
            w.extend(vec![Constraint::Percentage(100 / remaining_cols as u16); remaining_cols]);
        }
        w
    } else if num_visible_cols > 0 {
        vec![Constraint::Percentage(100 / num_visible_cols as u16); num_visible_cols]
    } else {
        vec![Constraint::Percentage(100)]
    };

    let table = Table::new(rows, widths)
        .header(header)
        .block(
            Block::default()
                .title(format!(
                    " Data (rows {}-{} of {}) ",
                    app.scroll_offset + 1,
                    (app.scroll_offset + table_data.num_rows()).min(table_data.total_rows),
                    table_data.total_rows
                ))
                .borders(Borders::ALL)
        )
        .style(Style::default().fg(Color::White));

    f.render_widget(table, area);
}

fn render_footer(f: &mut Frame, app: &App, area: Rect) {
    if app.filter_mode {
        let block = Block::default()
            .title(" Advanced Filter (Enter: apply, Esc: cancel) ")
            .borders(Borders::ALL)
            .style(Style::default().fg(Color::Green));

        // Insert cursor indicator in the pattern
        let mut display_text = app.filter_pattern.clone();
        if app.filter_cursor <= display_text.len() {
            display_text.insert(app.filter_cursor, '│'); // Use │ as cursor
        }

        let text = vec![
            Line::from(display_text),
            Line::from(vec![
                Span::styled("Operators: ", Style::default().fg(Color::Cyan)),
                Span::raw("= != > < >= <= :contains"),
            ]),
            Line::from(vec![
                Span::styled("Examples: ", Style::default().fg(Color::Yellow)),
                Span::raw("Price > 5000 | InstrumentID = IC2602 AND Price > 5000"),
            ]),
        ];
        let paragraph = Paragraph::new(text).block(block);
        f.render_widget(paragraph, area);
    } else if !app.filter_pattern.is_empty() {
        let block = Block::default()
            .title(" Active Filter (press Esc to clear) ")
            .borders(Borders::ALL)
            .style(Style::default().fg(Color::Yellow));

        let paragraph = Paragraph::new(app.filter_pattern.clone()).block(block);
        f.render_widget(paragraph, area);
    } else {
        let block = Block::default()
            .title(" Info ")
            .borders(Borders::ALL);

        let text = format!(
            "Total rows: {} | Columns: {} (showing {}-{})",
            app.original_total_rows(),
            app.current_page().headers.len(),
            app.column_offset + 1,
            (app.column_offset + 10).min(app.current_page().headers.len())
        );
        let paragraph = Paragraph::new(text).block(block);
        f.render_widget(paragraph, area);
    }
}

fn render_status(f: &mut Frame, app: &App, area: Rect) {
    let status = if let Some(err) = &app.error_message {
        Span::styled(
            format!(" ERROR: {} ", err),
            Style::default().fg(Color::Red).add_modifier(Modifier::BOLD)
        )
    } else {
        Span::styled(
            " Ready ",
            Style::default().fg(Color::Green)
        )
    };

    let paragraph = Paragraph::new(Line::from(vec![status]));
    f.render_widget(paragraph, area);
}
