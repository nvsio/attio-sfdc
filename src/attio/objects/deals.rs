//! Attio Deals object.

use chrono::{DateTime, Utc, NaiveDate};
use serde::{Deserialize, Serialize};

/// Typed representation of an Attio Deal record
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DealRecord {
    /// Deal name
    pub name: String,

    /// Deal value
    pub value: Option<DealValue>,

    /// Deal status
    pub status: Option<String>,

    /// Expected close date
    pub expected_close_date: Option<NaiveDate>,

    /// Actual close date
    pub close_date: Option<NaiveDate>,

    /// Win probability (0-100)
    pub probability: Option<f64>,

    /// Associated company IDs
    pub associated_companies: Vec<String>,

    /// Associated person IDs
    pub associated_people: Vec<String>,

    /// Deal owner (user ID)
    pub owner_id: Option<String>,

    /// Deal source
    pub source: Option<String>,

    /// Notes
    pub notes: Option<String>,
}

/// Deal monetary value
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DealValue {
    /// Numeric value
    pub value: f64,

    /// Currency code (e.g., "USD", "EUR")
    pub currency_code: String,
}

impl DealRecord {
    /// Get value in USD (assumes value is already in USD if no conversion needed)
    pub fn value_usd(&self) -> Option<f64> {
        self.value.as_ref().map(|v| v.value)
    }

    /// Check if deal is closed (won or lost)
    pub fn is_closed(&self) -> bool {
        self.status
            .as_ref()
            .map(|s| s == "won" || s == "lost")
            .unwrap_or(false)
    }

    /// Check if deal is won
    pub fn is_won(&self) -> bool {
        self.status.as_ref().map(|s| s == "won").unwrap_or(false)
    }

    /// Get primary company ID
    pub fn primary_company_id(&self) -> Option<&str> {
        self.associated_companies.first().map(|s| s.as_str())
    }

    /// Get primary person ID
    pub fn primary_person_id(&self) -> Option<&str> {
        self.associated_people.first().map(|s| s.as_str())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_closed() {
        let mut deal = DealRecord {
            name: "Test Deal".to_string(),
            value: None,
            status: Some("won".to_string()),
            expected_close_date: None,
            close_date: None,
            probability: None,
            associated_companies: vec![],
            associated_people: vec![],
            owner_id: None,
            source: None,
            notes: None,
        };

        assert!(deal.is_closed());
        assert!(deal.is_won());

        deal.status = Some("in_progress".to_string());
        assert!(!deal.is_closed());
    }
}
