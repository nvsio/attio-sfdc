//! Sync direction handling.

use crate::config::SyncDirection;

/// Handler for sync direction logic
pub struct SyncDirectionHandler {
    direction: SyncDirection,
}

impl SyncDirectionHandler {
    /// Create a new direction handler
    pub fn new(direction: SyncDirection) -> Self {
        Self { direction }
    }

    /// Check if Attio to Salesforce sync is enabled
    pub fn attio_to_sf_enabled(&self) -> bool {
        matches!(
            self.direction,
            SyncDirection::AttioToSalesforce | SyncDirection::Bidirectional
        )
    }

    /// Check if Salesforce to Attio sync is enabled
    pub fn sf_to_attio_enabled(&self) -> bool {
        matches!(
            self.direction,
            SyncDirection::SalesforceToAttio | SyncDirection::Bidirectional
        )
    }

    /// Check if bidirectional sync is enabled
    pub fn is_bidirectional(&self) -> bool {
        matches!(self.direction, SyncDirection::Bidirectional)
    }

    /// Get the current direction
    pub fn direction(&self) -> SyncDirection {
        self.direction
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bidirectional() {
        let handler = SyncDirectionHandler::new(SyncDirection::Bidirectional);
        assert!(handler.attio_to_sf_enabled());
        assert!(handler.sf_to_attio_enabled());
        assert!(handler.is_bidirectional());
    }

    #[test]
    fn test_attio_to_sf_only() {
        let handler = SyncDirectionHandler::new(SyncDirection::AttioToSalesforce);
        assert!(handler.attio_to_sf_enabled());
        assert!(!handler.sf_to_attio_enabled());
        assert!(!handler.is_bidirectional());
    }

    #[test]
    fn test_sf_to_attio_only() {
        let handler = SyncDirectionHandler::new(SyncDirection::SalesforceToAttio);
        assert!(!handler.attio_to_sf_enabled());
        assert!(handler.sf_to_attio_enabled());
        assert!(!handler.is_bidirectional());
    }
}
