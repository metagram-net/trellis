use crate::endpoint::{Body, Endpoint};
use crate::sessions;
use crate::StatusCode;
use http::Method;
use serde::{Deserialize, Serialize};

pub mod email;

#[derive(Debug, Clone, Serialize)]
pub struct Authenticate<'a> {
    pub token: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session_duration_minutes: Option<u32>,
    // TODO: the rest of the fields
}

#[derive(Debug, Deserialize)]
pub struct AuthenticateResponse {
    pub request_id: String,
    pub status_code: StatusCode,

    pub method_id: String,
    pub user_id: String,

    pub session: Option<sessions::Session>,
    pub session_token: Option<String>,
}

impl Endpoint for Authenticate<'_> {
    fn method(&self) -> Method {
        Method::POST
    }
    fn path(&self) -> String {
        "magic_links/authenticate".to_owned()
    }
    fn body(&self) -> serde_json::Result<Body> {
        Body::from_data(self)
    }
}
