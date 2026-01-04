//! Salesforce Account object.

use serde::{Deserialize, Serialize};

/// Typed representation of a Salesforce Account
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Account {
    /// Record ID
    #[serde(rename = "Id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// Account name (required)
    #[serde(rename = "Name")]
    pub name: String,

    /// Website
    #[serde(rename = "Website", skip_serializing_if = "Option::is_none")]
    pub website: Option<String>,

    /// Description
    #[serde(rename = "Description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// Industry
    #[serde(rename = "Industry", skip_serializing_if = "Option::is_none")]
    pub industry: Option<String>,

    /// Number of employees
    #[serde(rename = "NumberOfEmployees", skip_serializing_if = "Option::is_none")]
    pub number_of_employees: Option<i32>,

    /// Annual revenue
    #[serde(rename = "AnnualRevenue", skip_serializing_if = "Option::is_none")]
    pub annual_revenue: Option<f64>,

    /// Billing address
    #[serde(rename = "BillingStreet", skip_serializing_if = "Option::is_none")]
    pub billing_street: Option<String>,

    #[serde(rename = "BillingCity", skip_serializing_if = "Option::is_none")]
    pub billing_city: Option<String>,

    #[serde(rename = "BillingState", skip_serializing_if = "Option::is_none")]
    pub billing_state: Option<String>,

    #[serde(rename = "BillingPostalCode", skip_serializing_if = "Option::is_none")]
    pub billing_postal_code: Option<String>,

    #[serde(rename = "BillingCountry", skip_serializing_if = "Option::is_none")]
    pub billing_country: Option<String>,

    /// Phone
    #[serde(rename = "Phone", skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,

    /// Account type
    #[serde(rename = "Type", skip_serializing_if = "Option::is_none")]
    pub account_type: Option<String>,

    /// Owner ID
    #[serde(rename = "OwnerId", skip_serializing_if = "Option::is_none")]
    pub owner_id: Option<String>,

    /// Parent account ID
    #[serde(rename = "ParentId", skip_serializing_if = "Option::is_none")]
    pub parent_id: Option<String>,
}

impl Account {
    /// Create a new Account with just a name
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            ..Default::default()
        }
    }

    /// Get the formatted billing address
    pub fn billing_address(&self) -> Option<String> {
        let parts: Vec<&str> = [
            self.billing_street.as_deref(),
            self.billing_city.as_deref(),
            self.billing_state.as_deref(),
            self.billing_postal_code.as_deref(),
            self.billing_country.as_deref(),
        ]
        .into_iter()
        .flatten()
        .collect();

        if parts.is_empty() {
            None
        } else {
            Some(parts.join(", "))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_account_new() {
        let account = Account::new("Test Company");
        assert_eq!(account.name, "Test Company");
        assert!(account.id.is_none());
    }

    #[test]
    fn test_billing_address() {
        let account = Account {
            name: "Test".to_string(),
            billing_city: Some("San Francisco".to_string()),
            billing_state: Some("CA".to_string()),
            billing_country: Some("USA".to_string()),
            ..Default::default()
        };

        assert_eq!(
            account.billing_address(),
            Some("San Francisco, CA, USA".to_string())
        );
    }
}
