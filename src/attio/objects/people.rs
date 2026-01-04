//! Attio People object.

use serde::{Deserialize, Serialize};
use super::companies::Location;

/// Typed representation of an Attio Person record
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PersonRecord {
    /// Full name
    pub name: Option<PersonName>,

    /// Email addresses
    pub email_addresses: Vec<EmailAddress>,

    /// Phone numbers
    pub phone_numbers: Vec<PhoneNumber>,

    /// Job title
    pub job_title: Option<String>,

    /// Primary location
    pub primary_location: Option<Location>,

    /// Associated company ID
    pub company_id: Option<String>,

    /// LinkedIn URL
    pub linkedin_url: Option<String>,

    /// Twitter handle
    pub twitter_handle: Option<String>,
}

/// Person name struct
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PersonName {
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub full_name: Option<String>,
}

/// Email address
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmailAddress {
    pub email_address: String,
    #[serde(default)]
    pub is_primary: bool,
}

/// Phone number
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PhoneNumber {
    pub phone_number: String,
    #[serde(default)]
    pub is_primary: bool,
}

impl PersonRecord {
    /// Get primary email address
    pub fn primary_email(&self) -> Option<&str> {
        self.email_addresses
            .iter()
            .find(|e| e.is_primary)
            .or_else(|| self.email_addresses.first())
            .map(|e| e.email_address.as_str())
    }

    /// Get primary phone number
    pub fn primary_phone(&self) -> Option<&str> {
        self.phone_numbers
            .iter()
            .find(|p| p.is_primary)
            .or_else(|| self.phone_numbers.first())
            .map(|p| p.phone_number.as_str())
    }

    /// Get full name
    pub fn full_name(&self) -> Option<String> {
        self.name.as_ref().and_then(|n| {
            n.full_name.clone().or_else(|| {
                match (&n.first_name, &n.last_name) {
                    (Some(first), Some(last)) => Some(format!("{} {}", first, last)),
                    (Some(first), None) => Some(first.clone()),
                    (None, Some(last)) => Some(last.clone()),
                    (None, None) => None,
                }
            })
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_primary_email() {
        let person = PersonRecord {
            name: None,
            email_addresses: vec![
                EmailAddress {
                    email_address: "secondary@example.com".to_string(),
                    is_primary: false,
                },
                EmailAddress {
                    email_address: "primary@example.com".to_string(),
                    is_primary: true,
                },
            ],
            phone_numbers: vec![],
            job_title: None,
            primary_location: None,
            company_id: None,
            linkedin_url: None,
            twitter_handle: None,
        };

        assert_eq!(person.primary_email(), Some("primary@example.com"));
    }
}
