//! Storage abstraction for persisting sync state.

mod d1;
mod kv;
mod memory;

pub use d1::D1Storage;
pub use kv::KVStorage;
pub use memory::MemoryStorage;

use crate::error::Result;
use crate::sync::cursor::SyncCursor;
use crate::transform::reference::IdMapping;
use async_trait::async_trait;

/// Storage trait for persistence operations
#[async_trait(?Send)]
pub trait Storage {
    /// Save an ID mapping
    async fn save_id_mapping(&self, mapping: &IdMapping) -> Result<()>;

    /// Get ID mapping by Attio ID
    async fn get_mapping_by_attio_id(
        &self,
        object: &str,
        attio_id: &str,
    ) -> Result<Option<IdMapping>>;

    /// Get ID mapping by Salesforce ID
    async fn get_mapping_by_sf_id(
        &self,
        object: &str,
        sf_id: &str,
    ) -> Result<Option<IdMapping>>;

    /// Save sync cursor
    async fn save_cursor(&self, key: &str, cursor: &SyncCursor) -> Result<()>;

    /// Get sync cursor
    async fn get_cursor(&self, key: &str) -> Result<Option<SyncCursor>>;

    /// Delete ID mapping
    async fn delete_mapping(&self, attio_object: &str, attio_id: &str) -> Result<()>;
}
