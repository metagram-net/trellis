use serde::{Deserialize, Serialize};

pub mod client;
mod endpoint;
pub mod env;
pub mod magic_links;
mod status_code;

#[cfg(test)]
mod tests;

const USER_AGENT: &'static str = include_str!(concat!(env!("OUT_DIR"), "/user-agent"));

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("{} {}: {} ({})", .0.status_code, .0.error_type, .0.error_message, .0.error_url)]
    Response(ErrorResponse),
    #[error(transparent)]
    Reqwest(#[from] reqwest::Error),
    #[error(transparent)]
    Url(#[from] url::ParseError),
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ErrorResponse {
    pub status_code: StatusCode,
    pub request_id: String,
    pub error_type: String,
    pub error_message: String,
    pub error_url: String,
}

pub use client::Client;
pub use status_code::StatusCode;
