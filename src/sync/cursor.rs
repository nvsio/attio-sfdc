//! Sync cursor management for incremental sync.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Sync cursor tracking the last successful sync point
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyncCursor {
    /// Timestamp of last sync
    pub timestamp: DateTime<Utc>,

    /// Object-specific cursors
    pub objects: std::collections::HashMap<String, ObjectCursor>,

    /// Cursor version (for migrations)
    pub version: u32,
}

/// Cursor for a specific object type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ObjectCursor {
    /// Object identifier (e.g., "companies" or "Account")
    pub object: String,

    /// Last synced timestamp for this object
    pub last_sync: DateTime<Utc>,

    /// Last synced record ID (for pagination)
    pub last_record_id: Option<String>,

    /// Number of records synced in last batch
    pub last_batch_count: u64,
}

impl SyncCursor {
    /// Create a new cursor at the current time
    pub fn now() -> Self {
        Self {
            timestamp: Utc::now(),
            objects: std::collections::HashMap::new(),
            version: 1,
        }
    }

    /// Create a cursor from a specific timestamp
    pub fn from_timestamp(timestamp: DateTime<Utc>) -> Self {
        Self {
            timestamp,
            objects: std::collections::HashMap::new(),
            version: 1,
        }
    }

    /// Get cursor for a specific object
    pub fn get_object_cursor(&self, object: &str) -> Option<&ObjectCursor> {
        self.objects.get(object)
    }

    /// Update cursor for a specific object
    pub fn update_object_cursor(&mut self, cursor: ObjectCursor) {
        self.objects.insert(cursor.object.clone(), cursor);
        self.timestamp = Utc::now();
    }

    /// Advance the global timestamp
    pub fn advance(&mut self) {
        self.timestamp = Utc::now();
    }

    /// Serialize to JSON string
    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string(self)
    }

    /// Deserialize from JSON string
    pub fn from_json(json: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(json)
    }
}

impl ObjectCursor {
    /// Create a new object cursor
    pub fn new(object: impl Into<String>) -> Self {
        Self {
            object: object.into(),
            last_sync: Utc::now(),
            last_record_id: None,
            last_batch_count: 0,
        }
    }

    /// Update with sync results
    pub fn update(&mut self, last_record_id: Option<String>, batch_count: u64) {
        self.last_sync = Utc::now();
        self.last_record_id = last_record_id;
        self.last_batch_count = batch_count;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cursor_serialization() {
        let cursor = SyncCursor::now();
        let json = cursor.to_json().unwrap();
        let parsed = SyncCursor::from_json(&json).unwrap();
        assert_eq!(parsed.version, cursor.version);
    }

    #[test]
    fn test_object_cursor() {
        let mut cursor = SyncCursor::now();
        let obj_cursor = ObjectCursor::new("companies");
        cursor.update_object_cursor(obj_cursor);

        assert!(cursor.get_object_cursor("companies").is_some());
        assert!(cursor.get_object_cursor("people").is_none());
    }
}
