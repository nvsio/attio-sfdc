//! Salesforce Contact object.

use serde::{Deserialize, Serialize};

/// Typed representation of a Salesforce Contact
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Contact {
    /// Record ID
    #[serde(rename = "Id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// First name
    #[serde(rename = "FirstName", skip_serializing_if = "Option::is_none")]
    pub first_name: Option<String>,

    /// Last name (required)
    #[serde(rename = "LastName")]
    pub last_name: String,

    /// Email
    #[serde(rename = "Email", skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,

    /// Phone
    #[serde(rename = "Phone", skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,

    /// Mobile phone
    #[serde(rename = "MobilePhone", skip_serializing_if = "Option::is_none")]
    pub mobile_phone: Option<String>,

    /// Title
    #[serde(rename = "Title", skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,

    /// Account ID (lookup)
    #[serde(rename = "AccountId", skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,

    /// Department
    #[serde(rename = "Department", skip_serializing_if = "Option::is_none")]
    pub department: Option<String>,

    /// Mailing address
    #[serde(rename = "MailingStreet", skip_serializing_if = "Option::is_none")]
    pub mailing_street: Option<String>,

    #[serde(rename = "MailingCity", skip_serializing_if = "Option::is_none")]
    pub mailing_city: Option<String>,

    #[serde(rename = "MailingState", skip_serializing_if = "Option::is_none")]
    pub mailing_state: Option<String>,

    #[serde(rename = "MailingPostalCode", skip_serializing_if = "Option::is_none")]
    pub mailing_postal_code: Option<String>,

    #[serde(rename = "MailingCountry", skip_serializing_if = "Option::is_none")]
    pub mailing_country: Option<String>,

    /// Lead source
    #[serde(rename = "LeadSource", skip_serializing_if = "Option::is_none")]
    pub lead_source: Option<String>,

    /// Owner ID
    #[serde(rename = "OwnerId", skip_serializing_if = "Option::is_none")]
    pub owner_id: Option<String>,
}

impl Contact {
    /// Create a new Contact with just a last name
    pub fn new(last_name: impl Into<String>) -> Self {
        Self {
            last_name: last_name.into(),
            ..Default::default()
        }
    }

    /// Create a Contact with full name
    pub fn with_name(first_name: impl Into<String>, last_name: impl Into<String>) -> Self {
        Self {
            first_name: Some(first_name.into()),
            last_name: last_name.into(),
            ..Default::default()
        }
    }

    /// Get the full name
    pub fn full_name(&self) -> String {
        match &self.first_name {
            Some(first) => format!("{} {}", first, self.last_name),
            None => self.last_name.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_contact_new() {
        let contact = Contact::new("Doe");
        assert_eq!(contact.last_name, "Doe");
        assert!(contact.first_name.is_none());
    }

    #[test]
    fn test_contact_with_name() {
        let contact = Contact::with_name("John", "Doe");
        assert_eq!(contact.full_name(), "John Doe");
    }
}
