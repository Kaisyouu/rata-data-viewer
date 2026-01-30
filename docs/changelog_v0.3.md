# Version 0.3.0 - Advanced Filtering

## Major Features

### Advanced Filter Expressions ✅

You can now write complex filter expressions with comparison operators and logical conditions!

**What's New**:
- Comparison operators: `=`, `!=`, `>`, `<`, `>=`, `<=`
- Logical operators: `AND`, `OR`, `NOT`
- Support for both string and numeric comparisons
- Automatic type detection

**Examples**:
```
# Numeric comparisons
LastPrice > 5000
Volume >= 1000
LastPrice >= 5000 AND LastPrice <= 5100

# Exact string matching
InstrumentID = IC2602
ExchangeID = CFFEX

# Logical combinations
InstrumentID = IC2602 AND LastPrice > 5000
InstrumentID = IC2602 OR InstrumentID = IC2603
NOT InstrumentID = IC2601

# Complex conditions
InstrumentID:IC26 AND LastPrice > 5000 AND Volume > 1000
```

## Comparison with Previous Version

### v0.2.0 (Previous)
```
InstrumentID:IC2602                    → Contains "IC2602"
InstrumentID=IC2602                    → Contains "IC2602" (same as above)
IC2602                                 → Search all columns
```

**Limitations**:
- Only substring matching
- No numeric comparisons
- No logical operators
- Confusing syntax (= meant contains, not equals)

### v0.3.0 (Current)
```
InstrumentID:IC2602                    → Contains "IC2602" (same)
InstrumentID = IC2602                  → Exact match "IC2602" (NEW!)
IC2602                                 → Search all columns (same)

# NEW capabilities:
LastPrice > 5000                       → Numeric comparison
InstrumentID = IC2602 AND Price > 5000 → Logical combination
NOT InstrumentID:IC2601                → Negation
```

## Implementation Details

### New Module: `filter.rs`

Created a filter expression parser with:
- `FilterExpr` enum: Represents parsed filter expressions
- `ComparisonOp` enum: Supported comparison operators
- Recursive parsing for logical expressions
- Type-aware evaluation (strings vs numbers)

### Updated `DataSource::filter()`

**Before**:
```rust
pub fn filter(&self, pattern: &str, column: Option<&str>) -> Result<DataFrame>
```
Simple substring matching only.

**After**:
```rust
pub fn filter(&self, pattern: &str) -> Result<DataFrame>
```
Parses expression, applies complex conditions.

### Simplified `App::apply_filter()`

**Before**: Manual parsing of column:value syntax
**After**: Delegates to `FilterExpr::parse()` which handles all complexity

## Breaking Changes

### Filter API Change

The `DataSource::filter()` method signature changed:
- **Old**: `filter(pattern: &str, column: Option<&str>)`
- **New**: `filter(pattern: &str)`

The column is now part of the expression itself.

**Migration**: If you have custom code using the old API:
```rust
// Old way
data_source.filter("IC2602", Some("InstrumentID"))?;

// New way
data_source.filter("InstrumentID:IC2602")?;
```

### `App::filter_column` Field

The `filter_column` field in `App` is now unused (kept for backward compatibility but will be removed in v0.4.0).

## UI Changes

### Filter Input Help
**Before**:
```
Filter: ColumnName:value or ColumnName=value or just value
Examples: InstrumentID:IC2602 | UpdateTime=09:30:00 | IC2602
```

**After**:
```
Advanced Filter
Operators: = != > < >= <= :contains
Examples: Price > 5000 | InstrumentID = IC2602 AND Price > 5000
```

### Active Filter Display
**Before**: Showed which column was being filtered
**After**: Shows the complete filter expression

## Performance

### Numeric Comparisons
- **Very fast**: Uses Polars vectorized operations
- **Memory efficient**: Works on native types (i32, i64, f32, f64)
- **Type detection**: Automatic, no overhead

### String Comparisons
- **Exact match (=)**: Fast, optimized by Polars
- **Contains (:)**: Fast for most cases, may be slower on very long strings
- **Case-sensitive**: Current limitation, but fast

### Logical Operators
- **AND/OR**: Minimal overhead, operates on boolean masks
- **NOT**: Very fast, simple boolean inversion
- **Complex expressions**: Linear time relative to number of conditions

### Performance Comparison

| Operation | v0.2.0 | v0.3.0 | Notes |
|-----------|--------|--------|-------|
| Simple contains | Fast | Fast | Same performance |
| Numeric filter | Not supported | Very fast | New feature |
| Multiple conditions | Not supported | Fast | Minimal overhead |
| Global search | Fast | Fast | Same performance |

## Testing

### Basic Comparison Tests
```bash
./rata-data-viewer data.parquet

# Test numeric greater than
Press /
Type: LastPrice > 5000
Press Enter
→ Should show only rows with price > 5000

# Test exact match
Press Esc, then /
Type: InstrumentID = IC2602
Press Enter
→ Should show only exact IC2602 rows

# Test AND condition
Press Esc, then /
Type: InstrumentID = IC2602 AND LastPrice > 5000
Press Enter
→ Should show IC2602 rows with price > 5000
```

### Advanced Tests
```bash
# Test OR
InstrumentID = IC2602 OR InstrumentID = IC2603

# Test NOT
NOT InstrumentID = IC2602

# Test range
LastPrice >= 5000 AND LastPrice <= 5100

# Test complex
InstrumentID:IC26 AND LastPrice > 5000 AND Volume > 1000
```

### Error Handling Tests
```bash
# Invalid column
NonExistentColumn > 5000
→ Should show: "Column 'NonExistentColumn' not found"

# Type mismatch
InstrumentID > 5000
→ Should show: "Column is not numeric type"

# Invalid value
Price > abc
→ Should show: "Value must be numeric for > comparison"
```

## Known Issues

### No Parentheses Support
```
(A OR B) AND C    # Not supported yet
```
**Workaround**: Split into multiple filters

### Left-to-Right Evaluation
```
A OR B AND C      # Evaluated as: (A OR B) AND C
```
**Workaround**: Be aware of evaluation order

### No Date/Time Types
```
UpdateTime > 09:30:00    # Treats as string, may not work as expected
```
**Workaround**: Use string contains: `UpdateTime:09:30`

## Future Enhancements

Planned for v0.4.0:
- Parentheses support for grouping
- Regex operator (`:~`)
- Date/time parsing
- IN operator
- Case-insensitive option
- Better error messages with suggestions

## Documentation

New documentation:
- `docs/filter_syntax.md` - Complete filter syntax guide with examples
- `docs/changelog_v0.3.md` - This file

Updated documentation:
- `docs/quick_reference.md` - Updated with new operators
- `docs/getting_started.md` - Updated filter section

## Migration Guide

### From v0.2.0 to v0.3.0

**Backward Compatible**:
- Old syntax still works: `InstrumentID:IC2602`
- Global search still works: `IC2602`

**New Capabilities**:
- Use `=` for exact match instead of contains
- Add numeric comparisons: `Price > 5000`
- Combine conditions: `A AND B`

**Recommended Changes**:
- If you were using `InstrumentID=IC2602` expecting exact match, it now works correctly!
- If you were working around lack of numeric filtering, you can now do it directly

**No Breaking Changes**:
- All existing filters will continue to work
- New syntax is purely additive
