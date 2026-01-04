//! Error types for the attio-sfdc library.

use thiserror::Error;

/// Result type alias using our Error type
pub type Result<T> = std::result::Result<T, Error>;

/// Main error type for attio-sfdc operations
#[derive(Error, Debug)]
pub enum Error {
    /// Attio API errors
    #[error("Attio API error: {operation} - {message}")]
    AttioApi {
        operation: &'static str,
        message: String,
    },

    /// Salesforce API errors
    #[error("Salesforce API error: {operation} - {message}")]
    SalesforceApi {
        operation: &'static str,
        message: String,
    },

    /// OAuth authentication errors
    #[error("OAuth error: {message}")]
    OAuth { message: String },

    /// Configuration errors
    #[error("Configuration error: {message}")]
    Config { message: String },

    /// Mapping errors
    #[error("Mapping error: {message}")]
    Mapping { message: String },

    /// Transform errors
    #[error("Transform error for field '{field}': {message}")]
    Transform { field: String, message: String },

    /// Sync errors
    #[error("Sync error: {message}")]
    Sync { message: String },

    /// Conflict detected during sync
    #[error("Conflict detected for {object_type}/{record_id}: {message}")]
    Conflict {
        object_type: String,
        record_id: String,
        message: String,
    },

    /// Storage errors
    #[error("Storage error: {message}")]
    Storage { message: String },

    /// Rate limit exceeded
    #[error("Rate limit exceeded for {service}, retry after {retry_after_secs}s")]
    RateLimit {
        service: String,
        retry_after_secs: u64,
    },

    /// Record not found
    #[error("{object_type} record not found: {record_id}")]
    NotFound {
        object_type: String,
        record_id: String,
    },

    /// Validation errors
    #[error("Validation error: {message}")]
    Validation { message: String },

    /// Serialization errors
    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),

    /// HTTP errors
    #[error("HTTP error: {0}")]
    Http(String),

    /// Webhook signature verification failed
    #[error("Webhook signature verification failed")]
    WebhookSignature,

    /// Internal errors
    #[error("Internal error: {message}")]
    Internal { message: String },
}

impl Error {
    /// Create an Attio API error
    pub fn attio_api(operation: &'static str, message: impl Into<String>) -> Self {
        Self::AttioApi {
            operation,
            message: message.into(),
        }
    }

    /// Create a Salesforce API error
    pub fn salesforce_api(operation: &'static str, message: impl Into<String>) -> Self {
        Self::SalesforceApi {
            operation,
            message: message.into(),
        }
    }

    /// Create a config error
    pub fn config(message: impl Into<String>) -> Self {
        Self::Config {
            message: message.into(),
        }
    }

    /// Create a mapping error
    pub fn mapping(message: impl Into<String>) -> Self {
        Self::Mapping {
            message: message.into(),
        }
    }

    /// Create a transform error
    pub fn transform(field: impl Into<String>, message: impl Into<String>) -> Self {
        Self::Transform {
            field: field.into(),
            message: message.into(),
        }
    }

    /// Create a sync error
    pub fn sync(message: impl Into<String>) -> Self {
        Self::Sync {
            message: message.into(),
        }
    }

    /// Create a conflict error
    pub fn conflict(
        object_type: impl Into<String>,
        record_id: impl Into<String>,
        message: impl Into<String>,
    ) -> Self {
        Self::Conflict {
            object_type: object_type.into(),
            record_id: record_id.into(),
            message: message.into(),
        }
    }

    /// Create a not found error
    pub fn not_found(object_type: impl Into<String>, record_id: impl Into<String>) -> Self {
        Self::NotFound {
            object_type: object_type.into(),
            record_id: record_id.into(),
        }
    }

    /// Create a validation error
    pub fn validation(message: impl Into<String>) -> Self {
        Self::Validation {
            message: message.into(),
        }
    }

    /// Create a rate limit error
    pub fn rate_limit(service: impl Into<String>, retry_after_secs: u64) -> Self {
        Self::RateLimit {
            service: service.into(),
            retry_after_secs,
        }
    }

    /// Check if this is a retryable error
    pub fn is_retryable(&self) -> bool {
        matches!(
            self,
            Self::RateLimit { .. }
                | Self::Http(_)
                | Self::AttioApi { .. }
                | Self::SalesforceApi { .. }
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_display() {
        let err = Error::attio_api("get_record", "Record not found");
        assert_eq!(
            err.to_string(),
            "Attio API error: get_record - Record not found"
        );
    }

    #[test]
    fn test_is_retryable() {
        assert!(Error::rate_limit("attio", 60).is_retryable());
        assert!(Error::Http("timeout".into()).is_retryable());
        assert!(!Error::config("invalid").is_retryable());
    }
}
