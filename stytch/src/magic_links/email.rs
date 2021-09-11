use crate::endpoint::{Body, Endpoint};
use crate::StatusCode;
use http::Method;
use serde::Deserialize;
use serde_json::{self, json};

#[derive(Debug)]
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
    fn body(&self) -> Body {
        Body::from_json(json!({
            "email": self.email,
            "login_magic_link_url": self.login_magic_link_url,
            "signup_magic_link_url": self.signup_magic_link_url,
            // TODO: the rest of the fields
        }))
    }
}

// TODO: login_or_create
// TODO: invite
// TODO: ...
