//! Attio webhook handling.

use crate::error::{Error, Result};
use hmac::{Hmac, Mac};
use serde::{Deserialize, Serialize};
use sha2::Sha256;

type HmacSha256 = Hmac<Sha256>;

/// Attio webhook event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AttioWebhookEvent {
    /// Event type
    pub event_type: AttioEventType,

    /// Workspace ID
    pub workspace_id: String,

    /// Object type (e.g., "companies", "people")
    pub object: String,

    /// Record ID that was affected
    pub record_id: String,

    /// Event timestamp
    pub timestamp: String,

    /// Actor who triggered the event
    pub actor: Option<AttioActor>,

    /// Previous values (for updates)
    pub previous_values: Option<serde_json::Value>,

    /// New values
    pub new_values: Option<serde_json::Value>,
}

/// Attio event types
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum AttioEventType {
    /// Record created
    RecordCreated,
    /// Record updated
    RecordUpdated,
    /// Record deleted
    RecordDeleted,
    /// Record merged
    RecordMerged,
    /// List entry created
    ListEntryCreated,
    /// List entry updated
    ListEntryUpdated,
    /// List entry deleted
    ListEntryDeleted,
    /// Unknown event type
    #[serde(other)]
    Unknown,
}

/// Actor who triggered an event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AttioActor {
    /// Actor type
    #[serde(rename = "type")]
    pub actor_type: String,

    /// Actor ID
    pub id: Option<String>,
}

/// Verify webhook signature
pub fn verify_signature(payload: &[u8], signature: &str, secret: &str) -> Result<()> {
    let mut mac = HmacSha256::new_from_slice(secret.as_bytes())
        .map_err(|_| Error::Internal {
            message: "Invalid webhook secret".to_string(),
        })?;

    mac.update(payload);

    let expected = mac.finalize().into_bytes();
    let expected_hex = hex_encode(&expected);

    // Signature format is typically "sha256=<hex>"
    let provided = signature.strip_prefix("sha256=").unwrap_or(signature);

    if constant_time_compare(expected_hex.as_bytes(), provided.as_bytes()) {
        Ok(())
    } else {
        Err(Error::WebhookSignature)
    }
}

/// Parse webhook payload
pub fn parse_webhook(payload: &[u8]) -> Result<AttioWebhookEvent> {
    serde_json::from_slice(payload).map_err(Error::from)
}

/// Constant-time comparison to prevent timing attacks
fn constant_time_compare(a: &[u8], b: &[u8]) -> bool {
    if a.len() != b.len() {
        return false;
    }

    let mut result = 0u8;
    for (x, y) in a.iter().zip(b.iter()) {
        result |= x ^ y;
    }
    result == 0
}

/// Encode bytes as hex string
fn hex_encode(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{:02x}", b)).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_webhook() {
        let payload = r#"{
            "event_type": "record_created",
            "workspace_id": "ws_123",
            "object": "companies",
            "record_id": "rec_456",
            "timestamp": "2024-01-01T00:00:00Z"
        }"#;

        let event = parse_webhook(payload.as_bytes()).unwrap();
        assert_eq!(event.event_type, AttioEventType::RecordCreated);
        assert_eq!(event.object, "companies");
    }

    #[test]
    fn test_constant_time_compare() {
        assert!(constant_time_compare(b"hello", b"hello"));
        assert!(!constant_time_compare(b"hello", b"world"));
        assert!(!constant_time_compare(b"hello", b"hell"));
    }
}
