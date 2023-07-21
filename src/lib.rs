#[allow(dead_code)]
mod client;
mod config;
mod endpoints;
mod error;
#[allow(dead_code)]
mod models;
#[allow(dead_code)]
mod util;

pub use client::Client;

/// Create an active campaign client instance
///
/// ```
/// let _ = active_campaign::new();
/// ```
pub fn new() -> Client {
    Client::default()
}
