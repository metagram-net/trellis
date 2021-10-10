use serde::{Deserialize, Serialize};

pub mod client;
pub mod env;
pub mod magic_links;
pub mod sessions;

mod endpoint;
mod status_code;

const USER_AGENT: &'static str = include_str!(concat!(env!("OUT_DIR"), "/user-agent"));

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum Error {
    #[error("{} {}: {} ({})", .0.status_code, .0.error_type, .0.error_message, .0.error_url)]
    Response(ErrorResponse),
    #[error(transparent)]
    Reqwest(#[from] reqwest::Error),
    #[error(transparent)]
    Url(#[from] url::ParseError),
    #[error(transparent)]
    Serde(#[from] serde_json::Error),
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
