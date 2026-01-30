# Rata Data Viewer

A fast, terminal-based data viewer for Parquet and CSV files built with Rust and Ratatui.

## Features

- 📊 **Multiple Formats**: View Parquet and CSV files
- 🔍 **Advanced Filtering**: Powerful filter expressions with comparison operators (>, <, >=, <=, =, !=) and logical operators (AND, OR, NOT)
- 🎯 **Column Selection**: Choose which columns to display, hide unwanted columns
- 📝 **Smart Input**: Cursor movement in filter input for easy editing
- 🔢 **Line Numbers**: Toggleable line numbers for easy row reference
- ⌨️  **Vim-style Navigation**: Navigate with hjkl or arrow keys
- 📄 **Pagination**: Efficient handling of large datasets
- 🎨 **Clean TUI**: Intuitive terminal interface
- ⚡ **Fast Performance**: Powered by Polars DataFrame engine

## Quick Start

```bash
# Build and run
cargo build --release

# View a Parquet file
./target/release/rata-data-viewer data.parquet

# View a CSV file
./target/release/rata-data-viewer data.csv
```

## Keyboard Controls

### Navigation
- `↑/↓` or `j/k` - Scroll up/down one row
- `←/→` or `h/l` - Scroll left/right (between columns)
- `PgUp/PgDn` - Page up/down
- `Home/End` or `g/G` - Go to top/bottom

### Filtering
- `/` - Enter filter mode
- Type your filter expression (see Filter Syntax below)
- `Enter` - Apply filter
- `Esc` - Clear filter / Exit filter mode

**In Filter Input:**
- `←/→` - Move cursor left/right
- `Home/End` - Jump to start/end
- `Backspace` - Delete before cursor
- `Delete` - Delete at cursor

### Column Selection
- `c` - Enter column selection mode
- `↑/↓` or `j/k` - Navigate columns
- `Space` - Toggle column visibility
- `a` - Select all columns
- `Enter` - Apply selection
- `Esc` - Cancel

### Display
- `n` - Toggle line numbers on/off

### Application
- `q` or `Ctrl+C` - Quit

## Filter Syntax

### Comparison Operators

```bash
# Numeric comparisons
LastPrice > 5000
Volume >= 1000
LastPrice < 6000
Volume <= 10000

# Exact match
InstrumentID = IC2602

# Not equal
InstrumentID != IC2601

# Contains (substring match)
InstrumentID:IC2602
```

### Logical Operators

```bash
# AND - both conditions must be true
InstrumentID = IC2602 AND LastPrice > 5000

# OR - at least one condition must be true
InstrumentID = IC2602 OR InstrumentID = IC2603

# NOT - negate a condition
NOT InstrumentID = IC2602

# Complex expressions
InstrumentID:IC26 AND LastPrice > 5000 AND Volume > 1000
```

### String Comparison

Works for time/date strings in sortable format:

```bash
# Time comparisons
UpdateTime > "09:30:00"
UpdateTime >= "09:30:00" AND UpdateTime < "16:00:00"

# Date comparisons
TradingDay > "20240101"
TradingDay >= "20240101" AND TradingDay <= "20240131"
```

### Filter Examples

```bash
# Find high-value trades
LastPrice > 5000 AND Volume > 1000

# Specific instrument in time window
InstrumentID = IC2602 AND UpdateTime > "09:30:00"

# Multiple instruments with price filter
InstrumentID = IC2602 OR InstrumentID = IC2603 AND LastPrice > 5000

# Morning trades only
UpdateTime < "12:00:00"
```

## Column Selection

Choose which columns to display:

1. Press `c` to enter column selection mode
2. Use `↑/↓` to navigate through columns
3. Press `Space` to toggle a column on/off
   - `[✓]` = Column is visible
   - `[ ]` = Column is hidden
4. Press `Enter` to apply your selection
5. Press `a` to reset and show all columns

**Example**: Show only InstrumentID, LastPrice, and Volume
- Press `c`
- Uncheck all except the columns you want
- Press `Enter`

## Documentation

Comprehensive documentation in the `docs/` directory:
- [Architecture](docs/architecture.md) - System design and component overview
- [Getting Started](docs/getting_started.md) - Installation and usage guide
- [Filter Syntax](docs/filter_syntax.md) - Complete filter syntax reference
- [CSV Support](docs/csv_support.md) - CSV-specific features and tips
- [File Formats](docs/file_formats.md) - Parquet vs CSV comparison
- [Quick Reference](docs/quick_reference.md) - Keyboard shortcuts cheat sheet

## Supported Formats

### Parquet (.parquet)
- ✅ **Best for**: Large datasets (100MB+)
- ✅ **Performance**: Fastest loading and filtering
- ✅ **Features**: Compressed, columnar storage, type preservation

### CSV (.csv)
- ✅ **Best for**: Small to medium datasets (<500MB)
- ✅ **Features**: Human-readable, automatic delimiter detection, wide compatibility
- ✅ **Note**: All viewer features work identically to Parquet

## Examples

### Basic Usage

```bash
# View Parquet file
./rata-data-viewer market_data.parquet

# View CSV file
./rata-data-viewer sales_data.csv
```

### Advanced Filtering

```bash
# Start viewer
./rata-data-viewer data.parquet

# In viewer:
# 1. Press / to enter filter mode
# 2. Type: LastPrice > 5000 AND Volume > 1000
# 3. Press Enter to see filtered results
```

### Column Selection Workflow

```bash
# Start viewer
./rata-data-viewer data.csv

# In viewer:
# 1. Press c to select columns
# 2. Uncheck unwanted columns with Space
# 3. Press Enter to apply
# 4. Now see only selected columns
```

### Combined Features

```bash
# Start viewer
./rata-data-viewer market_data.parquet

# Workflow:
# 1. Press c → Select only: InstrumentID, LastPrice, Volume
# 2. Press Enter to apply
# 3. Press / → Type: LastPrice > 5000
# 4. Press Enter → Filtered rows with selected columns
# 5. Press n → Toggle line numbers
```

## Requirements

- Rust 1.70+
- Modern terminal with color support

## Building

```bash
# Release build (optimized)
cargo build --release

# Binary location
./target/release/rata-data-viewer

# Run directly with cargo
cargo run --release -- your-file.parquet
```

## Performance

### Loading Times (approximate)

**Parquet:**
- 10MB: 0.5-1 second
- 100MB: 2-5 seconds
- 1GB: 10-30 seconds

**CSV:**
- 10MB: 1 second
- 100MB: 5-10 seconds
- 500MB: 30-60 seconds

**Tip**: For large CSV files (>100MB), convert to Parquet for faster loading:
```python
import polars as pl
pl.read_csv("data.csv").write_parquet("data.parquet")
```

## Tips & Tricks

### Fast Navigation
- Use `g/G` to jump to top/bottom instantly
- Use `PgUp/PgDn` for quick page browsing
- Use `←/→` to scan through many columns

### Efficient Filtering
1. Start with broad filter: `InstrumentID:IC26`
2. Refine with more conditions: `InstrumentID:IC26 AND LastPrice > 5000`
3. Use `Esc` to clear and try different filters

### Column Management
- Hide columns you don't need with `c` key
- Press `a` in column selection to reset to all columns
- Column selection persists until you change it

### Filter Input Editing
- Use `Home/End` to quickly jump to start/end of filter
- Use `←/→` to position cursor for editing
- No need to delete everything to fix a typo!

## Troubleshooting

### Terminal garbled after crash
```bash
reset
```

### File not loading
- Check file extension (.parquet or .csv)
- Verify file exists and is not corrupted
- Try with a smaller test file first

### Filter syntax errors
- Check operator spacing: `Price > 5000` (with spaces)
- Verify column names match exactly (case-sensitive)
- Use quotes for string values with spaces

---

Built with ❤️ using Rust, Ratatui, and Polars
