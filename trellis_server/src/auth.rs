use rocket::http::{Cookie, CookieJar};
use serde::{Deserialize, Serialize};
use stytch;
use thiserror;

const SESSION_DURATION_MINUTES: u32 = 30 * 24 * 60; // Thirty days
const SESSION_COOKIE_NAME: &'static str = "trellis.session";

pub struct Auth {
    client: stytch::Client,
    login_url: String,
    signup_url: String,
}

#[derive(Deserialize)]
pub struct Config {
    pub env: String,
    pub project_id: String,
    pub secret: String,

    pub login_url: String,
    pub signup_url: String,
}

impl Auth {
    pub fn new(cfg: Config) -> Result<Self> {
        let stytch_env = match cfg.env.as_str() {
            "live" => stytch::env::LIVE,
            "test" => stytch::env::TEST,
            custom => custom,
        };
        let client = stytch::Client::new(cfg.project_id, cfg.secret, stytch_env)?;

        Ok(Self {
            login_url: cfg.login_url,
            signup_url: cfg.signup_url,
            client,
        })
    }

    pub async fn authenticate_magic(&self, token: &str) -> Result<Session> {
        let req = stytch::magic_links::Authenticate {
            token,
            session_duration_minutes: Some(SESSION_DURATION_MINUTES),
        };
        let res: stytch::magic_links::AuthenticateResponse = self.client.reqwest(req).await?;
        match res.session {
            Some(s) => Ok(convert_session(s, res.session_token)),
            None => Err(Error::NoSession),
        }
    }

    pub async fn send_email(&self, email: &str) -> Result<User> {
        let req = stytch::magic_links::email::Send {
            email,
            login_magic_link_url: &self.login_url,
            signup_magic_link_url: &self.signup_url,
        };
        let res: stytch::magic_links::email::SendResponse = self.client.reqwest(req).await?;
        Ok(User {
            user_id: res.user_id,
        })
    }

    pub async fn authenticate_session(&self, token: &str) -> Result<Session> {
        let req = stytch::sessions::Authenticate {
            session_token: token,
            session_duration_minutes: Some(SESSION_DURATION_MINUTES),
        };
        let res: stytch::sessions::AuthenticateResponse = self.client.reqwest(req).await?;
        Ok(convert_session(res.session, res.session_token))
    }

    pub async fn revoke_session(&self, token: &str) -> Result<()> {
        let req = stytch::sessions::Revoke {
            session_token: Some(token),
            session_id: None,
        };
        let _res: stytch::sessions::RevokeResponse = self.client.reqwest(req).await?;
        Ok(())
    }

    // TODO: Turn this into a request guard
    pub async fn current_user(&self, cookies: &CookieJar<'_>) -> Result<Session> {
        let token = match cookies.get_private(SESSION_COOKIE_NAME) {
            None => return Err(Error::NoSession),
            Some(cookie) => String::from(cookie.value()),
        };
        self.authenticate_session(&token).await
    }

    pub fn set_cookie(&self, cookies: &CookieJar<'_>, session: Session) -> () {
        cookies.add_private(
            Cookie::build(
                SESSION_COOKIE_NAME,
                session.session_token.unwrap_or_default(),
            )
            .secure(true)
            .finish(),
        );
    }

    fn clear_cookie(&self, cookies: &CookieJar<'_>) -> () {
        cookies.remove_private(Cookie::named(SESSION_COOKIE_NAME));
    }

    pub async fn logout(&self, cookies: &CookieJar<'_>) -> Result<()> {
        let token = match cookies.get_private(SESSION_COOKIE_NAME) {
            None => return Ok(()),
            Some(cookie) => String::from(cookie.value()),
        };
        self.revoke_session(&token).await?;
        self.clear_cookie(cookies);
        Ok(())
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    pub user_id: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Session {
    pub email: String,
    pub session_token: Option<String>,
    pub user_id: String,
}

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error(transparent)]
    Stytch(#[from] stytch::Error),
    #[error("no session")]
    NoSession,
}

pub type Result<T> = std::result::Result<T, Error>;

fn convert_session(session: stytch::sessions::Session, session_token: Option<String>) -> Session {
    let email = session
        .authentication_factors
        .iter()
        .find_map(|factor| match factor {
            stytch::sessions::AuthenticationFactor::Email { email_factor } => {
                Some(email_factor.email_address.clone())
            }
            _ => None,
        })
        .unwrap_or_default();
    Session {
        email,
        session_token: session_token,
        user_id: session.user_id,
    }
}
