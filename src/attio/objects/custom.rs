//! Attio custom objects support.

use crate::attio::types::{AttioObject, AttioAttribute, AttioRecord};
use crate::error::{Error, Result};
use serde_json::Value;
use std::collections::HashMap;

/// Handler for custom Attio objects
#[derive(Debug)]
pub struct CustomObjectHandler {
    /// Object definition
    object: AttioObject,

    /// Field type mappings
    field_types: HashMap<String, String>,
}

impl CustomObjectHandler {
    /// Create a handler from an object definition
    pub fn new(object: AttioObject) -> Self {
        let field_types = object
            .attributes
            .iter()
            .map(|attr| (attr.api_slug.clone(), format!("{:?}", attr.attribute_type)))
            .collect();

        Self {
            object,
            field_types,
        }
    }

    /// Get object slug
    pub fn slug(&self) -> &str {
        &self.object.api_slug
    }

    /// Get attribute by slug
    pub fn get_attribute(&self, slug: &str) -> Option<&AttioAttribute> {
        self.object.attributes.iter().find(|a| a.api_slug == slug)
    }

    /// Extract a field value from a record
    pub fn extract_field(&self, record: &AttioRecord, field: &str) -> Option<Value> {
        record.values.get(field).map(|v| serde_json::to_value(v).ok()).flatten()
    }

    /// Validate that required fields are present
    pub fn validate_required_fields(&self, data: &Value) -> Result<()> {
        for attr in &self.object.attributes {
            if attr.is_required {
                let has_value = data
                    .get(&attr.api_slug)
                    .map(|v| !v.is_null())
                    .unwrap_or(false);

                if !has_value {
                    return Err(Error::validation(format!(
                        "Missing required field: {}",
                        attr.api_slug
                    )));
                }
            }
        }
        Ok(())
    }

    /// Build a data payload for creating/updating a record
    pub fn build_payload(&self, fields: HashMap<String, Value>) -> Value {
        let mut data = serde_json::Map::new();

        for (key, value) in fields {
            if self.field_types.contains_key(&key) {
                data.insert(key, value);
            }
        }

        Value::Object(data)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::attio::types::{AttioObjectId, AttioAttributeType};
    use chrono::Utc;

    fn test_object() -> AttioObject {
        AttioObject {
            id: AttioObjectId("obj_123".to_string()),
            api_slug: "custom_object".to_string(),
            singular_noun: "Custom Object".to_string(),
            plural_noun: "Custom Objects".to_string(),
            is_system_object: false,
            attributes: vec![
                AttioAttribute {
                    id: "attr_1".to_string(),
                    api_slug: "name".to_string(),
                    title: "Name".to_string(),
                    attribute_type: AttioAttributeType::Text,
                    is_system_attribute: false,
                    is_required: true,
                    is_multiselect: false,
                },
            ],
            created_at: Utc::now(),
        }
    }

    #[test]
    fn test_handler_creation() {
        let handler = CustomObjectHandler::new(test_object());
        assert_eq!(handler.slug(), "custom_object");
    }

    #[test]
    fn test_validate_required_fields() {
        let handler = CustomObjectHandler::new(test_object());

        let valid = serde_json::json!({"name": "Test"});
        assert!(handler.validate_required_fields(&valid).is_ok());

        let invalid = serde_json::json!({});
        assert!(handler.validate_required_fields(&invalid).is_err());
    }
}
