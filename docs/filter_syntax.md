# Advanced Filter Syntax Guide

## Overview

The viewer now supports advanced filtering with comparison operators and logical expressions, allowing you to build complex queries similar to SQL WHERE clauses.

## Comparison Operators

### String Operations

#### Contains (`:`)
Substring matching within a column or globally.

```
InstrumentID:IC2602              → InstrumentID contains "IC2602"
IC2602                           → Any column contains "IC2602" (global search)
```

#### Exact Match (`=`)
Exact string matching.

```
InstrumentID = IC2602            → InstrumentID equals exactly "IC2602"
ExchangeID = CFFEX               → ExchangeID equals "CFFEX"
```

**Note**: Values with spaces should be quoted:
```
Description = "Futures Contract"
```

#### Not Equal (`!=`)
Negation of exact match.

```
InstrumentID != IC2602           → InstrumentID is not "IC2602"
ExchangeID != CFFEX              → ExchangeID is not "CFFEX"
```

### Numeric Operations

All numeric comparisons work with integer and floating-point columns.

#### Greater Than (`>`)
```
LastPrice > 5000                 → Price greater than 5000
Volume > 1000                    → Volume greater than 1000
```

#### Less Than (`<`)
```
LastPrice < 5000                 → Price less than 5000
Volume < 100                     → Volume less than 100
```

#### Greater Than or Equal (`>=`)
```
LastPrice >= 5000                → Price greater than or equal to 5000
Volume >= 1000                   → Volume at least 1000
```

#### Less Than or Equal (`<=`)
```
LastPrice <= 5000                → Price less than or equal to 5000
Volume <= 100                    → Volume at most 100
```

## Logical Operators

### AND
Combines conditions - both must be true.

```
InstrumentID = IC2602 AND LastPrice > 5000
```
Shows rows where InstrumentID is IC2602 **AND** price is greater than 5000.

### OR
Alternative conditions - at least one must be true.

```
InstrumentID = IC2602 OR InstrumentID = IC2603
```
Shows rows where InstrumentID is IC2602 **OR** IC2603.

### NOT
Negates a condition.

```
NOT InstrumentID = IC2602
```
Shows all rows except IC2602.

## Complex Expressions

You can combine multiple conditions with AND/OR.

### Example 1: Multiple Conditions
```
InstrumentID = IC2602 AND LastPrice > 5000 AND Volume > 1000
```
All three conditions must be true.

### Example 2: Alternative Instruments
```
InstrumentID = IC2602 OR InstrumentID = IC2603 OR InstrumentID = IC2604
```
Any of the three instruments.

### Example 3: Price Range
```
LastPrice >= 5000 AND LastPrice <= 5100
```
Price between 5000 and 5100 (inclusive).

### Example 4: Complex Logic
```
InstrumentID = IC2602 AND LastPrice > 5000 OR Volume > 10000
```
**Caution**: This will be evaluated as `(InstrumentID = IC2602 AND LastPrice > 5000) OR Volume > 10000`

**Note**: Current version evaluates left-to-right. Parentheses support coming soon!

## Real-World Examples

### Market Data Scenarios

#### High-value trades
```
LastPrice > 5000 AND Volume > 1000
```

#### Specific instrument during time window
```
InstrumentID = IC2602 AND UpdateTime:09:30
```
(Using contains for time matching)

#### Exclude certain instruments
```
NOT InstrumentID:IC2601 AND Volume > 100
```

#### Multiple instruments with price filter
```
InstrumentID:IC26 AND LastPrice > 5000
```
(Any IC26xx instrument with price > 5000)

#### Bid-Ask spread filter
```
BidPrice1 > 0 AND AskPrice1 > 0
```
(Only rows with valid bid and ask prices)

## Syntax Rules

### Column Names
- Must match exactly (case-sensitive)
- No quotes needed around column names
- If column doesn't exist, you'll get an error message

### Values
- **Strings**: Can be quoted or unquoted (unless containing spaces)
  - `InstrumentID = IC2602` (no quotes)
  - `InstrumentID = "IC2602"` (with quotes, same result)
  - `Description = "Futures Contract"` (quotes required for spaces)

- **Numbers**: No quotes
  - `Price > 5000` (correct)
  - `Price > "5000"` (also works, parsed as number)

### Operators
- Must have spaces around them (except `:`)
  - `Price>5000` ❌ (might work but discouraged)
  - `Price > 5000` ✅ (correct)
  - `InstrumentID:IC2602` ✅ (colon doesn't need spaces)

### Logical Keywords
- Case-insensitive: `AND`, `and`, `And` all work
- Must have spaces: `Price>5000ANDVolume>100` won't work
- Must be separate words: `Price>5000 AND Volume>100` ✅

## Type Handling

### Automatic Type Detection
The filter system automatically detects column types:

- **String columns**: Use string operations (=, !=, :)
- **Numeric columns**: Use numeric operations (>, <, >=, <=)
- **Mixed operations**: Try string first, fall back to numeric

### Examples
```
# String column
InstrumentID = IC2602        → String exact match
InstrumentID:IC26            → String contains

# Numeric column
LastPrice > 5000             → Numeric comparison
Volume >= 1000               → Numeric comparison

# Works on both
LastPrice = 5000             → Exact match (tries string, falls back to numeric)
```

## Current Limitations

### No Parentheses (Yet)
```
(InstrumentID = IC2602 OR InstrumentID = IC2603) AND Price > 5000
```
**Not supported yet** - coming in next version.

**Workaround**: Use multiple filters:
1. Filter: `InstrumentID:IC260` (gets IC2602, IC2603, IC2604, etc.)
2. Refine: Add `AND Price > 5000`

### Evaluation Order
- Evaluated left-to-right
- AND and OR have same precedence
- Can lead to unexpected results with mixed operators

```
A OR B AND C
```
Evaluated as: `(A OR B) AND C`
Not as: `A OR (B AND C)`

### No Regex (Yet)
```
InstrumentID =~ IC260[23]    # Not supported
```
Use contains for now: `InstrumentID:IC260`

### No Date/Time Parsing (Yet)
```
UpdateTime > 09:30:00        # Doesn't work as expected
```
Use string contains for now: `UpdateTime:09:30`

### No IN Operator (Yet)
```
InstrumentID IN (IC2602, IC2603, IC2604)    # Not supported
```
Use OR: `InstrumentID = IC2602 OR InstrumentID = IC2603 OR InstrumentID = IC2604`

## Performance Notes

- **Numeric comparisons**: Very fast (vectorized operations)
- **String contains**: Fast for most cases
- **Complex expressions**: AND/OR add minimal overhead
- **Global search** (`value` without column): Slower (searches all columns)

**Tip**: Use column-specific filters when possible for best performance.

## Error Messages

### Column not found
```
Filter: NonExistentColumn > 5000
Error: Column 'NonExistentColumn' not found
```
**Fix**: Check column name spelling (case-sensitive)

### Type mismatch
```
Filter: InstrumentID > 5000
Error: Column is not numeric type
```
**Fix**: Use appropriate operator for column type

### Parse error
```
Filter: Price > abc
Error: Value must be numeric for > comparison
```
**Fix**: Use valid numeric value

### Invalid syntax
```
Filter: Price >> 5000
Error: Invalid comparison: both column and value required
```
**Fix**: Use valid operator (>, <, >=, <=, =, !=, :)

## Tips & Tricks

### Start Simple, Then Refine
1. Start with basic filter: `InstrumentID:IC2602`
2. Check results
3. Add more conditions: `InstrumentID:IC2602 AND Price > 5000`
4. Refine further: `InstrumentID:IC2602 AND Price > 5000 AND Volume > 1000`

### Use Contains for Partial Matches
```
InstrumentID:IC26           → All IC26xx instruments
UpdateTime:09:30            → All times in 09:30:xx
```

### Combine Exact and Range
```
InstrumentID = IC2602 AND LastPrice >= 5000 AND LastPrice <= 5100
```

### Use NOT for Exclusions
```
NOT InstrumentID:IC2601     → Everything except IC2601
```

### Test Filters Incrementally
- Press `/` to enter filter mode
- Type your filter
- Press `Enter` to see results
- If wrong, press `Esc` and try again

## Quick Reference

| Operator | Meaning | Example |
|----------|---------|---------|
| `:` | Contains | `InstrumentID:IC2602` |
| `=` | Equals | `InstrumentID = IC2602` |
| `!=` | Not equals | `InstrumentID != IC2602` |
| `>` | Greater than | `Price > 5000` |
| `<` | Less than | `Price < 5000` |
| `>=` | Greater or equal | `Price >= 5000` |
| `<=` | Less or equal | `Price <= 5000` |
| `AND` | Logical AND | `Price > 5000 AND Volume > 100` |
| `OR` | Logical OR | `InstrumentID = IC2602 OR InstrumentID = IC2603` |
| `NOT` | Logical NOT | `NOT InstrumentID = IC2602` |

## Coming Soon

- Parentheses for grouping expressions
- Regex support (`:~` operator)
- Date/time parsing and comparison
- IN operator for multiple values
- LIKE operator with wildcards
- Case-insensitive string matching option
