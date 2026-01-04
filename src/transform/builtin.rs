//! Built-in transform implementations.

use crate::error::{Error, Result};
use serde_json::Value;
use std::collections::HashMap;

/// Built-in transformations
pub struct BuiltinTransforms {
    country_codes: HashMap<String, String>,
}

impl BuiltinTransforms {
    /// Create a new instance with default mappings
    pub fn new() -> Self {
        Self {
            country_codes: Self::default_country_codes(),
        }
    }

    /// Extract first element from an array
    pub fn extract_first(&self, value: &Value) -> Result<Value> {
        match value {
            Value::Array(arr) => arr
                .first()
                .cloned()
                .ok_or_else(|| Error::transform("array", "Array is empty")),
            _ => Ok(value.clone()),
        }
    }

    /// Extract nested value using dot notation
    pub fn extract_nested(&self, value: &Value, path: &str) -> Result<Value> {
        let parts: Vec<&str> = path.split('.').collect();
        let mut current = value.clone();

        for part in parts {
            match current.get(part) {
                Some(v) => current = v.clone(),
                None => {
                    return Err(Error::transform(
                        path,
                        format!("Field '{}' not found", part),
                    ))
                }
            }
        }

        Ok(current)
    }

    /// Map value using a lookup table
    pub fn map_value(&self, value: &Value, mappings: &HashMap<String, String>) -> Result<Value> {
        let key = match value {
            Value::String(s) => s.clone(),
            Value::Number(n) => n.to_string(),
            _ => return Ok(value.clone()),
        };

        match mappings.get(&key) {
            Some(mapped) => Ok(Value::String(mapped.clone())),
            None => Ok(value.clone()), // Pass through if no mapping
        }
    }

    /// Convert currency object to number
    pub fn currency_to_number(&self, value: &Value) -> Result<Value> {
        match value {
            Value::Object(obj) => {
                // Try common currency field names
                for key in &["value", "currency_value", "amount"] {
                    if let Some(Value::Number(n)) = obj.get(*key) {
                        return Ok(Value::Number(n.clone()));
                    }
                }
                Err(Error::transform("currency", "No numeric value found"))
            }
            Value::Number(_) => Ok(value.clone()),
            _ => Err(Error::transform("currency", "Expected object or number")),
        }
    }

    /// Convert country code to country name
    pub fn country_code_to_name(&self, value: &Value) -> Result<Value> {
        match value {
            Value::String(code) => {
                let name = self
                    .country_codes
                    .get(&code.to_uppercase())
                    .cloned()
                    .unwrap_or_else(|| code.clone());
                Ok(Value::String(name))
            }
            _ => Ok(value.clone()),
        }
    }

    /// Convert employee range to midpoint number
    pub fn employee_range_to_number(&self, value: &Value) -> Result<Value> {
        match value {
            Value::String(range) => {
                let num = parse_employee_range(range);
                Ok(Value::Number(serde_json::Number::from(num)))
            }
            Value::Number(_) => Ok(value.clone()),
            _ => Ok(Value::Null),
        }
    }

    /// Default country code to name mappings
    fn default_country_codes() -> HashMap<String, String> {
        [
            ("US", "United States"),
            ("GB", "United Kingdom"),
            ("CA", "Canada"),
            ("AU", "Australia"),
            ("DE", "Germany"),
            ("FR", "France"),
            ("JP", "Japan"),
            ("CN", "China"),
            ("IN", "India"),
            ("BR", "Brazil"),
            ("MX", "Mexico"),
            ("ES", "Spain"),
            ("IT", "Italy"),
            ("NL", "Netherlands"),
            ("SE", "Sweden"),
            ("NO", "Norway"),
            ("DK", "Denmark"),
            ("FI", "Finland"),
            ("SG", "Singapore"),
            ("HK", "Hong Kong"),
            ("KR", "South Korea"),
            ("NZ", "New Zealand"),
            ("IE", "Ireland"),
            ("CH", "Switzerland"),
            ("AT", "Austria"),
            ("BE", "Belgium"),
            ("PL", "Poland"),
            ("PT", "Portugal"),
            ("IL", "Israel"),
            ("AE", "United Arab Emirates"),
        ]
        .into_iter()
        .map(|(k, v)| (k.to_string(), v.to_string()))
        .collect()
    }
}

impl Default for BuiltinTransforms {
    fn default() -> Self {
        Self::new()
    }
}

/// Parse employee range string to midpoint number
fn parse_employee_range(range: &str) -> i64 {
    // Common formats: "1-10", "11-50", "51-200", "201-500", "500+"
    if range.ends_with('+') {
        range.trim_end_matches('+').parse().unwrap_or(0)
    } else if range.contains('-') {
        let parts: Vec<&str> = range.split('-').collect();
        if parts.len() == 2 {
            let low: i64 = parts[0].trim().parse().unwrap_or(0);
            let high: i64 = parts[1].trim().parse().unwrap_or(0);
            (low + high) / 2
        } else {
            0
        }
    } else {
        range.parse().unwrap_or(0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_first() {
        let transforms = BuiltinTransforms::new();
        let arr = serde_json::json!(["first", "second", "third"]);
        let result = transforms.extract_first(&arr).unwrap();
        assert_eq!(result, serde_json::json!("first"));
    }

    #[test]
    fn test_extract_nested() {
        let transforms = BuiltinTransforms::new();
        let data = serde_json::json!({
            "location": {
                "city": "San Francisco"
            }
        });
        let result = transforms.extract_nested(&data, "location.city").unwrap();
        assert_eq!(result, serde_json::json!("San Francisco"));
    }

    #[test]
    fn test_currency_to_number() {
        let transforms = BuiltinTransforms::new();
        let currency = serde_json::json!({
            "value": 10000,
            "currency_code": "USD"
        });
        let result = transforms.currency_to_number(&currency).unwrap();
        assert_eq!(result, serde_json::json!(10000));
    }

    #[test]
    fn test_country_code_to_name() {
        let transforms = BuiltinTransforms::new();
        let code = serde_json::json!("US");
        let result = transforms.country_code_to_name(&code).unwrap();
        assert_eq!(result, serde_json::json!("United States"));
    }

    #[test]
    fn test_employee_range_to_number() {
        let transforms = BuiltinTransforms::new();

        let range = serde_json::json!("11-50");
        let result = transforms.employee_range_to_number(&range).unwrap();
        assert_eq!(result, serde_json::json!(30));

        let plus = serde_json::json!("500+");
        let result = transforms.employee_range_to_number(&plus).unwrap();
        assert_eq!(result, serde_json::json!(500));
    }
}
