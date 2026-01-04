//! In-memory storage implementation for testing.

use crate::error::Result;
use crate::storage::Storage;
use crate::sync::cursor::SyncCursor;
use crate::transform::reference::IdMapping;
use async_trait::async_trait;
use std::collections::HashMap;
use std::sync::RwLock;

/// In-memory storage for testing
pub struct MemoryStorage {
    mappings: RwLock<HashMap<String, IdMapping>>,
    cursors: RwLock<HashMap<String, SyncCursor>>,
}

impl MemoryStorage {
    /// Create a new in-memory storage
    pub fn new() -> Self {
        Self {
            mappings: RwLock::new(HashMap::new()),
            cursors: RwLock::new(HashMap::new()),
        }
    }

    fn mapping_key(object: &str, id: &str) -> String {
        format!("{}:{}", object, id)
    }
}

impl Default for MemoryStorage {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait(?Send)]
impl Storage for MemoryStorage {
    async fn save_id_mapping(&self, mapping: &IdMapping) -> Result<()> {
        let mut mappings = self.mappings.write().unwrap();
        let attio_key = Self::mapping_key(&mapping.attio_object, &mapping.attio_id);
        let sf_key = Self::mapping_key(&mapping.salesforce_object, &mapping.salesforce_id);

        mappings.insert(attio_key, mapping.clone());
        mappings.insert(sf_key, mapping.clone());
        Ok(())
    }

    async fn get_mapping_by_attio_id(
        &self,
        object: &str,
        attio_id: &str,
    ) -> Result<Option<IdMapping>> {
        let mappings = self.mappings.read().unwrap();
        let key = Self::mapping_key(object, attio_id);
        Ok(mappings.get(&key).cloned())
    }

    async fn get_mapping_by_sf_id(
        &self,
        object: &str,
        sf_id: &str,
    ) -> Result<Option<IdMapping>> {
        let mappings = self.mappings.read().unwrap();
        let key = Self::mapping_key(object, sf_id);
        Ok(mappings.get(&key).cloned())
    }

    async fn save_cursor(&self, key: &str, cursor: &SyncCursor) -> Result<()> {
        let mut cursors = self.cursors.write().unwrap();
        cursors.insert(key.to_string(), cursor.clone());
        Ok(())
    }

    async fn get_cursor(&self, key: &str) -> Result<Option<SyncCursor>> {
        let cursors = self.cursors.read().unwrap();
        Ok(cursors.get(key).cloned())
    }

    async fn delete_mapping(&self, attio_object: &str, attio_id: &str) -> Result<()> {
        let mut mappings = self.mappings.write().unwrap();
        let key = Self::mapping_key(attio_object, attio_id);
        mappings.remove(&key);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_memory_storage() {
        let storage = MemoryStorage::new();

        let mapping = IdMapping::new("companies", "rec_123", "Account", "001xxx");

        storage.save_id_mapping(&mapping).await.unwrap();

        let found = storage
            .get_mapping_by_attio_id("companies", "rec_123")
            .await
            .unwrap();
        assert!(found.is_some());
        assert_eq!(found.unwrap().salesforce_id, "001xxx");
    }
}
