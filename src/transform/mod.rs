//! Field transformation pipeline.

mod builtin;
mod field;
pub mod reference;

pub use builtin::BuiltinTransforms;
pub use field::FieldTransformer;
pub use reference::ReferenceResolver;

use crate::config::mappings::{FieldMapping, TransformType};
use crate::error::{Error, Result};
use serde_json::Value;

/// Transform pipeline for converting between Attio and Salesforce formats
pub struct TransformPipeline {
    builtin: BuiltinTransforms,
}

impl TransformPipeline {
    /// Create a new transform pipeline
    pub fn new() -> Self {
        Self {
            builtin: BuiltinTransforms::new(),
        }
    }

    /// Transform a value using the specified transform type
    pub fn transform(&self, value: &Value, transform: &TransformType) -> Result<Value> {
        match transform {
            TransformType::Direct => Ok(value.clone()),

            TransformType::ExtractFirst => self.builtin.extract_first(value),

            TransformType::ExtractNested { path } => self.builtin.extract_nested(value, path),

            TransformType::MapValue { mappings } => self.builtin.map_value(value, mappings),

            TransformType::CurrencyToNumber => self.builtin.currency_to_number(value),

            TransformType::CountryCodeToName => self.builtin.country_code_to_name(value),

            TransformType::EmployeeRangeToNumber => self.builtin.employee_range_to_number(value),

            TransformType::Custom { function_name } => {
                Err(Error::transform(
                    function_name,
                    "Custom transforms not yet implemented",
                ))
            }
        }
    }

    /// Transform an Attio record to Salesforce format
    pub fn attio_to_salesforce(
        &self,
        attio_data: &Value,
        mappings: &[FieldMapping],
    ) -> Result<Value> {
        let mut sf_data = serde_json::Map::new();

        for mapping in mappings {
            if let Some(attio_value) = self.get_nested_value(attio_data, &mapping.attio_field) {
                let transformed = self.transform(&attio_value, &mapping.transform)?;
                sf_data.insert(mapping.salesforce_field.clone(), transformed);
            }
        }

        Ok(Value::Object(sf_data))
    }

    /// Transform a Salesforce record to Attio format
    pub fn salesforce_to_attio(
        &self,
        sf_data: &Value,
        mappings: &[FieldMapping],
    ) -> Result<Value> {
        let mut attio_data = serde_json::Map::new();

        for mapping in mappings {
            if let Some(sf_value) = sf_data.get(&mapping.salesforce_field) {
                // For reverse transform, we need inverse transforms
                // For now, just do direct copy
                attio_data.insert(mapping.attio_field.clone(), sf_value.clone());
            }
        }

        Ok(Value::Object(attio_data))
    }

    /// Get a nested value from JSON using dot notation path
    fn get_nested_value(&self, data: &Value, path: &str) -> Option<Value> {
        let parts: Vec<&str> = path.split('.').collect();
        let mut current = data.clone();

        for part in parts {
            // Handle array notation like "domains[0]"
            if let Some(bracket_pos) = part.find('[') {
                let field_name = &part[..bracket_pos];
                let index_str = &part[bracket_pos + 1..part.len() - 1];
                let index: usize = index_str.parse().ok()?;

                current = current.get(field_name)?.get(index)?.clone();
            } else {
                current = current.get(part)?.clone();
            }
        }

        Some(current)
    }
}

impl Default for TransformPipeline {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_direct_transform() {
        let pipeline = TransformPipeline::new();
        let value = serde_json::json!("test");
        let result = pipeline.transform(&value, &TransformType::Direct).unwrap();
        assert_eq!(result, value);
    }

    #[test]
    fn test_get_nested_value() {
        let pipeline = TransformPipeline::new();
        let data = serde_json::json!({
            "name": {
                "first_name": "John",
                "last_name": "Doe"
            },
            "emails": [
                {"email": "primary@example.com"},
                {"email": "secondary@example.com"}
            ]
        });

        assert_eq!(
            pipeline.get_nested_value(&data, "name.first_name"),
            Some(serde_json::json!("John"))
        );

        assert_eq!(
            pipeline.get_nested_value(&data, "emails[0].email"),
            Some(serde_json::json!("primary@example.com"))
        );
    }
}
