use anyhow::{Context, Result};
use polars::prelude::*;
use std::path::Path;
use crate::filter::FilterExpr;

/// Supported data source types
#[derive(Debug, Clone, Copy)]
pub enum DataSourceType {
    Parquet,
    Csv,
    Sqlite,
}

impl DataSourceType {
    /// Detect data source type from file extension
    pub fn from_path(path: &Path) -> Option<Self> {
        path.extension()
            .and_then(|ext| ext.to_str())
            .and_then(|ext| match ext.to_lowercase().as_str() {
                "parquet" => Some(DataSourceType::Parquet),
                "csv" => Some(DataSourceType::Csv),
                "db" | "sqlite" | "sqlite3" => Some(DataSourceType::Sqlite),
                _ => None,
            })
    }
}

/// Data source abstraction for loading different file formats
pub struct DataSource {
    df: DataFrame,
    source_type: DataSourceType,
}

impl DataSource {
    /// Load data from a file
    pub fn load(path: impl AsRef<Path>) -> Result<Self> {
        let path = path.as_ref();
        let source_type = DataSourceType::from_path(path)
            .context("Unsupported file type. Supported: .parquet, .csv, .db")?;

        let df = match source_type {
            DataSourceType::Parquet => Self::load_parquet(path)?,
            DataSourceType::Csv => Self::load_csv(path)?,
            DataSourceType::Sqlite => Self::load_sqlite(path)?,
        };

        Ok(Self { df, source_type })
    }

    fn load_parquet(path: &Path) -> Result<DataFrame> {
        LazyFrame::scan_parquet(path, Default::default())?
            .collect()
            .context("Failed to load Parquet file")
    }

    fn load_csv(path: &Path) -> Result<DataFrame> {
        CsvReadOptions::default()
            .try_into_reader_with_file_path(Some(path.into()))?
            .finish()
            .context("Failed to load CSV file")
    }

    fn load_sqlite(_path: &Path) -> Result<DataFrame> {
        // For SQLite, we need to query tables. We'll read the first table by default.
        // Users can extend this to select specific tables.
        // This is a simplified version. For full SQLite support, we'd need rusqlite
        // For now, return an error with a helpful message
        anyhow::bail!(
            "SQLite support requires table name. Future version will support table selection."
        )
    }

    /// Get the underlying DataFrame
    pub fn dataframe(&self) -> &DataFrame {
        &self.df
    }

    /// Get the source type
    pub fn source_type(&self) -> DataSourceType {
        self.source_type
    }

    /// Get column names
    pub fn columns(&self) -> Vec<String> {
        self.df.get_column_names().iter().map(|s| s.to_string()).collect()
    }

    /// Get number of rows
    pub fn len(&self) -> usize {
        self.df.height()
    }

    /// Check if empty
    pub fn is_empty(&self) -> bool {
        self.df.height() == 0
    }

    /// Apply a filter expression
    /// Supports advanced filtering with comparison operators and logical expressions
    /// Examples:
    ///   - "InstrumentID:IC2602" - substring match
    ///   - "Price > 5000" - greater than
    ///   - "InstrumentID = IC2602 AND Price > 5000" - logical AND
    pub fn filter(&self, pattern: &str) -> Result<DataFrame> {
        if pattern.is_empty() {
            return Ok(self.df.clone());
        }

        let expr = FilterExpr::parse(pattern)?;
        expr.apply(&self.df)
    }

    /// Legacy filter method for backward compatibility (deprecated)
    #[allow(dead_code)]
    pub fn filter_simple(&self, pattern: &str, column: Option<&str>) -> Result<DataFrame> {
        let df = &self.df;

        if pattern.is_empty() {
            return Ok(df.clone());
        }

        // If column specified, filter that column only
        if let Some(col_name) = column {
            if let Ok(col) = df.column(col_name) {
                // Convert column to string and filter
                let mask = col
                    .str()?
                    .contains_literal(pattern)?;
                return df.filter(&mask).context("Failed to apply filter");
            }
        }

        // Otherwise, search across all string columns
        let mut mask: Option<BooleanChunked> = None;

        for col_name in df.get_column_names() {
            if let Ok(col) = df.column(col_name) {
                // Try to convert to string and search
                if let Ok(str_col) = col.str() {
                    if let Ok(contains) = str_col.contains_literal(pattern) {
                        mask = match mask {
                            None => Some(contains),
                            Some(existing) => Some(existing | contains),
                        };
                    }
                }
            }
        }

        match mask {
            Some(m) => df.filter(&m).context("Failed to apply filter"),
            None => Ok(df.clone()), // No searchable columns found
        }
    }
}
