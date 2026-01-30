# Rata Data Viewer - Architecture Documentation

## Overview

Rata Data Viewer is a terminal-based data viewer built with Ratatui (Rust TUI framework). It supports viewing and filtering data from multiple file formats including Parquet, CSV, and SQLite databases.

## Core Architecture

### Module Structure

```
src/
├── main.rs           # Entry point, terminal setup, event loop
├── app.rs            # Application state and business logic
├── ui.rs             # UI rendering logic
└── data/
    ├── mod.rs        # Data module exports
    ├── source.rs     # DataSource trait and implementations
    └── table.rs      # TableData structure for UI display
```

### Component Responsibilities

#### 1. Main (`main.rs`)
- **Purpose**: Application entry point
- **Responsibilities**:
  - Command-line argument parsing
  - Terminal initialization and cleanup
  - Main event loop
  - Keyboard event handling
  - Terminal state management (raw mode, alternate screen)

#### 2. App (`app.rs`)
- **Purpose**: Application state management
- **Responsibilities**:
  - Holds data source and filtered dataframe
  - Manages scroll positions (vertical and horizontal)
  - Handles filter state and pattern
  - Implements navigation logic (scroll up/down, page navigation)
  - Applies filters to data
  - Provides current page data to UI

**Key State Variables**:
- `data_source`: Original data loaded from file
- `filtered_df`: Currently filtered view of data
- `scroll_offset`: Current row position in the dataset
- `column_offset`: Current column position for horizontal scrolling
- `page_size`: Number of rows to display per page
- `filter_pattern`: Current search/filter text
- `filter_mode`: Whether user is currently typing a filter

#### 3. UI (`ui.rs`)
- **Purpose**: Rendering the terminal UI
- **Responsibilities**:
  - Layout management (header, table, footer, status bar)
  - Table rendering with pagination
  - Filter input display
  - Status and error message display
  - Column truncation for display

**Layout Structure**:
```
┌─────────────────────────────────────┐
│  Header (Help text, file path)     │ 3 lines
├─────────────────────────────────────┤
│                                     │
│  Table (Data with headers)          │ Flexible
│                                     │
├─────────────────────────────────────┤
│  Footer (Filter/Info)               │ 3 lines
├─────────────────────────────────────┤
│  Status Bar (Errors/Ready)          │ 1 line
└─────────────────────────────────────┘
```

#### 4. Data Module (`data/`)

##### DataSource (`source.rs`)
- **Purpose**: Abstract data loading from different file formats
- **Responsibilities**:
  - Auto-detect file type from extension
  - Load data from Parquet, CSV, or SQLite
  - Provide filtering capabilities
  - Expose dataframe metadata (columns, row count)

**Supported Formats**:
- `.parquet` - Apache Parquet columnar format
- `.csv` - Comma-separated values
- `.db`, `.sqlite`, `.sqlite3` - SQLite databases (limited support)

##### TableData (`table.rs`)
- **Purpose**: Convert DataFrame to UI-friendly structure
- **Responsibilities**:
  - Extract paginated data from DataFrame
  - Convert all values to strings for display
  - Provide metadata (total rows, columns)

## Data Flow

```
File on Disk
    ↓
DataSource::load()
    ↓
DataFrame (Polars)
    ↓
[User applies filter]
    ↓
Filtered DataFrame
    ↓
TableData::from_dataframe()
    ↓
UI Rendering
```

## Key Technologies

### 1. Ratatui
- Terminal UI framework for Rust
- Provides widgets (Table, Paragraph, Block, etc.)
- Layout management
- Immediate mode rendering

### 2. Crossterm
- Cross-platform terminal manipulation
- Handles raw mode, keyboard events
- Terminal state management

### 3. Polars
- High-performance DataFrame library (similar to pandas)
- Lazy evaluation for efficient data processing
- Native support for Parquet and CSV
- Built-in filtering and querying

### 4. Anyhow
- Error handling library
- Provides context for errors
- Simplifies error propagation

## State Management

The application follows a simple state management pattern:

1. **Immutable Data Source**: Original data is loaded once and never modified
2. **Derived State**: Filtered dataframe is derived from applying filter to original data
3. **UI State**: Scroll positions, filter mode, etc. are maintained separately
4. **Unidirectional Flow**: Events → State Updates → UI Re-render

## Performance Considerations

### 1. Lazy Loading
- Polars uses lazy evaluation where possible
- Only the current page of data is converted to strings
- Filtering is done at DataFrame level (optimized)

### 2. Memory Management
- Full dataset is loaded into memory (limitation for very large files)
- Future improvement: Stream processing for massive files

### 3. Rendering Optimization
- Only visible columns/rows are rendered
- Cell content is truncated to avoid terminal overflow
- Immediate mode rendering (only draws what changed)

## Extensibility Points

### Adding New Data Sources
To add support for a new file format:
1. Add variant to `DataSourceType` enum
2. Implement loading logic in `DataSource::load()`
3. Update file extension detection in `DataSourceType::from_path()`

### Adding New Features
Common extensions:
- **Column-specific filtering**: Modify `App` to track selected column
- **Sorting**: Add sort state and apply to DataFrame
- **Export filtered data**: Add export functionality
- **Multiple table selection** (for SQLite): Add table picker UI
- **Regex filtering**: Use regex crate in filter logic

### UI Customization
- Modify `ui.rs` to change layout, colors, or styling
- All UI rendering is centralized in `ui::render()`
- Use Ratatui's styling system for themes

## Error Handling

The application uses `anyhow::Result` throughout for error handling:

1. **Load Errors**: File not found, unsupported format, parse errors
2. **Filter Errors**: Invalid filter expressions (displayed in status bar)
3. **Terminal Errors**: Terminal state issues (causes app exit)

Errors are either:
- **Fatal**: Displayed and cause application exit (file loading)
- **Non-fatal**: Displayed in status bar, user can continue (filter errors)

## Future Improvements

Potential enhancements:
1. **Streaming support** for very large files
2. **SQLite table selection** UI
3. **Column sorting** (click or keyboard shortcut)
4. **Export filtered results** to CSV/Parquet
5. **Column width customization**
6. **Search highlighting**
7. **Regex and advanced filtering** (numeric ranges, date filtering)
8. **Configuration file** for keybindings and colors
9. **Multiple file comparison** view
10. **JSON/XML support**
