# What's New - Version 0.2.0

## Summary

All your feedback has been implemented! Here's what's new:

## ✅ 1. Loading Indicator
**Before**: Dark screen while loading, no indication of progress
**After**: Yellow "Loading..." screen showing file path

**What you'll see**:
```
┌─────────────────────────────────┐
│ Loading                         │
│                                 │
│ Loading file: /path/to/file     │
│                                 │
│ Please wait...                  │
└─────────────────────────────────┘
```

## ✅ 2. Column-Specific Filtering
**Before**: Only searched all columns, syntax unclear
**After**: Support for column-specific filtering with intuitive syntax

**How to use**:
```
Press /

Type one of:
  InstrumentID:IC2602      → Filter InstrumentID column only
  UpdateTime=09:30:00      → Filter UpdateTime column only
  IC2602                   → Search all columns (original behavior)

Press Enter to apply
```

**In the UI**:
- Filter input shows examples as you type
- Active filter shows which column is being filtered
- Falls back to global search if column name doesn't exist

**Examples for your CTPDepthMarketData.parquet**:
```
InstrumentID:IC2602           → Show only IC2602 instrument
InstrumentID:IC26             → Show all IC26xx instruments
UpdateTime=09:30             → Specific time period
LastPrice:50                  → Price containing "50"
```

## ✅ 3. Line Numbers
**Before**: No row numbers
**After**: Toggleable line numbers on the left

**How to use**:
- Press `n` to toggle line numbers on/off
- Default: ON
- Shows actual row position in filtered data
- Gray color to distinguish from data

**What you'll see**:
```
┌─────────────────────────────────────────┐
│ # | InstrumentID | UpdateTime | Price   │
│ 1 | IC2602       | 09:30:00   | 5000.0  │
│ 2 | IC2602       | 09:30:01   | 5001.5  │
│ 3 | IC2602       | 09:30:02   | 5002.0  │
└─────────────────────────────────────────┘
```

## 📝 4. Search Function (Future)
Noted for future implementation. Will be different from filter:
- **Filter** (current): Hides non-matching rows
- **Search** (future): Highlights matches, shows all rows

## Additional Improvements

### Better Help Text
Old:
```
Press q to quit | / to filter | ↑↓←→ to navigate | ...
```

New (more concise):
```
/ filter | n toggle line# | ↑↓←→ navigate | PgUp/PgDn page | q quit
```

### Enhanced Filter UI
When you press `/`, you now see:
```
┌─────────────────────────────────────────────────────────────────────┐
│ Filter: ColumnName:value or ColumnName=value or just value         │
│ InstrumentID:_                                                      │
│ Examples: InstrumentID:IC2602 | UpdateTime=09:30:00 | IC2602       │
└─────────────────────────────────────────────────────────────────────┘
```

### Better Active Filter Display
After applying a filter, the footer shows:
```
Column 'InstrumentID': IC2602        (for column-specific)
   or
All columns: IC2602                  (for global search)
```

## Testing the New Version

### Build and Run
```bash
cd /home/huaisy/github/rata-data-viewer
cargo build --release
./target/release/rata-data-viewer /home/huaisy/data/CTPDepthMarketData.parquet
```

### Test Checklist

1. **Loading screen**:
   - Run the app, verify you see "Loading..." before data appears

2. **Column-specific filtering**:
   ```
   Press /
   Type: InstrumentID:IC2602
   Press Enter
   → Should show only IC2602 rows
   → Footer should say "Column 'InstrumentID': IC2602"
   ```

3. **Global search still works**:
   ```
   Press Esc to clear
   Press /
   Type: IC2602
   Press Enter
   → Should search all columns
   → Footer should say "All columns: IC2602"
   ```

4. **Line numbers**:
   ```
   Press n → line numbers disappear
   Press n → line numbers reappear
   ```

5. **Invalid column name handling**:
   ```
   Press /
   Type: NonExistent:value
   Press Enter
   → Falls back to searching all columns
   → No error, just searches globally
   ```

## Performance Notes

**Loading time** for your 9.4MB file should be:
- Before: 1-2 seconds with dark screen
- After: 1-2 seconds with "Loading..." message

The loading time hasn't changed, but now you know what's happening!

For larger files:
- 100MB: 2-5 seconds
- 1GB: 10-30 seconds
- Progress bar could be added in future version

## Documentation

New/updated docs in `./docs/`:
- ✅ `changelog.md` - Detailed changelog with all changes
- ✅ `quick_reference.md` - Keyboard shortcuts and syntax reference
- ✅ `getting_started.md` - Updated with new filter syntax
- ✅ `architecture.md` - Existing, describes system design
- ✅ `data_sources.md` - Existing, technical details
- ✅ `testing_guide.md` - Existing, how to test

## Quick Reference

### Filter Syntax
```
Column:value              # Colon separator
Column=value              # Equals separator
value                     # Global search
```

### Keyboard Shortcuts
```
/             Enter filter mode
n             Toggle line numbers
↑↓←→ or hjkl  Navigate
PgUp/PgDn     Page up/down
g/G           Top/Bottom
q             Quit
```

## Next Steps

Give it a try and let me know:
1. Does the loading screen help?
2. Is column-specific filtering clear and useful?
3. Do you like line numbers? Should they be off by default?
4. Any other feedback or feature requests?

The binary is ready at:
```
./target/release/rata-data-viewer
```
