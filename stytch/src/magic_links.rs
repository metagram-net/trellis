use crate::endpoint::{Body, Endpoint};
use crate::StatusCode;
use http::Method;
use serde::Deserialize;
use serde_json::{self, json};

pub mod email;

#[derive(Debug)]
pub struct Authenticate<'a> {
    pub token: &'a str,
    // TODO: the rest of the fields
}

#[derive(Debug, Deserialize)]
pub struct AuthenticateResponse {
    pub request_id: String,
    pub status_code: StatusCode,

    pub method_id: String,
    pub user_id: String,
    // TODO: sessions
    // pub session: Option<Session>,
    // pub session_token: String,
}

impl Endpoint for Authenticate<'_> {
    fn method(&self) -> Method {
        Method::POST
    }
    fn path(&self) -> String {
        "magic_links/authenticate".to_owned()
    }
    fn body(&self) -> Body {
        Body::from_json(json!({
            "token": self.token,
        }))
    }
}
