use crate::endpoint::{Body, Endpoint};
use crate::StatusCode;
use http::Method;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize)]
pub struct Send<'a> {
    pub email: &'a str,
    pub login_magic_link_url: &'a str,
    pub signup_magic_link_url: &'a str,
    // TODO: the rest of the fields
}

#[derive(Debug, Deserialize)]
pub struct SendResponse {
    pub request_id: String,
    pub status_code: StatusCode,

    pub email_id: String,
    pub user_id: String,
}

impl Endpoint for Send<'_> {
    fn method(&self) -> Method {
        Method::POST
    }
    fn path(&self) -> String {
        "magic_links/email/send".to_owned()
    }
    fn body(&self) -> serde_json::Result<Body> {
        Body::from_data(self)
    }
}

// TODO: login_or_create
// TODO: invite
// TODO: ...
