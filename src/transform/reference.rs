//! Reference resolution for foreign key relationships.

use crate::error::{Error, Result};
use std::collections::HashMap;

/// Reference resolver for looking up cross-system IDs
pub struct ReferenceResolver {
    /// Attio ID → Salesforce ID mappings
    attio_to_sf: HashMap<IdMappingKey, String>,

    /// Salesforce ID → Attio ID mappings
    sf_to_attio: HashMap<IdMappingKey, String>,
}

/// Key for ID mapping lookups
#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct IdMappingKey {
    /// Object type (e.g., "companies" or "Account")
    pub object: String,

    /// Record ID
    pub id: String,
}

/// ID mapping entry
#[derive(Debug, Clone)]
pub struct IdMapping {
    /// Attio object type
    pub attio_object: String,

    /// Attio record ID
    pub attio_id: String,

    /// Salesforce object type
    pub salesforce_object: String,

    /// Salesforce record ID
    pub salesforce_id: String,
}

impl ReferenceResolver {
    /// Create a new reference resolver
    pub fn new() -> Self {
        Self {
            attio_to_sf: HashMap::new(),
            sf_to_attio: HashMap::new(),
        }
    }

    /// Load mappings from storage
    pub fn load_mappings(&mut self, mappings: Vec<IdMapping>) {
        for mapping in mappings {
            let attio_key = IdMappingKey {
                object: mapping.attio_object.clone(),
                id: mapping.attio_id.clone(),
            };

            let sf_key = IdMappingKey {
                object: mapping.salesforce_object.clone(),
                id: mapping.salesforce_id.clone(),
            };

            self.attio_to_sf
                .insert(attio_key, mapping.salesforce_id.clone());
            self.sf_to_attio.insert(sf_key, mapping.attio_id);
        }
    }

    /// Add a single mapping
    pub fn add_mapping(&mut self, mapping: IdMapping) {
        let attio_key = IdMappingKey {
            object: mapping.attio_object.clone(),
            id: mapping.attio_id.clone(),
        };

        let sf_key = IdMappingKey {
            object: mapping.salesforce_object.clone(),
            id: mapping.salesforce_id.clone(),
        };

        self.attio_to_sf
            .insert(attio_key, mapping.salesforce_id.clone());
        self.sf_to_attio.insert(sf_key, mapping.attio_id);
    }

    /// Resolve Attio ID to Salesforce ID
    pub fn attio_to_salesforce(&self, object: &str, attio_id: &str) -> Option<&str> {
        let key = IdMappingKey {
            object: object.to_string(),
            id: attio_id.to_string(),
        };
        self.attio_to_sf.get(&key).map(|s| s.as_str())
    }

    /// Resolve Salesforce ID to Attio ID
    pub fn salesforce_to_attio(&self, object: &str, sf_id: &str) -> Option<&str> {
        let key = IdMappingKey {
            object: object.to_string(),
            id: sf_id.to_string(),
        };
        self.sf_to_attio.get(&key).map(|s| s.as_str())
    }

    /// Require Attio to Salesforce resolution (error if not found)
    pub fn require_attio_to_salesforce(&self, object: &str, attio_id: &str) -> Result<&str> {
        self.attio_to_salesforce(object, attio_id)
            .ok_or_else(|| Error::not_found(format!("{}/{}", object, attio_id), "ID mapping"))
    }

    /// Require Salesforce to Attio resolution (error if not found)
    pub fn require_salesforce_to_attio(&self, object: &str, sf_id: &str) -> Result<&str> {
        self.salesforce_to_attio(object, sf_id)
            .ok_or_else(|| Error::not_found(format!("{}/{}", object, sf_id), "ID mapping"))
    }

    /// Check if an Attio ID has a Salesforce mapping
    pub fn has_attio_mapping(&self, object: &str, attio_id: &str) -> bool {
        self.attio_to_salesforce(object, attio_id).is_some()
    }

    /// Check if a Salesforce ID has an Attio mapping
    pub fn has_salesforce_mapping(&self, object: &str, sf_id: &str) -> bool {
        self.salesforce_to_attio(object, sf_id).is_some()
    }

    /// Get all mappings (for persistence)
    pub fn get_all_mappings(&self) -> Vec<IdMapping> {
        // This is a simplified implementation - in practice, we'd need
        // to track both sides properly
        vec![]
    }
}

impl Default for ReferenceResolver {
    fn default() -> Self {
        Self::new()
    }
}

impl IdMapping {
    /// Create a new ID mapping
    pub fn new(
        attio_object: impl Into<String>,
        attio_id: impl Into<String>,
        salesforce_object: impl Into<String>,
        salesforce_id: impl Into<String>,
    ) -> Self {
        Self {
            attio_object: attio_object.into(),
            attio_id: attio_id.into(),
            salesforce_object: salesforce_object.into(),
            salesforce_id: salesforce_id.into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reference_resolver() {
        let mut resolver = ReferenceResolver::new();

        resolver.add_mapping(IdMapping::new(
            "companies",
            "rec_123",
            "Account",
            "001000000000001",
        ));

        assert_eq!(
            resolver.attio_to_salesforce("companies", "rec_123"),
            Some("001000000000001")
        );

        assert_eq!(
            resolver.salesforce_to_attio("Account", "001000000000001"),
            Some("rec_123")
        );

        assert!(resolver.attio_to_salesforce("companies", "rec_unknown").is_none());
    }

    #[test]
    fn test_has_mapping() {
        let mut resolver = ReferenceResolver::new();
        resolver.add_mapping(IdMapping::new(
            "people",
            "rec_456",
            "Contact",
            "003000000000001",
        ));

        assert!(resolver.has_attio_mapping("people", "rec_456"));
        assert!(!resolver.has_attio_mapping("people", "rec_unknown"));
    }
}
