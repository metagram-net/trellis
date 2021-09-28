use rand::prelude::*;
use rocket::fairing::{Fairing, Info, Kind};
use rocket::http::{Cookie, Status};
use rocket::request::{FromRequest, Outcome};
use rocket::{Data, Request};

// TODO: Unify with client-side definitions
const COOKIE_NAME: &'static str = "trellis.csrf_protection";
const HEADER_NAME: &'static str = "X-CSRF-Protection";

pub fn fairing() -> CsrfFairing {
    CsrfFairing {}
}

pub struct Protection {}

#[derive(Debug)]
pub enum CsrfStatus {
    Valid,
    Invalid,
}

#[derive(Debug)]
pub enum CsrfError {
    Invalid,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for Protection {
    type Error = CsrfError;

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        match validate(req) {
            CsrfStatus::Valid => Outcome::Success(Protection {}),
            CsrfStatus::Invalid => Outcome::Failure((Status::BadRequest, CsrfError::Invalid)),
        }
    }
}

pub struct CsrfFairing {}

#[rocket::async_trait]
impl Fairing for CsrfFairing {
    fn info(&self) -> Info {
        Info {
            name: "CSRF Protection",
            kind: Kind::Request | Kind::Singleton,
        }
    }

    async fn on_request(&self, req: &mut Request<'_>, _data: &mut Data<'_>) {
        let token = new_token();
        req.cookies().add(
            Cookie::build(COOKIE_NAME, token)
                // Allow JS to read this cookie value.  The whole point of this value is that
                // scripts running on the site (in other words, the whole trellis_web app) can
                // reliably read the latest value when creating a POST request.
                .http_only(false)
                .secure(true)
                .finish(),
        );
    }
}

fn validate(req: &Request<'_>) -> CsrfStatus {
    let cookie = req.cookies().get(COOKIE_NAME);
    let header = req.headers().get_one(HEADER_NAME);

    match (cookie, header) {
        (Some(c), Some(h)) => {
            if c.value() == h && h != "" {
                CsrfStatus::Valid
            } else {
                CsrfStatus::Invalid
            }
        }
        _ => CsrfStatus::Invalid,
    }
}

fn new_token() -> String {
    let mut b = [0u8; 32];
    thread_rng().fill(&mut b[..]);
    base64::encode_config(b, base64::URL_SAFE_NO_PAD)
}
