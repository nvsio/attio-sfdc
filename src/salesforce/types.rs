//! Salesforce type definitions.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

/// Salesforce record ID (18-character)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct SalesforceId(pub String);

/// Generic Salesforce SObject record
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SalesforceRecord {
    /// Record ID
    #[serde(rename = "Id")]
    pub id: Option<SalesforceId>,

    /// Object type (e.g., "Account", "Contact")
    #[serde(rename = "attributes")]
    pub attributes: Option<SObjectAttributes>,

    /// Record fields
    #[serde(flatten)]
    pub fields: HashMap<String, Value>,
}

/// SObject attributes metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SObjectAttributes {
    /// Object API name
    #[serde(rename = "type")]
    pub sobject_type: String,

    /// Record URL
    pub url: Option<String>,
}

/// SObject metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SObject {
    /// Object API name
    pub name: String,

    /// Human-readable label
    pub label: String,

    /// Plural label
    #[serde(rename = "labelPlural")]
    pub label_plural: String,

    /// Whether the object is custom
    pub custom: bool,

    /// Whether the object is queryable
    pub queryable: bool,

    /// Whether the object is createable
    pub createable: bool,

    /// Whether the object is updateable
    pub updateable: bool,

    /// Whether the object is deletable
    pub deletable: bool,

    /// Object fields
    #[serde(default)]
    pub fields: Vec<SObjectField>,
}

/// SObject field metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SObjectField {
    /// Field API name
    pub name: String,

    /// Human-readable label
    pub label: String,

    /// Field type
    #[serde(rename = "type")]
    pub field_type: SalesforceFieldType,

    /// Whether the field is required
    pub nillable: bool,

    /// Whether the field is createable
    pub createable: bool,

    /// Whether the field is updateable
    pub updateable: bool,

    /// Field length (for text fields)
    pub length: Option<u32>,

    /// Picklist values (for picklist fields)
    #[serde(rename = "picklistValues", default)]
    pub picklist_values: Vec<PicklistValue>,

    /// Reference object (for relationship fields)
    #[serde(rename = "referenceTo", default)]
    pub reference_to: Vec<String>,
}

/// Salesforce field types
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum SalesforceFieldType {
    Id,
    String,
    Boolean,
    Int,
    Double,
    Currency,
    Date,
    DateTime,
    Email,
    Phone,
    Url,
    Textarea,
    Picklist,
    MultiPicklist,
    Reference,
    Address,
    Location,
    #[serde(other)]
    Unknown,
}

/// Picklist value
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PicklistValue {
    /// Value
    pub value: String,

    /// Label
    pub label: String,

    /// Whether this is the default value
    #[serde(default)]
    pub default_value: bool,

    /// Whether this value is active
    #[serde(default = "default_true")]
    pub active: bool,
}

fn default_true() -> bool {
    true
}

/// Query result from SOQL
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueryResult {
    /// Total number of records matching the query
    #[serde(rename = "totalSize")]
    pub total_size: u32,

    /// Whether there are more records
    pub done: bool,

    /// URL for next batch (if not done)
    #[serde(rename = "nextRecordsUrl")]
    pub next_records_url: Option<String>,

    /// Records
    pub records: Vec<SalesforceRecord>,
}

/// Composite request for batch operations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompositeRequest {
    /// Whether to allow partial success
    #[serde(rename = "allOrNone")]
    pub all_or_none: bool,

    /// Individual requests
    #[serde(rename = "compositeRequest")]
    pub composite_request: Vec<CompositeSubrequest>,
}

/// Individual subrequest in a composite request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompositeSubrequest {
    /// HTTP method
    pub method: String,

    /// URL
    pub url: String,

    /// Reference ID for this request
    #[serde(rename = "referenceId")]
    pub reference_id: String,

    /// Request body
    pub body: Option<Value>,
}

impl SalesforceRecord {
    /// Create a new record for a given object type
    pub fn new(sobject_type: &str) -> Self {
        Self {
            id: None,
            attributes: Some(SObjectAttributes {
                sobject_type: sobject_type.to_string(),
                url: None,
            }),
            fields: HashMap::new(),
        }
    }

    /// Set a field value
    pub fn set(&mut self, field: &str, value: impl Into<Value>) {
        self.fields.insert(field.to_string(), value.into());
    }

    /// Get a field value
    pub fn get(&self, field: &str) -> Option<&Value> {
        self.fields.get(field)
    }

    /// Get a string field
    pub fn get_string(&self, field: &str) -> Option<String> {
        self.get(field).and_then(|v| v.as_str().map(String::from))
    }

    /// Get the object type
    pub fn object_type(&self) -> Option<&str> {
        self.attributes.as_ref().map(|a| a.sobject_type.as_str())
    }
}

impl std::fmt::Display for SalesforceId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl SalesforceId {
    /// Create a new Salesforce ID
    pub fn new(id: impl Into<String>) -> Self {
        Self(id.into())
    }

    /// Get the 15-character version of the ID
    pub fn to_15_char(&self) -> &str {
        if self.0.len() >= 15 {
            &self.0[..15]
        } else {
            &self.0
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_salesforce_record_new() {
        let mut record = SalesforceRecord::new("Account");
        record.set("Name", "Test Account");

        assert_eq!(record.object_type(), Some("Account"));
        assert_eq!(record.get_string("Name"), Some("Test Account".to_string()));
    }

    #[test]
    fn test_salesforce_id_to_15_char() {
        let id = SalesforceId::new("001000000000001AAA");
        assert_eq!(id.to_15_char(), "001000000000001");
    }
}
