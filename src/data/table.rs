use polars::prelude::*;

/// Table data structure for UI display
pub struct TableData {
    pub headers: Vec<String>,
    pub rows: Vec<Vec<String>>,
    pub total_rows: usize,
}

impl TableData {
    /// Create TableData from a DataFrame with pagination
    pub fn from_dataframe(df: &DataFrame, offset: usize, limit: usize) -> Self {
        let headers = df.get_column_names()
            .iter()
            .map(|s| s.to_string())
            .collect();

        let total_rows = df.height();
        let end = (offset + limit).min(total_rows);

        let rows = (offset..end)
            .map(|idx| {
                df.get_columns()
                    .iter()
                    .map(|col| {
                        col.get(idx)
                            .map(|av| format!("{}", av))
                            .unwrap_or_else(|_| "NULL".to_string())
                    })
                    .collect()
            })
            .collect();

        Self {
            headers,
            rows,
            total_rows,
        }
    }

    /// Filter to only show selected columns
    pub fn filter_columns(&mut self, selected_columns: &[String]) {
        // Find indices of selected columns
        let indices: Vec<usize> = selected_columns
            .iter()
            .filter_map(|col| self.headers.iter().position(|h| h == col))
            .collect();

        if indices.is_empty() {
            // No columns selected, show nothing
            self.headers.clear();
            self.rows.clear();
            return;
        }

        // Filter headers
        let new_headers: Vec<String> = indices
            .iter()
            .map(|&i| self.headers[i].clone())
            .collect();

        // Filter rows
        let new_rows: Vec<Vec<String>> = self.rows
            .iter()
            .map(|row| {
                indices
                    .iter()
                    .map(|&i| row[i].clone())
                    .collect()
            })
            .collect();

        self.headers = new_headers;
        self.rows = new_rows;
    }

    /// Get number of columns
    pub fn num_columns(&self) -> usize {
        self.headers.len()
    }

    /// Get number of visible rows
    pub fn num_rows(&self) -> usize {
        self.rows.len()
    }
}
