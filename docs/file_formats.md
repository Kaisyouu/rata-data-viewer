# Supported File Formats

## Fully Supported Formats

### 1. Parquet (.parquet)

**Status**: ✅ Full support

**Description**: Apache Parquet is a columnar storage format optimized for analytics.

**Best for**:
- Large datasets (100MB - 10GB+)
- Columnar operations
- Fast loading and filtering
- Long-term storage

**Features**:
- Compressed storage (smaller file size)
- Fast column scanning
- Efficient memory usage
- Type preservation

**Usage**:
```bash
./rata-data-viewer data.parquet
```

**Performance**:
- 10MB file: ~0.5 seconds
- 100MB file: ~2-5 seconds
- 1GB file: ~10-30 seconds

---

### 2. CSV (.csv)

**Status**: ✅ Full support

**Description**: Comma-Separated Values, the universal data exchange format.

**Best for**:
- Small to medium datasets (<500MB)
- Human-readable data
- Data from spreadsheets
- One-time analysis

**Features**:
- Automatic delimiter detection (comma, tab, semicolon, pipe)
- Automatic header detection
- Automatic type inference
- Handles quoted fields and escaped characters

**Usage**:
```bash
./rata-data-viewer data.csv
```

**Performance**:
- 10MB file: ~1 second
- 100MB file: ~5-10 seconds
- 500MB file: ~30-60 seconds

**See also**: [Complete CSV Guide](csv_support.md)

---

### 3. SQLite (.db, .sqlite, .sqlite3)

**Status**: ⚠️ Limited support (coming soon)

**Description**: SQLite database files.

**Current status**:
- File detection works
- Loading returns helpful error message
- Full support planned for future version

**Planned features**:
- Table selection UI
- Query interface
- Join support

**Workaround**:
Export tables to CSV or Parquet using SQLite CLI:
```bash
# Export to CSV
sqlite3 data.db "SELECT * FROM table_name;" > output.csv

# Then view
./rata-data-viewer output.csv
```

---

## Format Comparison

| Feature | Parquet | CSV | SQLite |
|---------|---------|-----|--------|
| **File size** | Small (compressed) | Large (text) | Medium |
| **Load speed** | ⚡ Fastest | 🐢 Slower | ⏳ Coming soon |
| **Human readable** | ❌ No (binary) | ✅ Yes (text) | ❌ No (binary) |
| **Column filtering** | ⚡ Very fast | ✅ Fast | ⏳ Coming soon |
| **Type preservation** | ✅ Perfect | ⚠️ Inferred | ✅ Perfect |
| **Compression** | ✅ Built-in | ❌ No | ✅ Built-in |
| **Max file size** | 10GB+ | ~1GB | ⏳ Coming soon |

## Feature Support Matrix

| Feature | Parquet | CSV | SQLite |
|---------|---------|-----|--------|
| Row filtering | ✅ | ✅ | ⏳ |
| Column selection | ✅ | ✅ | ⏳ |
| Line numbers | ✅ | ✅ | ⏳ |
| Comparison operators | ✅ | ✅ | ⏳ |
| Logical operators | ✅ | ✅ | ⏳ |
| String comparison | ✅ | ✅ | ⏳ |
| Navigation | ✅ | ✅ | ⏳ |
| Pagination | ✅ | ✅ | ⏳ |

**Legend**: ✅ Supported | ⏳ Planned | ❌ Not supported

## Choosing the Right Format

### Use Parquet when:

✅ **Performance is critical**
- Loading large files repeatedly
- Need fast column filtering
- Working with >100MB files

✅ **Storage efficiency matters**
- Limited disk space
- Archiving data
- Network transfer costs

✅ **Data types are important**
- Precise numeric types
- Date/time types
- Categorical data

✅ **Columnar operations**
- Selecting specific columns
- Aggregating by column
- Column statistics

### Use CSV when:

✅ **Simplicity is key**
- Quick one-time viewing
- Sharing with non-technical users
- Need to edit in spreadsheet

✅ **Human readability needed**
- Debugging data issues
- Manual inspection
- Version control (git)

✅ **Compatibility required**
- Tools that only read CSV
- Legacy systems
- Web applications

✅ **Small files**
- <50MB files
- Few columns (<20)
- Few rows (<100k)

### When to convert CSV → Parquet:

Convert when:
- ✅ File is >100MB
- ✅ You'll use it repeatedly
- ✅ Loading takes >10 seconds
- ✅ You need better performance

How to convert:
```python
# Using polars (recommended)
import polars as pl
df = pl.read_csv("input.csv")
df.write_parquet("output.parquet")

# Using pandas
import pandas as pd
df = pd.read_csv("input.csv")
df.to_parquet("output.parquet")
```

## File Size Guidelines

### Parquet
- ✅ **Ideal**: 1MB - 1GB
- ⚠️ **Works**: 1GB - 5GB (may be slow)
- ❌ **Too large**: >5GB (consider splitting)

### CSV
- ✅ **Ideal**: 1MB - 100MB
- ⚠️ **Works**: 100MB - 500MB (slower)
- ❌ **Too large**: >500MB (convert to Parquet)

### SQLite
- ⏳ **Coming soon**
- Planned support for databases up to 10GB

## Future Format Support

Planned for future versions:

### JSON (.json, .jsonl)
- Nested data support
- JSON Lines format
- Array handling

### Excel (.xlsx, .xls)
- Spreadsheet viewing
- Multiple sheet support
- Formula evaluation

### Apache Arrow (.arrow, .feather)
- Zero-copy reading
- Cross-language support
- Fast serialization

### HDF5 (.h5, .hdf5)
- Scientific data
- Multidimensional arrays
- Hierarchical structure

## Tips

### For Best Performance:
1. **Use Parquet** for files >100MB
2. **Convert frequently-used CSV files** to Parquet
3. **Filter data before loading** if file is very large
4. **Consider file splitting** for multi-GB files

### For Best Compatibility:
1. **Use CSV** for sharing with others
2. **Include headers** in CSV files
3. **Use standard delimiters** (comma, tab)
4. **UTF-8 encoding** for special characters

### For Best Experience:
1. **Start with CSV** for exploration
2. **Convert to Parquet** when performance matters
3. **Keep original CSV** as backup
4. **Document your data format** for others

## Examples

### Example 1: Quick CSV View
```bash
./rata-data-viewer data.csv
# Works perfectly for small files
```

### Example 2: Large Parquet Analysis
```bash
./rata-data-viewer large_data.parquet
# Loads fast, filters fast
```

### Example 3: Convert and Compare
```bash
# Original CSV (100MB, loads in 10 seconds)
./rata-data-viewer data.csv

# Convert to Parquet
python -c "import polars as pl; pl.read_csv('data.csv').write_parquet('data.parquet')"

# Parquet version (30MB, loads in 2 seconds)
./rata-data-viewer data.parquet
```

## Summary

| Format | Status | Best Use | Performance | Size |
|--------|--------|----------|-------------|------|
| **Parquet** | ✅ Full | Large files, repeated use | ⚡ Excellent | 🗜️ Compressed |
| **CSV** | ✅ Full | Small files, sharing | ✅ Good | 📄 Large |
| **SQLite** | ⏳ Soon | Relational data | ⏳ TBD | 🗜️ Compressed |

**Recommendation**: Start with CSV for simplicity, upgrade to Parquet for performance.
