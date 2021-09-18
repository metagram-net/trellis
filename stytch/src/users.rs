use crate::endpoint::{Body, Endpoint};
use crate::StatusCode;
use http::Method;
use serde::Deserialize;
use serde_json::{self, json};

#[derive(Debug)]
pub struct Create<'a> {
    pub email: &'a str,
    // TODO: the rest of the fields
}

#[derive(Debug, Deserialize)]
pub struct CreateResponse {
    pub request_id: String,
    pub status_code: StatusCode,

    pub user_id: String,
    pub email_id: String,
    pub phone_id: String,
    pub status: UserStatus,
}

#[Derive(Deserialize)]
pub enum UserStatus {
    Active,
    Pending,
}

impl Endpoint for Create<'_> {
    fn method(&self) -> Method {
        Method::POST
    }
    fn path(&self) -> String {
        "users".to_owned()
    }
    fn body(&self) -> Body {
        Body::from_json(json!({
            "email": self.email,
        }))
    }
}
