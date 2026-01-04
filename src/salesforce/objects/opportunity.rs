//! Salesforce Opportunity object.

use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

/// Typed representation of a Salesforce Opportunity
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Opportunity {
    /// Record ID
    #[serde(rename = "Id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// Opportunity name (required)
    #[serde(rename = "Name")]
    pub name: String,

    /// Stage name (required)
    #[serde(rename = "StageName")]
    pub stage_name: String,

    /// Close date (required)
    #[serde(rename = "CloseDate")]
    pub close_date: NaiveDate,

    /// Amount
    #[serde(rename = "Amount", skip_serializing_if = "Option::is_none")]
    pub amount: Option<f64>,

    /// Probability (0-100)
    #[serde(rename = "Probability", skip_serializing_if = "Option::is_none")]
    pub probability: Option<f64>,

    /// Account ID (lookup)
    #[serde(rename = "AccountId", skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,

    /// Primary contact ID
    #[serde(rename = "ContactId", skip_serializing_if = "Option::is_none")]
    pub contact_id: Option<String>,

    /// Description
    #[serde(rename = "Description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// Lead source
    #[serde(rename = "LeadSource", skip_serializing_if = "Option::is_none")]
    pub lead_source: Option<String>,

    /// Type
    #[serde(rename = "Type", skip_serializing_if = "Option::is_none")]
    pub opportunity_type: Option<String>,

    /// Next step
    #[serde(rename = "NextStep", skip_serializing_if = "Option::is_none")]
    pub next_step: Option<String>,

    /// Owner ID
    #[serde(rename = "OwnerId", skip_serializing_if = "Option::is_none")]
    pub owner_id: Option<String>,

    /// Is closed (derived)
    #[serde(rename = "IsClosed", skip_serializing_if = "Option::is_none")]
    pub is_closed: Option<bool>,

    /// Is won (derived)
    #[serde(rename = "IsWon", skip_serializing_if = "Option::is_none")]
    pub is_won: Option<bool>,

    /// Currency ISO code (for multi-currency orgs)
    #[serde(rename = "CurrencyIsoCode", skip_serializing_if = "Option::is_none")]
    pub currency_iso_code: Option<String>,
}

impl Opportunity {
    /// Create a new Opportunity with required fields
    pub fn new(
        name: impl Into<String>,
        stage_name: impl Into<String>,
        close_date: NaiveDate,
    ) -> Self {
        Self {
            name: name.into(),
            stage_name: stage_name.into(),
            close_date,
            ..Default::default()
        }
    }

    /// Check if opportunity is in a closed stage
    pub fn is_closed(&self) -> bool {
        self.is_closed.unwrap_or_else(|| {
            self.stage_name.to_lowercase().contains("closed")
        })
    }

    /// Check if opportunity is won
    pub fn is_won(&self) -> bool {
        self.is_won.unwrap_or_else(|| {
            self.stage_name.to_lowercase().contains("won")
        })
    }

    /// Calculate weighted amount
    pub fn weighted_amount(&self) -> Option<f64> {
        match (self.amount, self.probability) {
            (Some(amount), Some(prob)) => Some(amount * prob / 100.0),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_opportunity_new() {
        let opp = Opportunity::new(
            "Test Deal",
            "Prospecting",
            NaiveDate::from_ymd_opt(2024, 12, 31).unwrap(),
        );
        assert_eq!(opp.name, "Test Deal");
        assert_eq!(opp.stage_name, "Prospecting");
    }

    #[test]
    fn test_weighted_amount() {
        let mut opp = Opportunity::new(
            "Test",
            "Qualification",
            NaiveDate::from_ymd_opt(2024, 12, 31).unwrap(),
        );
        opp.amount = Some(10000.0);
        opp.probability = Some(50.0);

        assert_eq!(opp.weighted_amount(), Some(5000.0));
    }

    #[test]
    fn test_is_closed() {
        let opp = Opportunity::new(
            "Test",
            "Closed Won",
            NaiveDate::from_ymd_opt(2024, 12, 31).unwrap(),
        );
        assert!(opp.is_closed());
        assert!(opp.is_won());
    }
}
