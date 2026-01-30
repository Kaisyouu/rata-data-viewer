# Data Sources - Technical Documentation

## Overview

The data module provides an abstraction layer for loading and manipulating data from different file formats. It uses Polars DataFrame as the underlying data structure for unified data handling.

## DataSource API

### Core Type: `DataSource`

```rust
pub struct DataSource {
    df: DataFrame,
    source_type: DataSourceType,
}
```

**Fields**:
- `df`: The Polars DataFrame containing the loaded data
- `source_type`: Enum indicating the original file format

### DataSourceType Enum

```rust
pub enum DataSourceType {
    Parquet,
    Csv,
    Sqlite,
}
```

**Purpose**: Identifies the type of data source for logging and type-specific operations.

## Loading Data

### Method: `load(path: impl AsRef<Path>) -> Result<Self>`

**Purpose**: Factory method to load data from a file, auto-detecting format.

**Algorithm**:
1. Extract file extension from path
2. Match extension to `DataSourceType`
3. Call format-specific loader
4. Return `DataSource` instance

**Example**:
```rust
let data_source = DataSource::load("/path/to/data.parquet")?;
```

**Error Cases**:
- File extension not recognized → `anyhow::Error`
- File doesn't exist → I/O error
- File corrupted/invalid format → Parse error

### Format-Specific Loaders

#### Parquet: `load_parquet(path: &Path) -> Result<DataFrame>`

**Implementation**:
```rust
LazyFrame::scan_parquet(path, Default::default())?
    .collect()
    .context("Failed to load Parquet file")
```

**How it works**:
1. Uses Polars lazy scan for efficient loading
2. Scans Parquet metadata first
3. Collects data into memory
4. Parquet's columnar format allows selective column reading

**Performance**: Optimized for large files due to columnar storage.

#### CSV: `load_csv(path: &Path) -> Result<DataFrame>`

**Implementation**:
```rust
CsvReadOptions::default()
    .try_into_reader_with_file_path(Some(path.into()))?
    .finish()
    .context("Failed to load CSV file")
```

**How it works**:
1. Uses Polars CSV reader with default options
2. Automatically detects:
   - Delimiter (comma, tab, etc.)
   - Header row
   - Data types
3. Parses entire file into DataFrame

**Performance**: Good for small to medium files. For very large CSVs, consider converting to Parquet.

#### SQLite: `load_sqlite(path: &Path) -> Result<DataFrame>`

**Current Implementation**:
```rust
anyhow::bail!(
    "SQLite support requires table name. Future version will support table selection."
)
```

**Status**: Not yet implemented. Returns error with message.

**Planned Implementation**:
1. Open SQLite connection
2. Query available tables
3. If multiple tables: present selection UI
4. Execute SQL query to read table
5. Convert result to DataFrame

**Dependencies needed**:
- `rusqlite` for SQLite connection
- Or use Polars' `sql` feature (experimental)

## Data Access Methods

### `dataframe(&self) -> &DataFrame`

Returns reference to underlying Polars DataFrame.

**Use case**: Direct access for advanced DataFrame operations.

### `columns(&self) -> Vec<String>`

Returns list of column names.

**Implementation**:
```rust
self.df.get_column_names()
    .iter()
    .map(|s| s.to_string())
    .collect()
```

### `len(&self) -> usize` / `is_empty(&self) -> bool`

Returns number of rows in the DataFrame.

## Filtering

### Method: `filter(&self, pattern: &str, column: Option<&str>) -> Result<DataFrame>`

**Purpose**: Apply text-based filter to data.

**Parameters**:
- `pattern`: Search string (substring match)
- `column`: Optional column name to search (if None, searches all columns)

**Algorithm**:

```
if pattern is empty:
    return original dataframe

if column is specified:
    filter that column only
    return filtered dataframe

otherwise:
    for each column in dataframe:
        if column is string-compatible:
            create boolean mask for rows containing pattern
            combine masks with OR operation

    apply combined mask to dataframe
    return filtered result
```

**Implementation Details**:

1. **Column-specific filtering**:
```rust
let mask = col.str()?.contains_literal(pattern)?;
df.filter(&mask)
```

2. **Multi-column filtering**:
```rust
let mut mask: Option<BooleanChunked> = None;

for col_name in df.get_column_names() {
    if let Ok(str_col) = col.str() {
        if let Ok(contains) = str_col.contains_literal(pattern) {
            mask = match mask {
                None => Some(contains),
                Some(existing) => Some(existing | contains),
            };
        }
    }
}
```

**Performance**:
- Uses Polars vectorized operations (fast)
- Only searches string-compatible columns
- Short-circuits on first match per row

**Limitations**:
- Substring matching only (no regex)
- Case-sensitive
- Doesn't search numeric columns (could be added by converting to string)

## TableData Structure

### Purpose
Convert DataFrame to UI-friendly format with pagination.

```rust
pub struct TableData {
    pub headers: Vec<String>,
    pub rows: Vec<Vec<String>>,
    pub total_rows: usize,
}
```

### Method: `from_dataframe(df: &DataFrame, offset: usize, limit: usize) -> Self`

**Purpose**: Extract a page of data from DataFrame as strings.

**Algorithm**:
```
1. Extract column names as headers
2. Calculate end index: min(offset + limit, total_rows)
3. For each row in range [offset, end):
     For each column:
         Get cell value at (row, column)
         Convert to string
         Handle NULL values
     Add row to result
4. Return TableData with headers, rows, and total count
```

**String Conversion**:
- Uses Polars' `AnyValue` formatting
- NULL values → "NULL" string
- Numbers → decimal representation
- Dates → ISO format
- Booleans → "true"/"false"

**Performance**: Only converts the current page to strings, not entire dataset.

## Extending Data Sources

### Adding a New File Format

Example: Adding JSON support

1. **Add to enum**:
```rust
pub enum DataSourceType {
    Parquet,
    Csv,
    Sqlite,
    Json,  // new
}
```

2. **Update extension detection**:
```rust
"json" => Some(DataSourceType::Json),
```

3. **Implement loader**:
```rust
fn load_json(path: &Path) -> Result<DataFrame> {
    let file = std::fs::File::open(path)?;
    let reader = std::io::BufReader::new(file);

    // Use Polars JsonReader or serde_json
    JsonReader::new(reader)
        .finish()
        .context("Failed to load JSON file")
}
```

4. **Add to match in `load()`**:
```rust
DataSourceType::Json => Self::load_json(path)?,
```

### Adding Advanced Filtering

Example: Regex filtering

1. **Add dependency**:
```toml
regex = "1.11"
```

2. **Extend filter method**:
```rust
pub fn filter_regex(&self, pattern: &str, column: Option<&str>) -> Result<DataFrame> {
    let re = regex::Regex::new(pattern)?;

    // Similar logic but use regex matching
    let mask = col.str()?
        .into_iter()
        .map(|opt_s| {
            opt_s.map(|s| re.is_match(s)).unwrap_or(false)
        })
        .collect::<BooleanChunked>();

    self.df.filter(&mask).context("Failed to apply regex filter")
}
```

### Adding Numeric Filtering

Example: Filter by numeric range

```rust
pub fn filter_numeric(&self, column: &str, min: f64, max: f64) -> Result<DataFrame> {
    let col = self.df.column(column)?;

    // Assume numeric column
    let mask = col.gt(min)? & col.lt(max)?;

    self.df.filter(&mask).context("Failed to apply numeric filter")
}
```

## Polars Integration

### Why Polars?

**Advantages**:
1. **Unified API**: Single DataFrame type for all formats
2. **Performance**: Rust-native, zero-copy operations
3. **Lazy Evaluation**: Optimizes query plans
4. **Rich Features**: Filtering, aggregation, joins, etc.
5. **Native Format Support**: Parquet, CSV, JSON, etc.

### Alternative: Arrow

Could also use Apache Arrow directly:
- Lower-level API
- More control over memory layout
- Requires more manual handling

### Alternative: Custom per-format handling

Could use individual crates:
- `parquet` crate for Parquet
- `csv` crate for CSV
- `rusqlite` for SQLite

**Tradeoffs**:
- More control, less abstraction
- More code to maintain
- Harder to implement cross-format features

## Memory Considerations

### Current Approach: Load All Data

**Pros**:
- Fast scrolling and filtering
- Simple implementation
- All operations work on complete dataset

**Cons**:
- Memory usage scales with file size
- Large files (>1GB) may cause issues
- Initial load time for large files

### Future: Streaming/Chunked Loading

For very large files, consider:

1. **Lazy Frame queries** (Polars native):
```rust
LazyFrame::scan_parquet(path, Default::default())?
    .slice(offset, limit)
    .collect()
```

2. **Chunked reading**:
- Load data in chunks
- Only keep current chunk in memory
- Prefetch adjacent chunks

3. **Memory-mapped files**:
- Use memory mapping for Parquet
- OS handles paging
- Appears as in-memory but disk-backed

## Testing

### Unit Tests

Example tests to add:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_parquet() {
        let ds = DataSource::load("test_data/sample.parquet").unwrap();
        assert!(ds.len() > 0);
    }

    #[test]
    fn test_filter() {
        let ds = DataSource::load("test_data/sample.parquet").unwrap();
        let filtered = ds.filter("test_pattern", None).unwrap();
        assert!(filtered.height() <= ds.len());
    }
}
```

### Integration Tests

Create test data files:
- Small Parquet file
- CSV with various data types
- SQLite database with sample tables

Run tests:
```bash
cargo test
```
