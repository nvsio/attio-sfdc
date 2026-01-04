//! Configuration management for attio-sfdc.

pub mod mappings;
mod validation;

pub use mappings::{FieldMapping, ObjectMapping, TransformType, DEFAULT_MAPPINGS};
pub use validation::validate_config;

use crate::error::{Error, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Sync direction configuration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum SyncDirection {
    /// Sync from Attio to Salesforce only
    AttioToSalesforce,
    /// Sync from Salesforce to Attio only
    SalesforceToAttio,
    /// Bidirectional sync (default)
    #[default]
    Bidirectional,
}

/// Conflict resolution strategy
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum ConflictResolution {
    /// Most recent write wins (based on updated_at timestamps)
    #[default]
    LastWrite,
    /// Attio is always the source of truth
    AttioWins,
    /// Salesforce is always the source of truth
    SalesforceWins,
    /// Queue conflicts for manual resolution
    Manual,
}

/// Main configuration struct
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    /// Sync configuration
    pub sync: SyncConfig,

    /// Attio API configuration
    pub attio: AttioConfig,

    /// Salesforce API configuration
    pub salesforce: SalesforceConfig,

    /// Object mappings (overrides defaults)
    #[serde(default)]
    pub mappings: HashMap<String, ObjectMapping>,
}

/// Sync-specific configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyncConfig {
    /// Sync direction
    #[serde(default)]
    pub direction: SyncDirection,

    /// Batch size for bulk operations
    #[serde(default = "default_batch_size")]
    pub batch_size: usize,

    /// Conflict resolution strategy
    #[serde(default)]
    pub conflict_resolution: ConflictResolution,

    /// Enable real-time webhook sync
    #[serde(default = "default_true")]
    pub webhook_enabled: bool,

    /// Enable scheduled sync
    #[serde(default = "default_true")]
    pub scheduled_enabled: bool,
}

/// Attio API configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AttioConfig {
    /// API key (or environment variable name)
    pub api_key: String,

    /// Webhook secret for signature verification
    pub webhook_secret: Option<String>,

    /// Base URL (defaults to production)
    #[serde(default = "default_attio_base_url")]
    pub base_url: String,
}

/// Salesforce API configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SalesforceConfig {
    /// OAuth client ID
    pub client_id: String,

    /// OAuth client secret
    pub client_secret: String,

    /// Salesforce instance URL
    pub instance_url: String,

    /// OAuth refresh token (for persistent auth)
    pub refresh_token: Option<String>,

    /// API version
    #[serde(default = "default_sf_api_version")]
    pub api_version: String,
}

fn default_batch_size() -> usize {
    100
}

fn default_true() -> bool {
    true
}

fn default_attio_base_url() -> String {
    "https://api.attio.com".to_string()
}

fn default_sf_api_version() -> String {
    "v59.0".to_string()
}

impl Default for SyncConfig {
    fn default() -> Self {
        Self {
            direction: SyncDirection::default(),
            batch_size: default_batch_size(),
            conflict_resolution: ConflictResolution::default(),
            webhook_enabled: true,
            scheduled_enabled: true,
        }
    }
}

impl Config {
    /// Load configuration from environment variables
    pub fn from_env() -> Result<Self> {
        let config = Self {
            sync: SyncConfig {
                direction: Self::parse_env_or_default("SYNC_DIRECTION")?,
                batch_size: Self::parse_env_or_default("BATCH_SIZE")?,
                conflict_resolution: Self::parse_env_or_default("CONFLICT_RESOLUTION")?,
                webhook_enabled: Self::parse_env_or_default("WEBHOOK_ENABLED")?,
                scheduled_enabled: Self::parse_env_or_default("SCHEDULED_ENABLED")?,
            },
            attio: AttioConfig {
                api_key: Self::require_env("ATTIO_API_KEY")?,
                webhook_secret: Self::optional_env("ATTIO_WEBHOOK_SECRET"),
                base_url: Self::optional_env("ATTIO_BASE_URL")
                    .unwrap_or_else(default_attio_base_url),
            },
            salesforce: SalesforceConfig {
                client_id: Self::require_env("SALESFORCE_CLIENT_ID")?,
                client_secret: Self::require_env("SALESFORCE_CLIENT_SECRET")?,
                instance_url: Self::require_env("SALESFORCE_INSTANCE_URL")?,
                refresh_token: Self::optional_env("SALESFORCE_REFRESH_TOKEN"),
                api_version: Self::optional_env("SALESFORCE_API_VERSION")
                    .unwrap_or_else(default_sf_api_version),
            },
            mappings: HashMap::new(),
        };

        validate_config(&config)?;
        Ok(config)
    }

    /// Get object mapping, falling back to defaults
    pub fn get_mapping(&self, attio_object: &str, sf_object: &str) -> Option<&ObjectMapping> {
        let key = format!("{}_{}", attio_object, sf_object);
        self.mappings.get(&key)
    }

    fn require_env(name: &str) -> Result<String> {
        std::env::var(name).map_err(|_| Error::config(format!("Missing required env var: {}", name)))
    }

    fn optional_env(name: &str) -> Option<String> {
        std::env::var(name).ok()
    }

    fn parse_env_or_default<T: Default + std::str::FromStr>(name: &str) -> Result<T> {
        match std::env::var(name) {
            Ok(val) => val
                .parse()
                .map_err(|_| Error::config(format!("Invalid value for {}", name))),
            Err(_) => Ok(T::default()),
        }
    }
}

impl std::str::FromStr for SyncDirection {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        match s.to_lowercase().as_str() {
            "attio_to_salesforce" | "attio_to_sf" => Ok(Self::AttioToSalesforce),
            "salesforce_to_attio" | "sf_to_attio" => Ok(Self::SalesforceToAttio),
            "bidirectional" | "both" => Ok(Self::Bidirectional),
            _ => Err(Error::config(format!("Invalid sync direction: {}", s))),
        }
    }
}

impl std::str::FromStr for ConflictResolution {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        match s.to_lowercase().as_str() {
            "last_write" | "lastwrite" => Ok(Self::LastWrite),
            "attio_wins" | "attiowins" => Ok(Self::AttioWins),
            "salesforce_wins" | "salesforcewins" | "sf_wins" => Ok(Self::SalesforceWins),
            "manual" => Ok(Self::Manual),
            _ => Err(Error::config(format!(
                "Invalid conflict resolution: {}",
                s
            ))),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sync_direction_from_str() {
        assert_eq!(
            "bidirectional".parse::<SyncDirection>().unwrap(),
            SyncDirection::Bidirectional
        );
        assert_eq!(
            "attio_to_sf".parse::<SyncDirection>().unwrap(),
            SyncDirection::AttioToSalesforce
        );
    }

    #[test]
    fn test_conflict_resolution_from_str() {
        assert_eq!(
            "last_write".parse::<ConflictResolution>().unwrap(),
            ConflictResolution::LastWrite
        );
        assert_eq!(
            "manual".parse::<ConflictResolution>().unwrap(),
            ConflictResolution::Manual
        );
    }
}
