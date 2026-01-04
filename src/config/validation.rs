//! Configuration validation.

use crate::config::Config;
use crate::error::{Error, Result};

/// Validate the configuration
pub fn validate_config(config: &Config) -> Result<()> {
    // Validate Attio API key format
    if config.attio.api_key.is_empty() {
        return Err(Error::config("Attio API key cannot be empty"));
    }

    // Validate Salesforce configuration
    if config.salesforce.client_id.is_empty() {
        return Err(Error::config("Salesforce client ID cannot be empty"));
    }

    if config.salesforce.client_secret.is_empty() {
        return Err(Error::config("Salesforce client secret cannot be empty"));
    }

    if config.salesforce.instance_url.is_empty() {
        return Err(Error::config("Salesforce instance URL cannot be empty"));
    }

    // Validate instance URL format
    if !config.salesforce.instance_url.starts_with("https://") {
        return Err(Error::config(
            "Salesforce instance URL must start with https://",
        ));
    }

    // Validate batch size
    if config.sync.batch_size == 0 {
        return Err(Error::config("Batch size must be greater than 0"));
    }

    if config.sync.batch_size > 10000 {
        return Err(Error::config("Batch size cannot exceed 10000"));
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::{AttioConfig, SalesforceConfig, SyncConfig};
    use std::collections::HashMap;

    fn valid_config() -> Config {
        Config {
            sync: SyncConfig::default(),
            attio: AttioConfig {
                api_key: "test_key".to_string(),
                webhook_secret: None,
                base_url: "https://api.attio.com".to_string(),
            },
            salesforce: SalesforceConfig {
                client_id: "client_id".to_string(),
                client_secret: "client_secret".to_string(),
                instance_url: "https://test.salesforce.com".to_string(),
                refresh_token: None,
                api_version: "v59.0".to_string(),
            },
            mappings: HashMap::new(),
        }
    }

    #[test]
    fn test_valid_config() {
        let config = valid_config();
        assert!(validate_config(&config).is_ok());
    }

    #[test]
    fn test_empty_attio_key() {
        let mut config = valid_config();
        config.attio.api_key = String::new();
        assert!(validate_config(&config).is_err());
    }

    #[test]
    fn test_invalid_salesforce_url() {
        let mut config = valid_config();
        config.salesforce.instance_url = "http://test.salesforce.com".to_string();
        assert!(validate_config(&config).is_err());
    }
}
