//! # attio-sfdc
//!
//! A high-performance Rust bridge for bidirectional sync between Attio CRM and Salesforce.
//!
//! ## Features
//!
//! - Bidirectional sync between Attio and Salesforce
//! - Opinionated default mappings for standard objects
//! - Support for custom objects and fields
//! - Webhook-driven real-time sync
//! - Scheduled incremental sync
//! - Conflict resolution strategies
//! - Edge deployment on Cloudflare Workers
//!
//! ## Quick Start
//!
//! ```rust,ignore
//! use attio_sfdc::{Config, SyncEngine};
//!
//! let config = Config::from_env()?;
//! let engine = SyncEngine::new(config);
//! engine.sync_record("companies", "record_id").await?;
//! ```

pub mod attio;
pub mod config;
pub mod error;
pub mod salesforce;
pub mod storage;
pub mod sync;
pub mod transform;
pub mod worker;

pub use config::Config;
pub use error::{Error, Result};
pub use sync::SyncEngine;

/// Library version
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
