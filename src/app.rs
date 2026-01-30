use crate::data::{DataSource, TableData};
use anyhow::Result;
use polars::prelude::*;
use std::path::PathBuf;

/// Application state
pub struct App {
    /// Original data source
    data_source: DataSource,
    /// Filtered dataframe
    filtered_df: DataFrame,
    /// Current scroll position (row offset)
    pub scroll_offset: usize,
    /// Current column offset for horizontal scrolling
    pub column_offset: usize,
    /// Number of rows to display per page
    pub page_size: usize,
    /// Search/filter pattern
    pub filter_pattern: String,
    /// Cursor position in filter input (index in string)
    pub filter_cursor: usize,
    /// Whether we're in filter input mode
    pub filter_mode: bool,
    /// Column to filter (None = search all columns)
    pub filter_column: Option<String>,
    /// File path being viewed
    pub file_path: PathBuf,
    /// Error message to display
    pub error_message: Option<String>,
    /// Whether to quit the application
    pub should_quit: bool,
    /// Whether to show line numbers
    pub show_line_numbers: bool,
    /// Whether we're in column selection mode
    pub column_selection_mode: bool,
    /// Cursor position in column selection list
    pub column_selection_cursor: usize,
    /// Selected columns (None = all columns, Some = only selected ones)
    pub selected_columns: Option<Vec<String>>,
}

impl App {
    /// Create a new App instance
    pub fn new(file_path: PathBuf) -> Result<Self> {
        let data_source = DataSource::load(&file_path)?;
        let filtered_df = data_source.dataframe().clone();

        Ok(Self {
            data_source,
            filtered_df,
            scroll_offset: 0,
            column_offset: 0,
            page_size: 20,
            filter_pattern: String::new(),
            filter_cursor: 0,
            filter_mode: false,
            filter_column: None,
            file_path,
            error_message: None,
            should_quit: false,
            show_line_numbers: true,
            column_selection_mode: false,
            column_selection_cursor: 0,
            selected_columns: None, // None means all columns visible
        })
    }

    /// Get current page of data for display
    pub fn current_page(&self) -> TableData {
        let mut table_data = TableData::from_dataframe(&self.filtered_df, self.scroll_offset, self.page_size);

        // Apply column filtering if selected_columns is set
        if let Some(ref selected_cols) = self.selected_columns {
            table_data.filter_columns(selected_cols);
        }

        table_data
    }

    /// Scroll down by one page
    pub fn scroll_down(&mut self) {
        let max_offset = self.filtered_df.height().saturating_sub(self.page_size);
        self.scroll_offset = (self.scroll_offset + self.page_size).min(max_offset);
    }

    /// Scroll up by one page
    pub fn scroll_up(&mut self) {
        self.scroll_offset = self.scroll_offset.saturating_sub(self.page_size);
    }

    /// Scroll down by one row
    pub fn scroll_down_one(&mut self) {
        let max_offset = self.filtered_df.height().saturating_sub(self.page_size);
        self.scroll_offset = (self.scroll_offset + 1).min(max_offset);
    }

    /// Scroll up by one row
    pub fn scroll_up_one(&mut self) {
        self.scroll_offset = self.scroll_offset.saturating_sub(1);
    }

    /// Scroll to top
    pub fn scroll_to_top(&mut self) {
        self.scroll_offset = 0;
    }

    /// Scroll to bottom
    pub fn scroll_to_bottom(&mut self) {
        self.scroll_offset = self.filtered_df.height().saturating_sub(self.page_size);
    }

    /// Scroll right
    pub fn scroll_right(&mut self) {
        let max_columns = self.data_source.columns().len();
        if self.column_offset < max_columns.saturating_sub(1) {
            self.column_offset += 1;
        }
    }

    /// Scroll left
    pub fn scroll_left(&mut self) {
        self.column_offset = self.column_offset.saturating_sub(1);
    }

    /// Enter filter mode
    pub fn enter_filter_mode(&mut self) {
        self.filter_mode = true;
        self.filter_cursor = self.filter_pattern.len(); // Move cursor to end
        self.error_message = None;
    }

    /// Exit filter mode
    pub fn exit_filter_mode(&mut self) {
        self.filter_mode = false;
    }

    /// Add character to filter pattern at cursor position
    pub fn push_filter_char(&mut self, c: char) {
        self.filter_pattern.insert(self.filter_cursor, c);
        self.filter_cursor += 1;
    }

    /// Remove character before cursor (Backspace)
    pub fn pop_filter_char(&mut self) {
        if self.filter_cursor > 0 {
            self.filter_cursor -= 1;
            self.filter_pattern.remove(self.filter_cursor);
        }
    }

    /// Remove character at cursor (Delete key)
    pub fn delete_filter_char(&mut self) {
        if self.filter_cursor < self.filter_pattern.len() {
            self.filter_pattern.remove(self.filter_cursor);
        }
    }

    /// Move cursor left in filter input
    pub fn filter_cursor_left(&mut self) {
        if self.filter_cursor > 0 {
            self.filter_cursor -= 1;
        }
    }

    /// Move cursor right in filter input
    pub fn filter_cursor_right(&mut self) {
        if self.filter_cursor < self.filter_pattern.len() {
            self.filter_cursor += 1;
        }
    }

    /// Move cursor to start of filter input
    pub fn filter_cursor_home(&mut self) {
        self.filter_cursor = 0;
    }

    /// Move cursor to end of filter input
    pub fn filter_cursor_end(&mut self) {
        self.filter_cursor = self.filter_pattern.len();
    }

    /// Clear filter pattern
    pub fn clear_filter(&mut self) {
        self.filter_pattern.clear();
        self.filter_cursor = 0;
        self.apply_filter();
    }

    /// Apply the current filter
    pub fn apply_filter(&mut self) {
        // Use new advanced filter expression parser
        match self.data_source.filter(&self.filter_pattern) {
            Ok(df) => {
                self.filtered_df = df;
                self.scroll_offset = 0; // Reset scroll when filter changes
                self.error_message = None;
            }
            Err(e) => {
                self.error_message = Some(format!("Filter error: {}", e));
            }
        }
    }

    /// Parse filter pattern to extract column name and value (deprecated, kept for reference)
    /// The new filter system handles this automatically
    #[allow(dead_code)]
    fn parse_filter_pattern(&self, pattern: &str) -> (Option<String>, String) {
        // Try colon separator first
        if let Some(colon_pos) = pattern.find(':') {
            let col = pattern[..colon_pos].trim().to_string();
            let val = pattern[colon_pos + 1..].trim().to_string();

            // Verify column exists
            if self.data_source.columns().contains(&col) {
                return (Some(col), val);
            }
        }

        // Try equals separator
        if let Some(eq_pos) = pattern.find('=') {
            let col = pattern[..eq_pos].trim().to_string();
            let val = pattern[eq_pos + 1..].trim().to_string();

            // Verify column exists
            if self.data_source.columns().contains(&col) {
                return (Some(col), val);
            }
        }

        // No column specified or column not found, search all columns
        (None, pattern.to_string())
    }

    /// Get total number of rows (after filtering)
    pub fn total_rows(&self) -> usize {
        self.filtered_df.height()
    }

    /// Get total number of rows (before filtering)
    pub fn original_total_rows(&self) -> usize {
        self.data_source.len()
    }

    /// Quit the application
    pub fn quit(&mut self) {
        self.should_quit = true;
    }

    /// Toggle line numbers display
    pub fn toggle_line_numbers(&mut self) {
        self.show_line_numbers = !self.show_line_numbers;
    }

    /// Enter column selection mode
    pub fn enter_column_selection_mode(&mut self) {
        self.column_selection_mode = true;
        self.column_selection_cursor = 0;

        // Initialize selected_columns if not set (start with all columns selected)
        if self.selected_columns.is_none() {
            self.selected_columns = Some(self.data_source.columns());
        }
    }

    /// Exit column selection mode
    pub fn exit_column_selection_mode(&mut self) {
        self.column_selection_mode = false;
    }

    /// Move cursor up in column selection
    pub fn column_selection_up(&mut self) {
        if self.column_selection_cursor > 0 {
            self.column_selection_cursor -= 1;
        }
    }

    /// Move cursor down in column selection
    pub fn column_selection_down(&mut self) {
        let max_cursor = self.data_source.columns().len().saturating_sub(1);
        if self.column_selection_cursor < max_cursor {
            self.column_selection_cursor += 1;
        }
    }

    /// Toggle column visibility at cursor position
    pub fn toggle_column_at_cursor(&mut self) {
        let all_columns = self.data_source.columns();
        if self.column_selection_cursor >= all_columns.len() {
            return;
        }

        let column_name = &all_columns[self.column_selection_cursor];

        if let Some(ref mut selected) = self.selected_columns {
            if let Some(pos) = selected.iter().position(|c| c == column_name) {
                // Column is selected, remove it
                selected.remove(pos);
            } else {
                // Column is not selected, add it
                selected.push(column_name.clone());
            }
        }
    }

    /// Reset column selection to show all columns
    pub fn reset_column_selection(&mut self) {
        self.selected_columns = None;
        self.column_offset = 0;
    }

    /// Check if a column is currently selected/visible
    pub fn is_column_selected(&self, column_name: &str) -> bool {
        match &self.selected_columns {
            None => true, // All columns visible
            Some(selected) => selected.contains(&column_name.to_string()),
        }
    }

    /// Get all available columns
    pub fn all_columns(&self) -> Vec<String> {
        self.data_source.columns()
    }
}
