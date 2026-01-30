# Changelog - User Feedback Updates

## Version 0.2.0 - Improvements Based on User Feedback

### 1. Loading Indicator ✅
**Problem**: Application showed dark screen during loading, especially concerning for large files.

**Solution**: Added loading screen that displays immediately when application starts:
- Shows "Loading..." message with file path
- Displayed in yellow/bold for visibility
- Appears before data is loaded into memory

**Implementation**: `src/main.rs:34-55`

---

### 2. Column-Specific Filtering ✅
**Problem**: Filter function was unclear - searching all columns wasn't intuitive for structured data.

**Solution**: Enhanced filter syntax to support column-specific filtering:

**Supported Formats**:
- `ColumnName:value` - Filter specific column (colon separator)
- `ColumnName=value` - Filter specific column (equals separator)
- `value` - Search all columns (original behavior)

**Examples**:
```
InstrumentID:IC2602        → Shows only rows where InstrumentID contains "IC2602"
UpdateTime=09:30:00        → Shows only rows where UpdateTime contains "09:30:00"
IC2602                     → Searches all columns for "IC2602"
```

**Features**:
- Validates column names against actual columns in data
- Falls back to all-column search if column name invalid
- Shows active filter with column name in footer
- Helpful examples displayed in filter input mode

**Implementation**:
- Filter parsing: `src/app.rs:144-172`
- UI updates: `src/ui.rs:129-163`

---

### 3. Line Numbers ✅
**Problem**: No way to reference specific rows or track position in data.

**Solution**: Added toggleable line numbers:

**Features**:
- Displayed in leftmost column with "#" header
- Shows actual row numbers (1-indexed)
- Gray color to distinguish from data
- Toggle on/off with `n` or `N` key
- Default: ON

**Implementation**:
- State management: `src/app.rs:30` (show_line_numbers field)
- Toggle logic: `src/app.rs:210-212`
- UI rendering: `src/ui.rs:55-127`

---

### 4. Search Function (Future)
**Status**: Noted for future implementation

**Difference from Filter**:
- **Filter**: Hides non-matching rows (current implementation)
- **Search**: Would highlight matches while showing all rows

**Planned Features**:
- Incremental search (find-as-you-type)
- "Find next" / "Find previous" navigation
- Highlight matching text in different color
- Show match count (e.g., "3/15 matches")

**Suggested Implementation**:
- Add search mode separate from filter mode
- Use `/` for filter, `?` or `f` for search
- Store search pattern and current match index
- Render with highlighted cells for matches

---

## UI/UX Improvements

### Updated Keyboard Shortcuts Display
**Old**:
```
Press q to quit | / to filter | ↑↓←→ to navigate | PgUp/PgDn page | Home/End top/bottom
```

**New**:
```
/ filter | n toggle line# | ↑↓←→ navigate | PgUp/PgDn page | q quit
```

More concise, highlights new line number toggle feature.

### Enhanced Filter Input Experience
**Old**:
```
Filter (press Enter to apply, Esc to cancel)
Filter: InstrumentID = "IC2602"_
```

**New**:
```
Filter: ColumnName:value or ColumnName=value or just value (Enter: apply, Esc: cancel)
IC2602_
Examples: InstrumentID:IC2602 | UpdateTime=09:30:00 | IC2602
```

Shows syntax help and examples directly in UI.

### Active Filter Display
**Old**:
```
Filter: InstrumentID:IC2602
```

**New**:
```
Column 'InstrumentID': IC2602    (for column-specific)
All columns: IC2602              (for global search)
```

Clearer indication of filter scope.

---

## Performance Considerations

### Loading Time
The initial loading time depends on:
1. **File format**: Parquet is fastest (columnar), CSV is slower (row-based parsing)
2. **File size**: Linear relationship with size
3. **Compression**: Compressed Parquet reads less data but needs decompression

**Benchmarks** (approximate):
- 10MB Parquet: 0.5-1 second
- 100MB Parquet: 2-5 seconds
- 1GB Parquet: 10-30 seconds
- CSV is typically 2-3x slower than Parquet

**Future Optimization**:
- Stream data instead of loading all into memory
- Load first page immediately, load rest in background
- Progress bar showing percentage loaded

---

## Testing Notes

### Testing Column-Specific Filtering

1. **Valid column name**:
   ```
   Press /
   Type: InstrumentID:IC2602
   Press Enter
   → Should show only rows with IC2602 in InstrumentID column
   → Footer shows: "Column 'InstrumentID': IC2602"
   ```

2. **Invalid column name**:
   ```
   Press /
   Type: NonExistentColumn:value
   Press Enter
   → Falls back to searching all columns for "NonExistentColumn:value"
   → Footer shows: "All columns: NonExistentColumn:value"
   ```

3. **Global search**:
   ```
   Press /
   Type: IC2602
   Press Enter
   → Searches all columns
   → Footer shows: "All columns: IC2602"
   ```

### Testing Line Numbers

1. **Toggle line numbers**:
   ```
   Press n
   → Line numbers disappear
   Press n again
   → Line numbers reappear
   ```

2. **Verify line numbers**:
   ```
   Scroll to different positions
   → Line numbers should match actual row position in filtered data
   → First visible row shows scroll_offset + 1
   ```

---

## Code Quality Notes

### Maintainability
- All filter logic centralized in `App::apply_filter()` and `App::parse_filter_pattern()`
- UI rendering separated from business logic
- Easy to extend with new filter operators (regex, numeric ranges, etc.)

### Extensibility Points

**Adding new filter operators**:
Edit `App::parse_filter_pattern()` to recognize new syntax:
```rust
// Add support for "Column>value" (greater than)
if let Some(gt_pos) = pattern.find('>') {
    // ... parse and validate
}
```

**Adding regex filtering**:
Update `DataSource::filter()` to accept filter mode parameter:
```rust
pub fn filter(&self, pattern: &str, column: Option<&str>, mode: FilterMode) -> Result<DataFrame>
```

---

## Documentation Updates

Updated files:
- `docs/getting_started.md` - Add column-specific filter examples
- `docs/architecture.md` - Document new filter parsing logic
- `docs/changelog.md` - This file

Recommended additions:
- `docs/filter_syntax.md` - Complete filter syntax reference
- `docs/performance.md` - Loading time benchmarks and optimization tips
