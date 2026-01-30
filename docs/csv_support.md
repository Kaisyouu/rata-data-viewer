# CSV Support - Complete Guide

## Overview

CSV (Comma-Separated Values) files are **fully supported** in Rata Data Viewer with all the same features as Parquet files.

## Features

✅ **Automatic loading**: Just pass the .csv file path
✅ **Automatic delimiter detection**: Handles comma, tab, semicolon, etc.
✅ **Header detection**: Automatically identifies header row
✅ **Type inference**: Automatically detects column types (string, numeric, etc.)
✅ **All viewer features**: Filtering, column selection, line numbers, etc.

## Usage

### Basic Usage

```bash
./rata-data-viewer data.csv
```

That's it! The viewer will:
1. Detect it's a CSV file (by .csv extension)
2. Load and parse the data
3. Display in the TUI with all features enabled

### Example

```bash
# Create test CSV
cat > test_market_data.csv << 'EOF'
InstrumentID,LastPrice,Volume,UpdateTime,ExchangeID
IC2602,5100.5,1200,09:30:15,CFFEX
IC2602,5105.0,800,09:30:30,CFFEX
IC2603,5200.0,1500,09:30:15,CFFEX
EOF

# View it
./rata-data-viewer test_market_data.csv
```

## CSV Format Support

### Delimiters

Automatically detects and handles:
- **Comma**: `value1,value2,value3`
- **Tab**: `value1	value2	value3`
- **Semicolon**: `value1;value2;value3`
- **Pipe**: `value1|value2|value3`

### Headers

- **With headers** (recommended):
  ```csv
  Name,Age,City
  Alice,30,NYC
  Bob,25,LA
  ```

- **Without headers** (uses column_1, column_2, etc.):
  ```csv
  Alice,30,NYC
  Bob,25,LA
  ```

### Data Types

Automatically infers types:

- **Strings**: `"IC2602"`, `"CFFEX"`
- **Integers**: `1200`, `800`
- **Floats**: `5100.5`, `5105.0`
- **Dates/Times**: `"2024-01-30"`, `"09:30:15"`

### Special Values

Handles common CSV conventions:
- **Empty values**: Displayed as empty or NULL
- **Quoted strings**: `"value with spaces"`, `"value,with,commas"`
- **Escaped quotes**: `"value with ""quotes"" inside"`

## All Features Work with CSV

### 1. Row Filtering

```bash
# Open CSV
./rata-data-viewer data.csv

# Filter by price
Press /
Type: LastPrice > 5000
Press Enter

# Filter with multiple conditions
Press /
Type: InstrumentID = IC2602 AND LastPrice > 5000
Press Enter
```

### 2. Column Selection

```bash
# Open CSV
./rata-data-viewer data.csv

# Select columns
Press c
Use ↑↓ to navigate
Press Space to toggle columns
Press Enter to apply
```

### 3. String Comparison

```bash
# Time-based filtering works
Press /
Type: UpdateTime > "09:30:00"
Press Enter
```

### 4. Line Numbers

```bash
# Toggle line numbers
Press n
```

### 5. All Navigation

```bash
↑↓←→  # Navigate
PgUp/PgDn  # Page through data
Home/End or g/G  # Jump to top/bottom
```

## CSV-Specific Considerations

### File Size

- **Small files** (<10MB): Load instantly
- **Medium files** (10-100MB): 1-5 seconds
- **Large files** (100MB-1GB): 5-30 seconds
- **Very large files** (>1GB): Consider converting to Parquet

### Performance Comparison

| Operation | CSV | Parquet | Notes |
|-----------|-----|---------|-------|
| Loading | Slower | Faster | Parquet is pre-processed |
| Filtering | Same | Same | Both use Polars in-memory |
| Column selection | Same | Same | Both use same rendering |
| Memory usage | Similar | Similar | Both loaded into memory |

**Recommendation**:
- For one-time viewing: CSV is fine
- For repeated analysis: Convert to Parquet

### Converting CSV to Parquet

If you work with the same large CSV repeatedly, convert to Parquet:

```python
# Using Python + polars
import polars as pl
df = pl.read_csv("data.csv")
df.write_parquet("data.parquet")
```

Then use Parquet file for faster loading.

## Testing CSV Support

### Test 1: Basic CSV

```bash
# Create test file
cat > test.csv << 'EOF'
Name,Age,Score
Alice,30,95.5
Bob,25,87.3
Charlie,35,92.1
EOF

# View it
./rata-data-viewer test.csv
```

### Test 2: Market Data CSV

```bash
# Create market data CSV
cat > market.csv << 'EOF'
InstrumentID,LastPrice,Volume,UpdateTime
IC2602,5100.5,1200,09:30:15
IC2602,5105.0,800,09:30:30
IC2603,5200.0,1500,09:30:15
EOF

# View and filter
./rata-data-viewer market.csv

# In viewer:
# Press / → Type: LastPrice > 5100
```

### Test 3: Different Delimiters

```bash
# Tab-delimited
cat > data.tsv << 'EOF'
Name	Age	City
Alice	30	NYC
Bob	25	LA
EOF

# Rename to .csv or polars will handle it
./rata-data-viewer data.tsv
```

## Common CSV Issues & Solutions

### Issue 1: Wrong delimiter detected

**Problem**: File uses semicolon but detected as comma
**Solution**: Polars auto-detection is very good, but if it fails, convert to standard comma-delimited CSV first

### Issue 2: Encoding problems

**Problem**: Special characters display incorrectly
**Solution**: Ensure CSV is UTF-8 encoded:
```bash
iconv -f ISO-8859-1 -t UTF-8 input.csv > output.csv
```

### Issue 3: Quoted fields with embedded delimiters

**Problem**: Values like `"Smith, John"` causing issues
**Solution**: This should work automatically. If not, check CSV is properly quoted.

### Issue 4: Very large CSV loads slowly

**Problem**: 500MB CSV takes 30+ seconds to load
**Solution**:
1. Convert to Parquet for faster loading
2. Or pre-filter the CSV with tools like `awk` or `duckdb`

## Example Workflows

### Workflow 1: Quick Data Check

```bash
# Got a CSV from colleague
./rata-data-viewer their_data.csv

# Quickly browse
Press PgDn a few times
Press → to see more columns
Press q when done
```

### Workflow 2: Find Specific Records

```bash
./rata-data-viewer transactions.csv

# Find high-value transactions
Press /
Type: Amount > 10000
Press Enter

# Further filter by date
Press Esc, then /
Type: Amount > 10000 AND Date >= "2024-01-01"
Press Enter
```

### Workflow 3: Export Filtered Data

Currently not supported directly, but you can:
1. Note the filter you used
2. Apply same filter with external tools (pandas, polars, etc.)

Example:
```python
import polars as pl
df = pl.read_csv("data.csv")
filtered = df.filter((pl.col("LastPrice") > 5000))
filtered.write_csv("filtered_data.csv")
```

## CSV vs Parquet Decision Guide

**Use CSV when**:
- ✅ File is small (<50MB)
- ✅ One-time viewing
- ✅ Received from others (standard format)
- ✅ Need human-readable format
- ✅ Editing data frequently

**Use Parquet when**:
- ✅ File is large (>100MB)
- ✅ Repeated analysis
- ✅ Best performance needed
- ✅ Columnar operations (filtering columns)
- ✅ Long-term storage

## Complete Example

```bash
# 1. Create test CSV with realistic data
cat > stock_data.csv << 'EOF'
Symbol,Date,Open,High,Low,Close,Volume
AAPL,2024-01-29,180.50,182.30,179.80,181.90,65000000
AAPL,2024-01-30,182.00,183.50,181.20,183.10,58000000
GOOGL,2024-01-29,150.20,151.80,149.50,151.30,42000000
GOOGL,2024-01-30,151.50,152.90,150.80,152.40,39000000
MSFT,2024-01-29,420.30,422.50,419.80,421.70,28000000
MSFT,2024-01-30,422.00,424.20,421.50,423.90,25000000
EOF

# 2. View the data
./rata-data-viewer stock_data.csv

# 3. In viewer:
#    - Press c → Select only Symbol, Date, Close, Volume
#    - Press Enter
#    - Press / → Type: Close > 180
#    - Press Enter
#    - Now seeing filtered data with selected columns

# 4. Results:
#    Shows only AAPL and MSFT rows (Close > 180)
#    Shows only 4 columns (not all 7)
```

## Summary

✅ **CSV support is complete** - no additional implementation needed
✅ **All features work** - filtering, column selection, navigation, etc.
✅ **Automatic format detection** - delimiter, headers, types
✅ **Performance is good** - suitable for files up to ~1GB
✅ **No special setup** - just `./rata-data-viewer file.csv`

CSV files work exactly like Parquet files with all the same capabilities!
