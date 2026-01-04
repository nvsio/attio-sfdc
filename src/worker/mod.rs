//! Cloudflare Worker entry point and HTTP handlers.

mod middleware;
mod routes;

pub use routes::handle_request;

use worker::{event, Context, Env, Request, Response, Result};

/// Main worker entry point
#[event(fetch)]
pub async fn main(req: Request, env: Env, _ctx: Context) -> Result<Response> {
    handle_request(req, env).await
}
