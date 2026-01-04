//! Conflict detection and resolution.

use crate::config::ConflictResolution;
use crate::error::{Error, Result};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// A record with conflict between Attio and Salesforce
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConflictRecord {
    /// Conflict ID
    pub id: String,

    /// Object type (Attio side)
    pub attio_object: String,

    /// Attio record ID
    pub attio_record_id: String,

    /// Salesforce object type
    pub salesforce_object: String,

    /// Salesforce record ID
    pub salesforce_record_id: String,

    /// Attio record data
    pub attio_data: Value,

    /// Salesforce record data
    pub salesforce_data: Value,

    /// Fields with conflicts
    pub conflicting_fields: Vec<FieldConflict>,

    /// When the conflict was detected
    pub detected_at: DateTime<Utc>,

    /// Resolution status
    pub status: ConflictStatus,

    /// How it was resolved (if resolved)
    pub resolution: Option<ConflictResolutionResult>,
}

/// Conflict in a specific field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FieldConflict {
    /// Field name (Attio side)
    pub attio_field: String,

    /// Field name (Salesforce side)
    pub salesforce_field: String,

    /// Attio value
    pub attio_value: Value,

    /// Salesforce value
    pub salesforce_value: Value,

    /// Attio last modified
    pub attio_modified_at: Option<DateTime<Utc>>,

    /// Salesforce last modified
    pub salesforce_modified_at: Option<DateTime<Utc>>,
}

/// Status of a conflict
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ConflictStatus {
    /// Pending resolution
    Pending,
    /// Resolved automatically
    AutoResolved,
    /// Resolved manually
    ManuallyResolved,
    /// Skipped/ignored
    Skipped,
}

/// Result of conflict resolution
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConflictResolutionResult {
    /// Which side won
    pub winner: ConflictWinner,

    /// Resolved at
    pub resolved_at: DateTime<Utc>,

    /// Resolved by (user ID or "system")
    pub resolved_by: String,

    /// Notes
    pub notes: Option<String>,
}

/// Which side won the conflict
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ConflictWinner {
    Attio,
    Salesforce,
    Merged,
    Neither,
}

/// Conflict resolver
pub struct ConflictResolver {
    strategy: ConflictResolution,
}

impl ConflictResolver {
    /// Create a new conflict resolver
    pub fn new(strategy: ConflictResolution) -> Self {
        Self { strategy }
    }

    /// Check if two values conflict
    pub fn detect_conflict(&self, attio_value: &Value, sf_value: &Value) -> bool {
        // Simple equality check - could be enhanced with fuzzy matching
        attio_value != sf_value
    }

    /// Resolve a conflict based on strategy
    pub fn resolve(&self, conflict: &ConflictRecord) -> Result<ConflictResolutionResult> {
        match self.strategy {
            ConflictResolution::LastWrite => self.resolve_last_write(conflict),
            ConflictResolution::AttioWins => Ok(ConflictResolutionResult {
                winner: ConflictWinner::Attio,
                resolved_at: Utc::now(),
                resolved_by: "system".to_string(),
                notes: Some("Attio wins strategy".to_string()),
            }),
            ConflictResolution::SalesforceWins => Ok(ConflictResolutionResult {
                winner: ConflictWinner::Salesforce,
                resolved_at: Utc::now(),
                resolved_by: "system".to_string(),
                notes: Some("Salesforce wins strategy".to_string()),
            }),
            ConflictResolution::Manual => Err(Error::conflict(
                &conflict.attio_object,
                &conflict.attio_record_id,
                "Manual resolution required",
            )),
        }
    }

    /// Resolve using last-write-wins strategy
    fn resolve_last_write(&self, conflict: &ConflictRecord) -> Result<ConflictResolutionResult> {
        // Look at the most recent modification across all conflicting fields
        let attio_latest = conflict
            .conflicting_fields
            .iter()
            .filter_map(|f| f.attio_modified_at)
            .max();

        let sf_latest = conflict
            .conflicting_fields
            .iter()
            .filter_map(|f| f.salesforce_modified_at)
            .max();

        let winner = match (attio_latest, sf_latest) {
            (Some(a), Some(s)) if a > s => ConflictWinner::Attio,
            (Some(a), Some(s)) if s > a => ConflictWinner::Salesforce,
            (Some(_), None) => ConflictWinner::Attio,
            (None, Some(_)) => ConflictWinner::Salesforce,
            _ => ConflictWinner::Attio, // Default to Attio if timestamps are equal or missing
        };

        Ok(ConflictResolutionResult {
            winner,
            resolved_at: Utc::now(),
            resolved_by: "system".to_string(),
            notes: Some("Last write wins strategy".to_string()),
        })
    }

    /// Get merged values (field-level last-write-wins)
    pub fn merge_values(&self, conflict: &ConflictRecord) -> Value {
        let mut merged = conflict.attio_data.clone();

        if let (Some(attio_obj), Some(sf_obj)) =
            (merged.as_object_mut(), conflict.salesforce_data.as_object())
        {
            for field_conflict in &conflict.conflicting_fields {
                // Determine winner for this specific field
                let use_sf = match (
                    field_conflict.attio_modified_at,
                    field_conflict.salesforce_modified_at,
                ) {
                    (Some(a), Some(s)) => s > a,
                    (None, Some(_)) => true,
                    _ => false,
                };

                if use_sf {
                    if let Some(sf_value) = sf_obj.get(&field_conflict.salesforce_field) {
                        attio_obj.insert(
                            field_conflict.attio_field.clone(),
                            sf_value.clone(),
                        );
                    }
                }
            }
        }

        merged
    }
}

impl ConflictRecord {
    /// Create a new conflict record
    pub fn new(
        attio_object: impl Into<String>,
        attio_record_id: impl Into<String>,
        salesforce_object: impl Into<String>,
        salesforce_record_id: impl Into<String>,
        attio_data: Value,
        salesforce_data: Value,
    ) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            attio_object: attio_object.into(),
            attio_record_id: attio_record_id.into(),
            salesforce_object: salesforce_object.into(),
            salesforce_record_id: salesforce_record_id.into(),
            attio_data,
            salesforce_data,
            conflicting_fields: vec![],
            detected_at: Utc::now(),
            status: ConflictStatus::Pending,
            resolution: None,
        }
    }

    /// Add a field conflict
    pub fn add_field_conflict(&mut self, conflict: FieldConflict) {
        self.conflicting_fields.push(conflict);
    }

    /// Mark as resolved
    pub fn resolve(&mut self, resolution: ConflictResolutionResult) {
        self.status = ConflictStatus::AutoResolved;
        self.resolution = Some(resolution);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_detect_conflict() {
        let resolver = ConflictResolver::new(ConflictResolution::LastWrite);

        assert!(resolver.detect_conflict(
            &serde_json::json!("value1"),
            &serde_json::json!("value2")
        ));

        assert!(!resolver.detect_conflict(
            &serde_json::json!("same"),
            &serde_json::json!("same")
        ));
    }

    #[test]
    fn test_attio_wins() {
        let resolver = ConflictResolver::new(ConflictResolution::AttioWins);
        let conflict = ConflictRecord::new(
            "companies",
            "rec_1",
            "Account",
            "001xxx",
            serde_json::json!({}),
            serde_json::json!({}),
        );

        let result = resolver.resolve(&conflict).unwrap();
        assert_eq!(result.winner, ConflictWinner::Attio);
    }
}
