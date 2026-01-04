//! Attio type definitions.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

/// Represents an Attio object definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AttioObject {
    /// Object ID
    pub id: AttioObjectId,

    /// Object slug (e.g., "companies", "people")
    pub api_slug: String,

    /// Human-readable name
    pub singular_noun: String,

    /// Plural name
    pub plural_noun: String,

    /// Whether this is a system object
    pub is_system_object: bool,

    /// Object attributes
    #[serde(default)]
    pub attributes: Vec<AttioAttribute>,

    /// Creation timestamp
    pub created_at: DateTime<Utc>,
}

/// Attio object ID
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct AttioObjectId(pub String);

/// Attio record ID
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct AttioRecordId(pub String);

/// Represents an attribute definition on an Attio object
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AttioAttribute {
    /// Attribute ID
    pub id: String,

    /// Attribute slug
    pub api_slug: String,

    /// Human-readable title
    pub title: String,

    /// Attribute type
    #[serde(rename = "type")]
    pub attribute_type: AttioAttributeType,

    /// Whether this is a system attribute
    pub is_system_attribute: bool,

    /// Whether this attribute is required
    pub is_required: bool,

    /// Whether this attribute supports multiple values
    pub is_multiselect: bool,
}

/// Attio attribute types
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum AttioAttributeType {
    Text,
    Number,
    Currency,
    Date,
    Timestamp,
    Checkbox,
    Select,
    Status,
    Rating,
    EmailAddress,
    PhoneNumber,
    Domain,
    Location,
    PersonalName,
    RecordReference,
    ActorReference,
    Interaction,
    #[serde(other)]
    Unknown,
}

/// Represents a record in Attio
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AttioRecord {
    /// Record ID
    pub id: AttioRecordId,

    /// Object this record belongs to
    pub object: AttioObjectId,

    /// Record values (attribute slug → value)
    pub values: HashMap<String, AttioValue>,

    /// Creation timestamp
    pub created_at: DateTime<Utc>,

    /// Last update timestamp
    pub updated_at: DateTime<Utc>,
}

/// Attio attribute value (can be single or multi-value)
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AttioValue {
    /// Single value
    Single(AttioValueItem),

    /// Multiple values
    Multiple(Vec<AttioValueItem>),
}

/// Individual value item
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AttioValueItem {
    /// The actual value
    #[serde(flatten)]
    pub value: AttioValueType,

    /// Value metadata
    #[serde(flatten)]
    pub metadata: HashMap<String, Value>,
}

/// Typed value representations
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AttioValueType {
    /// Text value
    Text { value: String },

    /// Number value
    Number { value: f64 },

    /// Currency value
    Currency {
        currency_value: f64,
        currency_code: String,
    },

    /// Date value
    Date { value: String },

    /// Timestamp value
    Timestamp { value: DateTime<Utc> },

    /// Boolean value
    Boolean { value: bool },

    /// Email address
    Email {
        email_address: String,
        #[serde(default)]
        is_primary: bool,
    },

    /// Phone number
    Phone {
        phone_number: String,
        #[serde(default)]
        is_primary: bool,
    },

    /// Domain
    Domain { domain: String },

    /// Location
    Location {
        #[serde(default)]
        line_1: Option<String>,
        #[serde(default)]
        line_2: Option<String>,
        #[serde(default)]
        locality: Option<String>,
        #[serde(default)]
        region: Option<String>,
        #[serde(default)]
        postcode: Option<String>,
        #[serde(default)]
        country_code: Option<String>,
    },

    /// Personal name
    PersonalName {
        #[serde(default)]
        first_name: Option<String>,
        #[serde(default)]
        last_name: Option<String>,
        #[serde(default)]
        full_name: Option<String>,
    },

    /// Record reference
    RecordReference {
        target_object: String,
        target_record_id: String,
    },

    /// Select/Status option
    Option { option: AttioSelectOption },

    /// Raw JSON for unknown types
    Raw(Value),
}

/// Select/Status option
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AttioSelectOption {
    /// Option ID
    pub id: String,

    /// Option title
    pub title: String,
}

impl AttioRecord {
    /// Get a string value from the record
    pub fn get_string(&self, field: &str) -> Option<String> {
        self.values.get(field).and_then(|v| match v {
            AttioValue::Single(item) => item.as_string(),
            AttioValue::Multiple(items) => items.first().and_then(|i| i.as_string()),
        })
    }

    /// Get a number value from the record
    pub fn get_number(&self, field: &str) -> Option<f64> {
        self.values.get(field).and_then(|v| match v {
            AttioValue::Single(item) => item.as_number(),
            AttioValue::Multiple(items) => items.first().and_then(|i| i.as_number()),
        })
    }
}

impl AttioValueItem {
    /// Convert to string if possible
    pub fn as_string(&self) -> Option<String> {
        match &self.value {
            AttioValueType::Text { value } => Some(value.clone()),
            AttioValueType::Email { email_address, .. } => Some(email_address.clone()),
            AttioValueType::Phone { phone_number, .. } => Some(phone_number.clone()),
            AttioValueType::Domain { domain } => Some(domain.clone()),
            AttioValueType::PersonalName { full_name, .. } => full_name.clone(),
            _ => None,
        }
    }

    /// Convert to number if possible
    pub fn as_number(&self) -> Option<f64> {
        match &self.value {
            AttioValueType::Number { value } => Some(*value),
            AttioValueType::Currency { currency_value, .. } => Some(*currency_value),
            _ => None,
        }
    }
}

impl std::fmt::Display for AttioRecordId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::fmt::Display for AttioObjectId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_attio_record_id_display() {
        let id = AttioRecordId("abc123".to_string());
        assert_eq!(id.to_string(), "abc123");
    }
}
