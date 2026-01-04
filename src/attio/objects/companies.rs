//! Attio Companies object.

use serde::{Deserialize, Serialize};

/// Typed representation of an Attio Company record
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompanyRecord {
    /// Company name
    pub name: Option<String>,

    /// Primary domain
    pub primary_domain: Option<String>,

    /// All domains
    pub domains: Vec<String>,

    /// Description
    pub description: Option<String>,

    /// Primary location
    pub primary_location: Option<Location>,

    /// Industry categories
    pub categories: Vec<String>,

    /// Employee range (e.g., "11-50")
    pub employee_range: Option<String>,

    /// Estimated ARR in USD
    pub estimated_arr_usd: Option<f64>,

    /// LinkedIn URL
    pub linkedin_url: Option<String>,

    /// Twitter handle
    pub twitter_handle: Option<String>,
}

/// Location struct
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Location {
    pub line_1: Option<String>,
    pub line_2: Option<String>,
    pub locality: Option<String>,
    pub region: Option<String>,
    pub postcode: Option<String>,
    pub country_code: Option<String>,
}

impl CompanyRecord {
    /// Get formatted address
    pub fn formatted_address(&self) -> Option<String> {
        self.primary_location.as_ref().map(|loc| {
            [
                loc.line_1.as_deref(),
                loc.line_2.as_deref(),
                loc.locality.as_deref(),
                loc.region.as_deref(),
                loc.postcode.as_deref(),
                loc.country_code.as_deref(),
            ]
            .iter()
            .filter_map(|&s| s)
            .collect::<Vec<_>>()
            .join(", ")
        })
    }

    /// Parse employee range to midpoint number
    pub fn employee_count_estimate(&self) -> Option<u32> {
        self.employee_range.as_ref().and_then(|range| {
            parse_employee_range(range)
        })
    }
}

/// Parse employee range string to midpoint
fn parse_employee_range(range: &str) -> Option<u32> {
    // Common formats: "1-10", "11-50", "51-200", "201-500", "500+"
    if range.ends_with('+') {
        range.trim_end_matches('+').parse().ok()
    } else if range.contains('-') {
        let parts: Vec<&str> = range.split('-').collect();
        if parts.len() == 2 {
            let low: u32 = parts[0].trim().parse().ok()?;
            let high: u32 = parts[1].trim().parse().ok()?;
            Some((low + high) / 2)
        } else {
            None
        }
    } else {
        range.parse().ok()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_employee_range() {
        assert_eq!(parse_employee_range("11-50"), Some(30));
        assert_eq!(parse_employee_range("1-10"), Some(5));
        assert_eq!(parse_employee_range("500+"), Some(500));
        assert_eq!(parse_employee_range("100"), Some(100));
    }
}
