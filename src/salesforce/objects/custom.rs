//! Salesforce custom objects support.

use crate::error::{Error, Result};
use crate::salesforce::types::{SObject, SObjectField, SalesforceRecord};
use serde_json::Value;
use std::collections::HashMap;

/// Handler for custom Salesforce objects
#[derive(Debug)]
pub struct CustomObjectHandler {
    /// Object metadata
    object: SObject,

    /// Field name to type mapping
    field_types: HashMap<String, String>,
}

impl CustomObjectHandler {
    /// Create a handler from object metadata
    pub fn new(object: SObject) -> Self {
        let field_types = object
            .fields
            .iter()
            .map(|f| (f.name.clone(), format!("{:?}", f.field_type)))
            .collect();

        Self {
            object,
            field_types,
        }
    }

    /// Get object API name
    pub fn api_name(&self) -> &str {
        &self.object.name
    }

    /// Check if this is a custom object
    pub fn is_custom(&self) -> bool {
        self.object.custom
    }

    /// Get field metadata
    pub fn get_field(&self, name: &str) -> Option<&SObjectField> {
        self.object.fields.iter().find(|f| f.name == name)
    }

    /// Get all required fields
    pub fn required_fields(&self) -> Vec<&SObjectField> {
        self.object
            .fields
            .iter()
            .filter(|f| !f.nillable && f.createable)
            .collect()
    }

    /// Validate that all required fields are present
    pub fn validate_required_fields(&self, data: &Value) -> Result<()> {
        for field in self.required_fields() {
            let has_value = data
                .get(&field.name)
                .map(|v| !v.is_null())
                .unwrap_or(false);

            if !has_value {
                return Err(Error::validation(format!(
                    "Missing required field: {}",
                    field.name
                )));
            }
        }
        Ok(())
    }

    /// Build a record from field values
    pub fn build_record(&self, fields: HashMap<String, Value>) -> SalesforceRecord {
        let mut record = SalesforceRecord::new(&self.object.name);

        for (key, value) in fields {
            // Only include fields that exist on the object
            if self.field_types.contains_key(&key) {
                record.set(&key, value);
            }
        }

        record
    }

    /// Get all updateable fields
    pub fn updateable_fields(&self) -> Vec<&str> {
        self.object
            .fields
            .iter()
            .filter(|f| f.updateable)
            .map(|f| f.name.as_str())
            .collect()
    }

    /// Get all createable fields
    pub fn createable_fields(&self) -> Vec<&str> {
        self.object
            .fields
            .iter()
            .filter(|f| f.createable)
            .map(|f| f.name.as_str())
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::salesforce::types::SalesforceFieldType;

    fn test_object() -> SObject {
        SObject {
            name: "Custom__c".to_string(),
            label: "Custom".to_string(),
            label_plural: "Customs".to_string(),
            custom: true,
            queryable: true,
            createable: true,
            updateable: true,
            deletable: true,
            fields: vec![
                SObjectField {
                    name: "Name".to_string(),
                    label: "Name".to_string(),
                    field_type: SalesforceFieldType::String,
                    nillable: false,
                    createable: true,
                    updateable: true,
                    length: Some(80),
                    picklist_values: vec![],
                    reference_to: vec![],
                },
            ],
        }
    }

    #[test]
    fn test_handler_creation() {
        let handler = CustomObjectHandler::new(test_object());
        assert_eq!(handler.api_name(), "Custom__c");
        assert!(handler.is_custom());
    }

    #[test]
    fn test_required_fields() {
        let handler = CustomObjectHandler::new(test_object());
        let required = handler.required_fields();
        assert_eq!(required.len(), 1);
        assert_eq!(required[0].name, "Name");
    }
}
