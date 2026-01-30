# Quick Reference Guide

## Starting the Viewer

```bash
./target/release/rata-data-viewer <file.parquet|file.csv|file.db>
```

## Keyboard Shortcuts

### Navigation
| Key | Action |
|-----|--------|
| `↑` or `k` | Scroll up one row |
| `↓` or `j` | Scroll down one row |
| `←` or `h` | Scroll left (previous columns) |
| `→` or `l` | Scroll right (next columns) |
| `PgUp` | Scroll up one page (20 rows) |
| `PgDn` | Scroll down one page (20 rows) |
| `Home` or `g` | Jump to top |
| `End` or `G` | Jump to bottom |

### Filtering
| Key | Action |
|-----|--------|
| `/` | Enter filter mode |
| `Enter` | Apply filter (in filter mode) |
| `Esc` | Clear active filter OR cancel filter input |

### Display
| Key | Action |
|-----|--------|
| `n` or `N` | Toggle line numbers on/off |

### Application
| Key | Action |
|-----|--------|
| `q` or `Q` | Quit application |
| `Ctrl+C` | Force quit |

## Filter Syntax

### Column-Specific
```
ColumnName:value          # Search in specific column (colon)
ColumnName=value          # Search in specific column (equals)
```

### Global Search
```
value                     # Search all columns
```

### Examples
```
InstrumentID:IC2602       # Only InstrumentID column
UpdateTime=09:30:00       # Only UpdateTime column
Price:50                  # Only Price column
IC2602                    # All columns
```

## UI Layout

```
┌─────────────────────────────────────────────────────┐
│ File path and keyboard shortcuts                    │ Header (3 lines)
├─────────────────────────────────────────────────────┤
│ # | Col1    | Col2    | Col3    | ...               │
│ 1 | value   | value   | value   | ...               │
│ 2 | value   | value   | value   | ...               │ Table (flexible)
│ ...                                                  │
├─────────────────────────────────────────────────────┤
│ Filter input OR Active filter OR Info               │ Footer (3 lines)
├─────────────────────────────────────────────────────┤
│ Status: Ready / ERROR: message                      │ Status (1 line)
└─────────────────────────────────────────────────────┘
```

## Common Workflows

### Basic Data Exploration
1. Open file: `./rata-data-viewer data.parquet`
2. Scroll through data with `↓` and `↑`
3. See more columns with `→`
4. Jump to end with `G`, back to start with `g`
5. Quit with `q`

### Finding Specific Records
1. Press `/` to enter filter mode
2. Type column and value: `InstrumentID:IC2602`
3. Press `Enter` to apply
4. Browse filtered results
5. Press `Esc` to clear filter and see all data again

### Checking Row Numbers
1. Ensure line numbers are visible (press `n` if not)
2. Note the row number of interest
3. Can reference this in discussions or notes

### Viewing Large Files
1. Application shows "Loading..." while file loads
2. Once loaded, all operations are instant
3. Use filters to narrow down large datasets
4. Page navigation (`PgUp`/`PgDn`) for efficiency

## Tips & Tricks

### Efficient Column Navigation
- Most data has many columns but you need specific ones
- Use `→` repeatedly or note column position for quick access
- Currently shows 10 columns at a time

### Filter Strategy
1. **Start broad**: Use global search first (`IC2602`)
2. **Refine**: If too many results, use column-specific (`InstrumentID:IC2602`)
3. **Clear and retry**: Use `Esc` to clear and try different filter

### Performance
- **Parquet files** load fastest (optimized format)
- **CSV files** take longer (needs parsing)
- **Filtering** is instant after loading (in-memory operations)
- File size affects loading time, not filtering/scrolling time

### Troubleshooting

**Terminal garbled after crash**:
```bash
reset
```

**Column name not recognized**:
- Check exact spelling (case-sensitive)
- Verify column exists by scrolling through data first
- If unsure, use global search instead

**Filter shows no results**:
- Check filter syntax
- Try global search to verify data contains value
- Remember: case-sensitive matching

**Performance issues**:
- Very large files (>1GB) may take time to load
- Consider pre-filtering with tools like `duckdb`
- Once loaded, operations are fast

## Advanced Usage

### Comparing Different Filters
1. Note current row count from footer
2. Apply filter 1, note results
3. Press `Esc` to clear
4. Apply filter 2, compare results

### Exporting Filtered Results
Currently not supported directly. Workaround:
1. Use external tool (pandas, polars, duckdb)
2. Apply same filter logic
3. Export to new file
4. View in rata-data-viewer

### Viewing Multiple Files
- Open separate terminal windows
- Run viewer in each
- Compare side-by-side

## Keyboard Cheat Sheet (Print-Friendly)

```
╔════════════════════════════════════════════════════╗
║          RATA DATA VIEWER - QUICK KEYS             ║
╠════════════════════════════════════════════════════╣
║ NAVIGATE                                           ║
║   ↑↓←→ / hjkl      Scroll                          ║
║   PgUp/PgDn        Page up/down                    ║
║   Home/End or g/G  Top/Bottom                      ║
║                                                    ║
║ FILTER                                             ║
║   /                Enter filter mode               ║
║   Enter            Apply filter                    ║
║   Esc              Clear filter                    ║
║                                                    ║
║ DISPLAY                                            ║
║   n                Toggle line numbers             ║
║                                                    ║
║ EXIT                                               ║
║   q                Quit                            ║
║   Ctrl+C           Force quit                      ║
╠════════════════════════════════════════════════════╣
║ FILTER EXAMPLES                                    ║
║   InstrumentID:IC2602      Column-specific         ║
║   UpdateTime=09:30:00      Column-specific         ║
║   IC2602                   All columns             ║
╚════════════════════════════════════════════════════╝
```
