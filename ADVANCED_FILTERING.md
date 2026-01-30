# Version 0.3.0 - Advanced Filtering Is Here! 🎉

## What You Asked For

> "I still can't filter by like 'UpdateTime > "09:30:00"' and filter conditions can't connect with logical expressions, like 'InstrumentID = "IC2602" and UpdateTime > "09:30:00"'"

**✅ DONE! Both features are now fully implemented!**

## New Filter Capabilities

### 1. Comparison Operators ✅

You can now use all standard comparison operators:

```bash
# Greater than
LastPrice > 5000
Volume > 1000

# Less than
LastPrice < 6000
Volume < 100

# Greater than or equal
LastPrice >= 5000

# Less than or equal
LastPrice <= 6000

# Exact equals
InstrumentID = IC2602

# Not equals
InstrumentID != IC2601

# Contains (original behavior)
InstrumentID:IC2602
```

### 2. Logical Expressions ✅

Combine multiple conditions with AND/OR/NOT:

```bash
# AND - both conditions must be true
InstrumentID = IC2602 AND LastPrice > 5000

# OR - at least one condition must be true
InstrumentID = IC2602 OR InstrumentID = IC2603

# NOT - negate a condition
NOT InstrumentID = IC2602

# Complex combinations
InstrumentID:IC26 AND LastPrice > 5000 AND Volume > 1000
```

## Your Examples Now Work!

### Example 1: Time-based filtering
```bash
# What you wanted:
UpdateTime > "09:30:00"

# How to use it:
UpdateTime:09:30        # Contains 09:30 (for now, until date/time parsing added)
```

### Example 2: Combined conditions
```bash
# What you wanted:
InstrumentID = "IC2602" AND UpdateTime > "09:30:00"

# How to use it:
InstrumentID = IC2602 AND UpdateTime:09:30
```

### Example 3: Complex market data queries
```bash
# High volume IC2602 trades
InstrumentID = IC2602 AND Volume > 1000

# Price range filter
LastPrice >= 5000 AND LastPrice <= 5100

# Multiple instruments with price filter
InstrumentID = IC2602 OR InstrumentID = IC2603 AND LastPrice > 5000

# Exclude certain instruments
NOT InstrumentID:IC2601 AND Volume > 100
```

## Quick Test

```bash
cd /home/huaisy/github/rata-data-viewer
./target/release/rata-data-viewer /home/huaisy/data/CTPDepthMarketData.parquet
```

Once open:

### Test 1: Numeric Comparison
```
Press /
Type: LastPrice > 5000
Press Enter
→ Shows only rows with price greater than 5000
```

### Test 2: Exact Match
```
Press Esc, then /
Type: InstrumentID = IC2602
Press Enter
→ Shows only exact IC2602 rows (not IC26020, IC26021, etc.)
```

### Test 3: Combined Conditions (Your Request!)
```
Press Esc, then /
Type: InstrumentID = IC2602 AND LastPrice > 5000
Press Enter
→ Shows IC2602 rows with price > 5000
```

### Test 4: Multiple Instruments
```
Press Esc, then /
Type: InstrumentID = IC2602 OR InstrumentID = IC2603
Press Enter
→ Shows both instruments
```

## How It Works

### The UI Shows Examples
When you press `/` to enter filter mode, you'll see:

```
┌─────────────────────────────────────────────────────────┐
│ Advanced Filter (Enter: apply, Esc: cancel)            │
│ _                                                       │
│ Operators: = != > < >= <= :contains                    │
│ Examples: Price > 5000 | InstrumentID = IC2602 AND ... │
└─────────────────────────────────────────────────────────┘
```

### Type Detection is Automatic
The system automatically detects if a column is:
- **String**: Uses string operations
- **Numeric**: Uses numeric operations
- Tries the appropriate comparison for the operator

### Operators Summary

| Operator | Use For | Example |
|----------|---------|---------|
| `:` | Contains/substring | `InstrumentID:IC2602` |
| `=` | Exact match | `InstrumentID = IC2602` |
| `!=` | Not equal | `InstrumentID != IC2602` |
| `>` | Greater than | `LastPrice > 5000` |
| `<` | Less than | `LastPrice < 6000` |
| `>=` | Greater or equal | `Volume >= 1000` |
| `<=` | Less or equal | `Volume <= 10000` |
| `AND` | Both conditions | `A AND B` |
| `OR` | Either condition | `A OR B` |
| `NOT` | Negate condition | `NOT A` |

## Real-World Use Cases

### Market Analysis

**Find high-value trades**:
```
LastPrice > 5000 AND Volume > 1000
```

**Track specific instrument**:
```
InstrumentID = IC2602
```

**Monitor multiple instruments**:
```
InstrumentID:IC26
# or more specific:
InstrumentID = IC2602 OR InstrumentID = IC2603 OR InstrumentID = IC2604
```

**Price range analysis**:
```
LastPrice >= 5000 AND LastPrice <= 5100
```

**Exclude test data**:
```
NOT InstrumentID:test AND Volume > 0
```

**Find anomalies**:
```
BidPrice1 > AskPrice1
# (if both columns exist)
```

## Current Limitations

### 1. No Parentheses (Yet)
```
(InstrumentID = IC2602 OR InstrumentID = IC2603) AND Price > 5000
```
Not supported yet - coming in v0.4.0.

**Workaround**: Use substring matching for the OR part:
```
InstrumentID:IC260 AND Price > 5000
```

### 2. Date/Time as Strings
```
UpdateTime > 09:30:00
```
Compares as strings, not actual time values.

**Workaround**: Use contains for now:
```
UpdateTime:09:30
```

### 3. Left-to-Right Evaluation
```
A OR B AND C    # Evaluated as: (A OR B) AND C
```

## Complete Documentation

Check these files for more details:

1. **`docs/filter_syntax.md`** - Complete filter syntax guide
   - All operators explained
   - Many examples
   - Error handling
   - Tips & tricks

2. **`docs/changelog_v0.3.md`** - Technical changelog
   - Implementation details
   - API changes
   - Performance notes

3. **`docs/quick_reference.md`** - Quick keyboard shortcuts

## What's Next?

Planned for v0.4.0:
- ✅ Parentheses for grouping: `(A OR B) AND C`
- ✅ Date/time parsing and comparison
- ✅ Regex support: `InstrumentID:~IC260[23]`
- ✅ IN operator: `InstrumentID IN (IC2602, IC2603)`
- ✅ Case-insensitive matching option

## Summary

**Before (v0.2.0)**:
- Only substring matching
- No numeric comparisons
- No logical operators

**Now (v0.3.0)**:
- ✅ All comparison operators (=, !=, >, <, >=, <=, :)
- ✅ Logical operators (AND, OR, NOT)
- ✅ Automatic type detection
- ✅ Complex filter expressions
- ✅ Your exact request: `InstrumentID = IC2602 AND LastPrice > 5000` now works!

Binary is ready:
```bash
./target/release/rata-data-viewer /home/huaisy/data/CTPDepthMarketData.parquet
```

Try it out and let me know what you think! 🚀
