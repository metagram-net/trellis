use crate::endpoint::{Body, Endpoint};
use crate::StatusCode;
use http::Method;
use serde::{Deserialize, Serialize};

// Shared types

#[derive(Debug, Deserialize, PartialEq, Eq)]
pub struct Session {
    pub authentication_factors: Vec<AuthenticationFactor>,
    pub user_id: String,
    // TODO: the rest of the fields
}

#[derive(Debug, Deserialize, PartialEq, Eq)]
#[serde(tag = "delivery_method")]
#[non_exhaustive]
pub enum AuthenticationFactor {
    #[serde(rename = "email")]
    Email { email_factor: EmailFactor },
    // TODO: the rest of the fields
}

#[derive(Debug, Deserialize, PartialEq, Eq)]
pub struct EmailFactor {
    pub email_address: String,
    pub email_id: String,
}

// Endpoints

#[derive(Debug, Clone, Serialize)]
pub struct Authenticate<'a> {
    pub session_token: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session_duration_minutes: Option<u32>,
    // TODO: the rest of the fields
}

#[derive(Debug, Deserialize)]
pub struct AuthenticateResponse {
    pub request_id: String,
    pub status_code: StatusCode,

    pub session: Session,
    pub session_token: Option<String>,
}

impl Endpoint for Authenticate<'_> {
    fn method(&self) -> Method {
        Method::POST
    }
    fn path(&self) -> String {
        "sessions/authenticate".to_owned()
    }
    fn body(&self) -> serde_json::Result<Body> {
        Body::from_data(self)
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct Revoke<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session_token: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session_id: Option<&'a str>,
}

#[derive(Debug, Deserialize)]
pub struct RevokeResponse {
    pub request_id: String,
    pub status_code: StatusCode,
}

impl Endpoint for Revoke<'_> {
    fn method(&self) -> Method {
        Method::POST
    }
    fn path(&self) -> String {
        "sessions/revoke".to_owned()
    }
    fn body(&self) -> serde_json::Result<Body> {
        Body::from_data(self)
    }
}
