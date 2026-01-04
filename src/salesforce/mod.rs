//! Salesforce API client and types.

pub mod auth;
pub mod bulk;
pub mod client;
pub mod objects;
pub mod types;

pub use auth::SalesforceAuth;
pub use client::SalesforceClient;
pub use types::{SalesforceRecord, SObject};
