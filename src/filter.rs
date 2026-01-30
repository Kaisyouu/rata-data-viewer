use anyhow::{Context, Result, bail};
use polars::prelude::*;

/// Filter expression for advanced filtering
#[derive(Debug, Clone)]
pub enum FilterExpr {
    /// Column comparison: column_name, operator, value
    Comparison {
        column: String,
        op: ComparisonOp,
        value: String,
    },
    /// Logical AND
    And(Box<FilterExpr>, Box<FilterExpr>),
    /// Logical OR
    Or(Box<FilterExpr>, Box<FilterExpr>),
    /// Negation
    Not(Box<FilterExpr>),
}

#[derive(Debug, Clone, PartialEq)]
pub enum ComparisonOp {
    Equal,           // =
    NotEqual,        // !=
    GreaterThan,     // >
    LessThan,        // <
    GreaterOrEqual,  // >=
    LessOrEqual,     // <=
    Contains,        // : (substring match)
}

impl FilterExpr {
    /// Parse filter expression from string
    pub fn parse(input: &str) -> Result<Self> {
        let input = input.trim();
        if input.is_empty() {
            bail!("Empty filter expression");
        }

        // Try to parse logical expressions first
        if let Some(expr) = Self::try_parse_or(input)? {
            return Ok(expr);
        }

        // Try AND
        if let Some(expr) = Self::try_parse_and(input)? {
            return Ok(expr);
        }

        // Try NOT
        if let Some(expr) = Self::try_parse_not(input)? {
            return Ok(expr);
        }

        // Parse comparison
        Self::parse_comparison(input)
    }

    fn try_parse_or(input: &str) -> Result<Option<Self>> {
        // Find " OR " (case-insensitive, with spaces)
        let input_upper = input.to_uppercase();
        if let Some(pos) = input_upper.find(" OR ") {
            let left = input[..pos].trim();
            let right = input[pos + 4..].trim();

            let left_expr = Self::parse(left)?;
            let right_expr = Self::parse(right)?;

            return Ok(Some(FilterExpr::Or(
                Box::new(left_expr),
                Box::new(right_expr),
            )));
        }
        Ok(None)
    }

    fn try_parse_and(input: &str) -> Result<Option<Self>> {
        // Find " AND " (case-insensitive, with spaces)
        let input_upper = input.to_uppercase();
        if let Some(pos) = input_upper.find(" AND ") {
            let left = input[..pos].trim();
            let right = input[pos + 5..].trim();

            let left_expr = Self::parse(left)?;
            let right_expr = Self::parse(right)?;

            return Ok(Some(FilterExpr::And(
                Box::new(left_expr),
                Box::new(right_expr),
            )));
        }
        Ok(None)
    }

    fn try_parse_not(input: &str) -> Result<Option<Self>> {
        let input_upper = input.to_uppercase();
        if input_upper.starts_with("NOT ") {
            let inner = input[4..].trim();
            let inner_expr = Self::parse(inner)?;
            return Ok(Some(FilterExpr::Not(Box::new(inner_expr))));
        }
        Ok(None)
    }

    fn parse_comparison(input: &str) -> Result<Self> {
        // Try operators in order of precedence (longer first to avoid partial matches)
        let operators = [
            (">=", ComparisonOp::GreaterOrEqual),
            ("<=", ComparisonOp::LessOrEqual),
            ("!=", ComparisonOp::NotEqual),
            ("=", ComparisonOp::Equal),
            (">", ComparisonOp::GreaterThan),
            ("<", ComparisonOp::LessThan),
            (":", ComparisonOp::Contains),
        ];

        for (op_str, op) in operators {
            if let Some(pos) = input.find(op_str) {
                let column = input[..pos].trim().to_string();
                let value = input[pos + op_str.len()..].trim();

                // Remove quotes if present
                let value = value
                    .trim_matches('"')
                    .trim_matches('\'')
                    .to_string();

                if column.is_empty() || value.is_empty() {
                    bail!("Invalid comparison: both column and value required");
                }

                return Ok(FilterExpr::Comparison { column, op, value });
            }
        }

        // No operator found - treat as global search (contains in any column)
        Ok(FilterExpr::Comparison {
            column: "*".to_string(),
            op: ComparisonOp::Contains,
            value: input.to_string(),
        })
    }

    /// Apply filter expression to DataFrame
    pub fn apply(&self, df: &DataFrame) -> Result<DataFrame> {
        let mask = self.evaluate(df)?;
        df.filter(&mask).context("Failed to apply filter")
    }

    /// Evaluate expression to boolean mask
    fn evaluate(&self, df: &DataFrame) -> Result<BooleanChunked> {
        match self {
            FilterExpr::Comparison { column, op, value } => {
                if column == "*" {
                    // Global search across all columns
                    Self::evaluate_global_search(df, value)
                } else {
                    Self::evaluate_comparison(df, column, op, value)
                }
            }
            FilterExpr::And(left, right) => {
                let left_mask = left.evaluate(df)?;
                let right_mask = right.evaluate(df)?;
                Ok(&left_mask & &right_mask)
            }
            FilterExpr::Or(left, right) => {
                let left_mask = left.evaluate(df)?;
                let right_mask = right.evaluate(df)?;
                Ok(&left_mask | &right_mask)
            }
            FilterExpr::Not(inner) => {
                let mask = inner.evaluate(df)?;
                Ok(!mask)
            }
        }
    }

    fn evaluate_global_search(df: &DataFrame, pattern: &str) -> Result<BooleanChunked> {
        let mut mask: Option<BooleanChunked> = None;

        for col_name in df.get_column_names() {
            if let Ok(col) = df.column(col_name) {
                if let Ok(str_col) = col.str() {
                    if let Ok(contains) = str_col.contains_literal(pattern) {
                        mask = match mask {
                            None => Some(contains),
                            Some(existing) => Some(existing | contains),
                        };
                    }
                }
            }
        }

        mask.context("No searchable columns found")
    }

    fn evaluate_comparison(
        df: &DataFrame,
        column: &str,
        op: &ComparisonOp,
        value: &str,
    ) -> Result<BooleanChunked> {
        let col = df
            .column(column)
            .with_context(|| format!("Column '{}' not found", column))?;

        match op {
            ComparisonOp::Contains => {
                // String contains (substring match)
                let str_col = col.str().context("Column is not string type for contains operation")?;
                str_col.contains_literal(value).context("Contains operation failed")
            }
            ComparisonOp::Equal => {
                // Try as string first, then numeric
                if let Ok(str_col) = col.str() {
                    Ok(str_col.equal(value))
                } else if let Ok(num_value) = value.parse::<f64>() {
                    Self::numeric_comparison(col, num_value, |a, b| a == b)
                } else {
                    bail!("Cannot parse value for equality comparison")
                }
            }
            ComparisonOp::NotEqual => {
                if let Ok(str_col) = col.str() {
                    Ok(str_col.not_equal(value))
                } else if let Ok(num_value) = value.parse::<f64>() {
                    Self::numeric_comparison(col, num_value, |a, b| a != b)
                } else {
                    bail!("Cannot parse value for inequality comparison")
                }
            }
            ComparisonOp::GreaterThan => {
                // Try numeric comparison first
                if let Ok(num_value) = value.parse::<f64>() {
                    Self::numeric_comparison(col, num_value, |a, b| a > b)
                } else if let Ok(str_col) = col.str() {
                    // Fall back to string comparison (works for times like "09:30:00")
                    Self::string_comparison(str_col, value, |a, b| a > b)
                } else {
                    bail!("Column must be numeric or string type for > comparison")
                }
            }
            ComparisonOp::LessThan => {
                // Try numeric comparison first
                if let Ok(num_value) = value.parse::<f64>() {
                    Self::numeric_comparison(col, num_value, |a, b| a < b)
                } else if let Ok(str_col) = col.str() {
                    // Fall back to string comparison
                    Self::string_comparison(str_col, value, |a, b| a < b)
                } else {
                    bail!("Column must be numeric or string type for < comparison")
                }
            }
            ComparisonOp::GreaterOrEqual => {
                // Try numeric comparison first
                if let Ok(num_value) = value.parse::<f64>() {
                    Self::numeric_comparison(col, num_value, |a, b| a >= b)
                } else if let Ok(str_col) = col.str() {
                    // Fall back to string comparison
                    Self::string_comparison(str_col, value, |a, b| a >= b)
                } else {
                    bail!("Column must be numeric or string type for >= comparison")
                }
            }
            ComparisonOp::LessOrEqual => {
                // Try numeric comparison first
                if let Ok(num_value) = value.parse::<f64>() {
                    Self::numeric_comparison(col, num_value, |a, b| a <= b)
                } else if let Ok(str_col) = col.str() {
                    // Fall back to string comparison
                    Self::string_comparison(str_col, value, |a, b| a <= b)
                } else {
                    bail!("Column must be numeric or string type for <= comparison")
                }
            }
        }
    }

    fn numeric_comparison<F>(col: &Column, value: f64, op: F) -> Result<BooleanChunked>
    where
        F: Fn(f64, f64) -> bool,
    {
        // Try different numeric types
        if let Ok(i64_col) = col.i64() {
            Ok(i64_col
                .into_iter()
                .map(|opt_val| opt_val.map(|v| op(v as f64, value)))
                .collect())
        } else if let Ok(i32_col) = col.i32() {
            Ok(i32_col
                .into_iter()
                .map(|opt_val| opt_val.map(|v| op(v as f64, value)))
                .collect())
        } else if let Ok(f64_col) = col.f64() {
            Ok(f64_col
                .into_iter()
                .map(|opt_val| opt_val.map(|v| op(v, value)))
                .collect())
        } else if let Ok(f32_col) = col.f32() {
            Ok(f32_col
                .into_iter()
                .map(|opt_val| opt_val.map(|v| op(v as f64, value)))
                .collect())
        } else {
            bail!("Column is not numeric type")
        }
    }

    fn string_comparison<F>(str_col: &StringChunked, value: &str, op: F) -> Result<BooleanChunked>
    where
        F: Fn(&str, &str) -> bool,
    {
        Ok(str_col
            .into_iter()
            .map(|opt_val| opt_val.map(|v| op(v, value)))
            .collect())
    }
}
