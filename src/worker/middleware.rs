//! HTTP middleware for authentication and logging.

use worker::{Request, Response};

/// Verify API key authentication
pub fn verify_api_key(req: &Request, expected_key: &str) -> bool {
    req.headers()
        .get("Authorization")
        .ok()
        .flatten()
        .map(|auth| {
            auth.strip_prefix("Bearer ")
                .map(|token| token == expected_key)
                .unwrap_or(false)
        })
        .unwrap_or(false)
}

/// Create unauthorized response
pub fn unauthorized() -> Response {
    Response::error("Unauthorized", 401).unwrap()
}

/// Create bad request response
pub fn bad_request(message: &str) -> Response {
    Response::error(message, 400).unwrap()
}

/// Create internal server error response
pub fn internal_error(message: &str) -> Response {
    Response::error(message, 500).unwrap()
}

/// Create not found response
pub fn not_found() -> Response {
    Response::error("Not Found", 404).unwrap()
}

/// Log request details (for debugging)
pub fn log_request(req: &Request) {
    let method = req.method().to_string();
    let path = req.path();
    worker::console_log!("{} {}", method, path);
}
