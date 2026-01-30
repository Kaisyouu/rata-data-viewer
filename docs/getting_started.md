# Getting Started with Rata Data Viewer

## Installation

### Prerequisites
- Rust toolchain (1.70+)
- Cargo package manager

### Building from Source

```bash
# Clone the repository
git clone <repository-url>
cd rata-data-viewer

# Build the project
cargo build --release

# The binary will be at: target/release/rata-data-viewer
```

### Running Directly with Cargo

```bash
cargo run -- /path/to/your/data.parquet
```

## Usage

### Basic Usage

```bash
# View a Parquet file
./rata-data-viewer data.parquet

# View a CSV file
./rata-data-viewer data.csv

# View a SQLite database
./rata-data-viewer data.db
```

### Keyboard Controls

#### Navigation
- `↑` / `k` - Scroll up one row
- `↓` / `j` - Scroll down one row
- `←` / `h` - Scroll left (previous columns)
- `→` / `l` - Scroll right (next columns)
- `PgUp` - Scroll up one page
- `PgDn` - Scroll down one page
- `Home` / `g` - Go to top
- `End` / `G` - Go to bottom

#### Filtering
- `/` - Enter filter mode
- Type your search pattern (see Filter Syntax below)
- `Enter` - Apply filter
- `Esc` - Cancel filter input (or clear active filter)

#### Display
- `n` / `N` - Toggle line numbers on/off

#### Application
- `q` / `Q` - Quit application
- `Ctrl+C` - Force quit

## Filtering

### Filter Syntax

The viewer supports multiple filter formats:

1. **Column-specific filtering** (recommended for structured data):
   ```
   ColumnName:value      (using colon)
   ColumnName=value      (using equals)
   ```

2. **Global search** (searches all columns):
   ```
   value
   ```

### Filter Examples

Assuming your data has columns like `InstrumentID`, `UpdateTime`, `Price`:

```bash
# Filter by InstrumentID
InstrumentID:IC2602

# Filter by UpdateTime
UpdateTime=09:30:00

# Filter by partial match
InstrumentID:IC26        # Matches IC2601, IC2602, etc.

# Search all columns
IC2602                   # Finds "IC2602" in any column
```

### How Filtering Works

**Column-specific filtering**:
- Searches only the specified column
- Column name must match exactly (case-sensitive)
- Value matching is substring-based (contains)
- If column doesn't exist, falls back to global search

**Global search**:
- Searches across ALL columns
- Only searches string-compatible columns
- Uses substring matching (contains)
- Results include rows where ANY column matches

**Filter behavior**:
- Case-sensitive substring matching
- Filters are applied to the entire dataset
- Results are displayed with pagination
- Scroll position resets to top when filter applied

### Current Limitations
- Substring matching only (no regex yet)
- Case-sensitive
- No numeric range filtering (e.g., "price > 100")
- No date range filtering

### Planned Features
- Regex support (`InstrumentID~IC26.*`)
- Case-insensitive option (`InstrumentID:i:ic2602`)
- Numeric comparisons (`Price>100`, `Volume>=1000`)
- Date range filtering (`UpdateTime>09:30:00`)
- Multiple conditions (`InstrumentID:IC2602 AND Price>100`)

## Display Features

### Line Numbers

Press `n` to toggle line numbers on/off.

**When enabled**:
- Shows row numbers in leftmost column
- Numbers are 1-indexed (first row is 1)
- Reflects actual position in filtered data
- Gray color to distinguish from data
- Takes ~6 characters of width

**Use cases**:
- Reference specific rows in discussions
- Track position in large datasets
- Count rows visually

## File Format Support

### Parquet Files
- **Extensions**: `.parquet`
- **Features**: Full support, optimized reading
- **Use case**: Large datasets, columnar data

### CSV Files
- **Extensions**: `.csv`
- **Features**: Full support with automatic delimiter detection
- **Use case**: Spreadsheet exports, simple data

### SQLite Databases
- **Extensions**: `.db`, `.sqlite`, `.sqlite3`
- **Current Status**: Limited support (error message displayed)
- **Planned**: Table selection UI in future version

## Display Features

### Visible Columns
- Shows up to 10 columns at a time
- Use arrow keys to scroll horizontally
- Column indicator shows which columns are visible (e.g., "1-10" of total)

### Cell Truncation
- Long cell values are truncated to 50 characters
- Truncated cells show "..." at the end
- Prevents terminal overflow

### Row Pagination
- Default page size: 20 rows
- Smooth scrolling by single row or full page
- Status shows current position (e.g., "rows 1-20 of 1000")

## Performance Tips

### Large Files
For files with millions of rows:
- Initial load may take a few seconds
- Once loaded, scrolling is instant
- Filtering is optimized (DataFrame-level operations)

### Memory Usage
- Entire file is loaded into memory
- For very large files (>1GB), consider:
  - Filtering externally before loading
  - Using tools like `duckdb` to pre-filter

### Optimal Formats
- **Parquet**: Best for large datasets (compressed, columnar)
- **CSV**: Good for small to medium datasets
- **SQLite**: Good for structured data with relationships

## Troubleshooting

### Application Won't Start
**Error**: "File does not exist"
- Check the file path is correct
- Use absolute path if relative path fails

**Error**: "Unsupported file type"
- Ensure file has correct extension (.parquet, .csv, .db)
- Check file is not corrupted

### Data Not Displaying
**Issue**: Empty table shown
- Check if file actually contains data
- Try opening file with another tool to verify

**Issue**: Garbled characters
- Check terminal encoding (should be UTF-8)
- Some special characters may not display correctly

### Terminal Issues
**Issue**: Terminal garbled after crash
- Run: `reset` to restore terminal
- Or close and reopen terminal

**Issue**: Colors not showing
- Ensure terminal supports colors
- Try a modern terminal (iTerm2, Windows Terminal, Alacritty)

## Examples

### Example 1: Viewing Market Data
```bash
./rata-data-viewer ~/data/CTPDepthMarketData.parquet

# Once open:
# - Press / to filter
# - Type "000001" to find specific instrument
# - Press Enter to apply
# - Use ↑↓ to browse results
# - Press Esc to clear filter
```

### Example 2: Exploring CSV Data
```bash
./rata-data-viewer sales_data.csv

# Navigation:
# - Press End to jump to last page
# - Press Home to return to top
# - Use PgUp/PgDn to browse pages
```

### Example 3: Quick Data Check
```bash
# Check first few rows
cargo run -- data.parquet
# Once open, just scroll to verify data looks correct
# Press q to quit
```

## Integration with Other Tools

### Exporting Filtered Data
Currently not supported directly, but you can:
1. Use Polars/Pandas to filter externally
2. Future version will support export

### Piping Data
Currently requires files on disk. Future versions may support:
- stdin input
- URL loading (HTTP/S3)

## Configuration

### Future Configuration Options
Planned configuration file (~/.config/rata-data-viewer/config.toml):
- Custom keybindings
- Color schemes
- Default page size
- Filter options

Currently, these are hardcoded but easy to modify in source code:
- Page size: `app.rs:16` - `page_size: 20`
- Max cell display: `ui.rs:100` - `if cell.len() > 50`
- Visible columns: `ui.rs:88` - `.take(10)`
