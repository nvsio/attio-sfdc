//! Field-level transformation utilities.

use crate::config::mappings::FieldMapping;
use crate::error::Result;
use serde_json::Value;

/// Field transformer for individual field transformations
pub struct FieldTransformer {
    mapping: FieldMapping,
}

impl FieldTransformer {
    /// Create a new field transformer
    pub fn new(mapping: FieldMapping) -> Self {
        Self { mapping }
    }

    /// Get the Attio field name
    pub fn attio_field(&self) -> &str {
        &self.mapping.attio_field
    }

    /// Get the Salesforce field name
    pub fn salesforce_field(&self) -> &str {
        &self.mapping.salesforce_field
    }

    /// Check if this is a required field
    pub fn is_required(&self) -> bool {
        self.mapping.required
    }

    /// Get the transform type
    pub fn transform_type(&self) -> &crate::config::mappings::TransformType {
        &self.mapping.transform
    }
}

/// Utility functions for field transformations
pub mod utils {
    use super::*;

    /// Normalize a string value (trim whitespace, normalize case)
    pub fn normalize_string(value: &Value) -> Option<String> {
        value.as_str().map(|s| s.trim().to_string())
    }

    /// Convert to boolean
    pub fn to_boolean(value: &Value) -> Option<bool> {
        match value {
            Value::Bool(b) => Some(*b),
            Value::String(s) => match s.to_lowercase().as_str() {
                "true" | "yes" | "1" => Some(true),
                "false" | "no" | "0" => Some(false),
                _ => None,
            },
            Value::Number(n) => n.as_i64().map(|i| i != 0),
            _ => None,
        }
    }

    /// Convert to number
    pub fn to_number(value: &Value) -> Option<f64> {
        match value {
            Value::Number(n) => n.as_f64(),
            Value::String(s) => s.parse().ok(),
            _ => None,
        }
    }

    /// Truncate string to max length
    pub fn truncate_string(value: &Value, max_length: usize) -> Value {
        match value {
            Value::String(s) if s.len() > max_length => {
                Value::String(s.chars().take(max_length).collect())
            }
            _ => value.clone(),
        }
    }

    /// Format phone number (basic normalization)
    pub fn normalize_phone(value: &Value) -> Option<String> {
        value.as_str().map(|s| {
            // Remove common separators, keep digits and +
            s.chars()
                .filter(|c| c.is_ascii_digit() || *c == '+')
                .collect()
        })
    }

    /// Format email (lowercase)
    pub fn normalize_email(value: &Value) -> Option<String> {
        value.as_str().map(|s| s.trim().to_lowercase())
    }

    /// Check if value is empty
    pub fn is_empty(value: &Value) -> bool {
        match value {
            Value::Null => true,
            Value::String(s) => s.trim().is_empty(),
            Value::Array(arr) => arr.is_empty(),
            Value::Object(obj) => obj.is_empty(),
            _ => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::utils::*;
    use serde_json::json;

    #[test]
    fn test_normalize_string() {
        assert_eq!(
            normalize_string(&json!("  hello  ")),
            Some("hello".to_string())
        );
    }

    #[test]
    fn test_to_boolean() {
        assert_eq!(to_boolean(&json!(true)), Some(true));
        assert_eq!(to_boolean(&json!("yes")), Some(true));
        assert_eq!(to_boolean(&json!("no")), Some(false));
        assert_eq!(to_boolean(&json!(1)), Some(true));
        assert_eq!(to_boolean(&json!(0)), Some(false));
    }

    #[test]
    fn test_truncate_string() {
        let long = json!("This is a very long string");
        let truncated = truncate_string(&long, 10);
        assert_eq!(truncated, json!("This is a "));
    }

    #[test]
    fn test_normalize_phone() {
        assert_eq!(
            normalize_phone(&json!("+1 (555) 123-4567")),
            Some("+15551234567".to_string())
        );
    }

    #[test]
    fn test_normalize_email() {
        assert_eq!(
            normalize_email(&json!("  John.Doe@Example.com  ")),
            Some("john.doe@example.com".to_string())
        );
    }

    #[test]
    fn test_is_empty() {
        assert!(is_empty(&json!(null)));
        assert!(is_empty(&json!("")));
        assert!(is_empty(&json!([])));
        assert!(!is_empty(&json!("hello")));
        assert!(!is_empty(&json!(0)));
    }
}
