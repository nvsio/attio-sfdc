//! Attio API client and types.

pub mod client;
pub mod objects;
pub mod types;
pub mod webhooks;

pub use client::AttioClient;
pub use types::{AttioRecord, AttioObject, AttioAttribute};
