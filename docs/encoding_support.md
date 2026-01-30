# Encoding Support - CSV Files

## Problem Solved

**Issue**: CSV files with non-UTF-8 encoding failed to load with error:
```
Error: invalid utf-8 sequence
```

**Solution**: Automatic encoding detection and conversion to UTF-8.

## How It Works

The viewer now automatically:
1. **Detects encoding** by reading the first 8KB of the file
2. **Tries common encodings** in order:
   - UTF-8 (standard)
   - GBK (Chinese Simplified)
   - GB18030 (Chinese Extended)
   - Windows-1252 (Western European)
3. **Converts to UTF-8** transparently
4. **Loads the data** as if it were UTF-8

## Supported Encodings

### Automatic Detection

The following encodings are automatically detected and handled:

- ✅ **UTF-8** (Unicode, default)
- ✅ **UTF-8 with BOM** (Byte Order Mark)
- ✅ **GBK** (Chinese Simplified, GB2312 compatible)
- ✅ **GB18030** (Chinese Extended, includes all CJK characters)
- ✅ **Windows-1252** (Western European, similar to ISO-8859-1)

### Common Use Cases

| Source | Likely Encoding | Handled |
|--------|----------------|---------|
| Excel (Chinese Windows) | GBK | ✅ Yes |
| Modern text editors | UTF-8 | ✅ Yes |
| Legacy Windows systems | Windows-1252 | ✅ Yes |
| Chinese trading systems | GBK/GB18030 | ✅ Yes |
| Web exports | UTF-8 with BOM | ✅ Yes |

## Examples

### Chinese CSV Files

```bash
# Chinese instrument names, GBK encoded
./rata-data-viewer ctp_instruments.csv

# Automatically detects GBK and converts to UTF-8
# Chinese characters display correctly
```

### European CSV Files

```bash
# File with accented characters (é, ñ, ü)
./rata-data-viewer european_data.csv

# Automatically handles Windows-1252 encoding
```

### Mixed Encoding Files

The viewer will:
1. Try UTF-8 first (fastest)
2. If that fails, try GBK (for Chinese)
3. If that fails, try GB18030 (extended Chinese)
4. Fall back to Windows-1252 (accepts any byte)

## Performance

- **Encoding detection**: ~1ms for small files
- **Transcoding overhead**: Minimal (streamed to temp file)
- **No performance impact** on UTF-8 files (fastest path)

## Technical Details

### Detection Algorithm

```
1. Read first 8KB of file
2. Check for BOM (Byte Order Mark)
   - If found: Use BOM-specified encoding
3. If no BOM:
   - Try UTF-8 validation
   - If valid: Use UTF-8
   - If invalid:
     a. Try GBK decode without errors
     b. Try GB18030 decode without errors
     c. Fall back to Windows-1252
4. Transcode entire file to UTF-8
5. Load with Polars
```

### Temporary Files

- Transcoded data is written to temporary file
- Location: System temp directory (`/tmp` on Linux)
- Filename: `rata_temp_<timestamp>.csv`
- Automatically cleaned up after loading
- Only exists for the duration of loading

## Troubleshooting

### Issue: Characters still display as �

**Cause**: File encoding not properly detected
**Solution**: The file might use a rare encoding. Convert to UTF-8 first:

```bash
# Using iconv
iconv -f <source-encoding> -t UTF-8 input.csv > output.csv

# Common source encodings:
# - ISO-8859-1 (Latin-1)
# - Shift_JIS (Japanese)
# - EUC-KR (Korean)
```

### Issue: File loads but data looks wrong

**Cause**: Encoding was misdetected
**Solution**: Convert to UTF-8 explicitly with the correct source encoding:

```bash
# If it's actually ISO-8859-1
iconv -f ISO-8859-1 -t UTF-8 input.csv > output.csv

# If it's actually Shift_JIS (Japanese)
iconv -f Shift_JIS -t UTF-8 input.csv > output.csv
```

### Issue: Want to force a specific encoding

**Current**: Not supported directly
**Workaround**: Convert first using `iconv`

**Future**: May add command-line option:
```bash
# Planned feature
./rata-data-viewer --encoding GBK data.csv
```

## Best Practices

### For Data Producers

If you're generating CSV files:
1. ✅ **Use UTF-8** (universal compatibility)
2. ✅ **Include BOM** if your tools require it (Excel compatibility)
3. ✅ **Document encoding** if using non-UTF-8

### For Data Consumers

If you're loading CSV files:
1. ✅ **Try loading directly** - encoding detection usually works
2. ✅ **Check character display** - verify Chinese/special characters look correct
3. ✅ **Convert if needed** - use `iconv` for problematic files

## Examples

### Example 1: Chinese Market Data

```bash
# File: ctp_instruments.csv
# Encoding: GBK (from Chinese trading system)
# Content: Instrument names in Chinese

./rata-data-viewer ctp_instruments.csv

# Automatically detected as GBK
# Chinese characters display correctly
# No manual conversion needed!
```

### Example 2: Excel Export (Windows)

```bash
# File: sales_data.csv
# Encoding: Windows-1252 (Excel default on Windows)
# Content: Names with accents (José, François)

./rata-data-viewer sales_data.csv

# Automatically detected as Windows-1252
# Accented characters display correctly
```

### Example 3: UTF-8 with BOM

```bash
# File: data.csv
# Encoding: UTF-8 with BOM (from Notepad)

./rata-data-viewer data.csv

# BOM detected and removed
# Loads as clean UTF-8
```

## Encoding Information

### What is GBK?

**GBK** (Chinese: 汉字内码扩展规范) is a character encoding for Simplified Chinese:
- Used extensively in mainland China
- Compatible with GB2312 (earlier standard)
- Supports ~21,000 Chinese characters
- Common in Chinese trading systems, databases

### What is GB18030?

**GB18030** is the extended version of GBK:
- Mandatory standard in mainland China
- Supports all CJK (Chinese-Japanese-Korean) characters
- Backward compatible with GBK and GB2312
- Covers rare characters and minority languages

### Why These Encodings?

Chinese market data often uses GBK/GB18030 because:
- Legacy systems and databases use these encodings
- Trading platforms use native encodings
- Excel on Chinese Windows defaults to GBK
- Government systems require GB18030 compliance

## Summary

✅ **Automatic encoding detection** - no manual configuration needed
✅ **Chinese support** - GBK and GB18030 handled automatically
✅ **European support** - Windows-1252 and ISO-8859-1 compatible
✅ **BOM support** - UTF-8 BOM automatically detected and removed
✅ **No performance impact** - UTF-8 files load at full speed
✅ **Transparent** - you don't need to know the encoding

Your CSV files should now load regardless of encoding! 🎉
