//! Default field mappings between Attio and Salesforce objects.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Mapping between an Attio object and a Salesforce object
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ObjectMapping {
    /// Attio object slug (e.g., "companies")
    pub attio_object: String,

    /// Salesforce object API name (e.g., "Account")
    pub salesforce_object: String,

    /// Whether this mapping is enabled
    #[serde(default = "default_true")]
    pub enabled: bool,

    /// Field mappings
    pub fields: Vec<FieldMapping>,

    /// Status/stage mappings (for Deals → Opportunity)
    #[serde(default)]
    pub status_mappings: HashMap<String, String>,
}

/// Mapping between an Attio field and a Salesforce field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FieldMapping {
    /// Attio attribute slug or path (e.g., "name" or "primary_location.locality")
    pub attio_field: String,

    /// Salesforce field API name (e.g., "Name" or "BillingCity")
    pub salesforce_field: String,

    /// Transform to apply (default: direct copy)
    #[serde(default)]
    pub transform: TransformType,

    /// Whether this is a required field
    #[serde(default)]
    pub required: bool,

    /// Sync direction for this specific field
    #[serde(default)]
    pub direction: FieldSyncDirection,
}

/// Type of transformation to apply during mapping
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum TransformType {
    /// Direct copy with no transformation
    #[default]
    Direct,

    /// Extract first element from array
    ExtractFirst,

    /// Extract nested field by path
    ExtractNested { path: String },

    /// Map value using a lookup table
    MapValue { mappings: HashMap<String, String> },

    /// Convert currency to number
    CurrencyToNumber,

    /// Convert country code to country name
    CountryCodeToName,

    /// Parse employee range to number (e.g., "11-50" → 30)
    EmployeeRangeToNumber,

    /// Custom JavaScript/WASM function (future)
    Custom { function_name: String },
}

/// Sync direction for a specific field
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum FieldSyncDirection {
    /// Sync in both directions (default)
    #[default]
    Bidirectional,

    /// Only sync from Attio to Salesforce
    AttioToSalesforce,

    /// Only sync from Salesforce to Attio
    SalesforceToAttio,

    /// Never sync this field
    None,
}

fn default_true() -> bool {
    true
}

/// Default mappings for standard objects
pub static DEFAULT_MAPPINGS: &[(&str, &str, &[(&str, &str, &str)])] = &[
    // Companies → Account
    (
        "companies",
        "Account",
        &[
            ("name", "Name", "direct"),
            ("domains[0].domain", "Website", "extract_first"),
            ("description", "Description", "direct"),
            ("primary_location.locality", "BillingCity", "extract_nested"),
            ("primary_location.region", "BillingState", "extract_nested"),
            (
                "primary_location.country_code",
                "BillingCountry",
                "country_code_to_name",
            ),
            (
                "primary_location.postcode",
                "BillingPostalCode",
                "extract_nested",
            ),
            ("categories", "Industry", "map_value"),
            (
                "employee_range",
                "NumberOfEmployees",
                "employee_range_to_number",
            ),
            ("estimated_arr_usd", "AnnualRevenue", "currency_to_number"),
        ],
    ),
    // People → Contact
    (
        "people",
        "Contact",
        &[
            ("name.first_name", "FirstName", "extract_nested"),
            ("name.last_name", "LastName", "extract_nested"),
            (
                "email_addresses[0].email_address",
                "Email",
                "extract_first",
            ),
            ("phone_numbers[0].phone_number", "Phone", "extract_first"),
            ("job_title", "Title", "direct"),
            (
                "primary_location.locality",
                "MailingCity",
                "extract_nested",
            ),
            (
                "primary_location.region",
                "MailingState",
                "extract_nested",
            ),
            (
                "primary_location.country_code",
                "MailingCountry",
                "country_code_to_name",
            ),
            // Note: AccountId requires reference resolution via ID mapping
        ],
    ),
    // Deals → Opportunity
    (
        "deals",
        "Opportunity",
        &[
            ("name", "Name", "direct"),
            ("value.value", "Amount", "currency_to_number"),
            ("expected_close_date", "CloseDate", "direct"),
            ("status", "StageName", "map_value"),
            ("probability", "Probability", "direct"),
            // Note: AccountId and ContactId require reference resolution
        ],
    ),
];

impl ObjectMapping {
    /// Create a new object mapping with default fields
    pub fn from_defaults(attio_object: &str, salesforce_object: &str) -> Option<Self> {
        DEFAULT_MAPPINGS
            .iter()
            .find(|(attio, sf, _)| *attio == attio_object && *sf == salesforce_object)
            .map(|(attio, sf, fields)| Self {
                attio_object: attio.to_string(),
                salesforce_object: sf.to_string(),
                enabled: true,
                fields: fields
                    .iter()
                    .map(|(a, s, t)| FieldMapping {
                        attio_field: a.to_string(),
                        salesforce_field: s.to_string(),
                        transform: TransformType::from_str(t),
                        required: false,
                        direction: FieldSyncDirection::default(),
                    })
                    .collect(),
                status_mappings: if attio_object == "deals" {
                    default_deal_status_mappings()
                } else {
                    HashMap::new()
                },
            })
    }
}

impl TransformType {
    fn from_str(s: &str) -> Self {
        match s {
            "direct" => Self::Direct,
            "extract_first" => Self::ExtractFirst,
            "extract_nested" => Self::ExtractNested {
                path: String::new(),
            },
            "currency_to_number" => Self::CurrencyToNumber,
            "country_code_to_name" => Self::CountryCodeToName,
            "employee_range_to_number" => Self::EmployeeRangeToNumber,
            "map_value" => Self::MapValue {
                mappings: HashMap::new(),
            },
            _ => Self::Direct,
        }
    }
}

/// Default status mappings for Deals → Opportunity stages
fn default_deal_status_mappings() -> HashMap<String, String> {
    [
        ("open", "Prospecting"),
        ("qualified", "Qualification"),
        ("in_progress", "Proposal/Price Quote"),
        ("negotiation", "Negotiation/Review"),
        ("won", "Closed Won"),
        ("lost", "Closed Lost"),
    ]
    .into_iter()
    .map(|(k, v)| (k.to_string(), v.to_string()))
    .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_mappings_exist() {
        assert!(ObjectMapping::from_defaults("companies", "Account").is_some());
        assert!(ObjectMapping::from_defaults("people", "Contact").is_some());
        assert!(ObjectMapping::from_defaults("deals", "Opportunity").is_some());
    }

    #[test]
    fn test_unknown_mapping_returns_none() {
        assert!(ObjectMapping::from_defaults("unknown", "Unknown").is_none());
    }

    #[test]
    fn test_deal_status_mappings() {
        let mapping = ObjectMapping::from_defaults("deals", "Opportunity").unwrap();
        assert_eq!(
            mapping.status_mappings.get("won"),
            Some(&"Closed Won".to_string())
        );
    }
}
