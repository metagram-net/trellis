use serde::{Deserialize, Serialize};
use stytch;
use thiserror;

const ALLOW_LOGINS: bool = false; // TODO: enable!

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

    pub async fn authenticate_token(&self, token: &str) -> Result<User> {
        if !ALLOW_LOGINS {
            return Err(Error::Disabled);
        }
        let req = stytch::magic_links::Authenticate { token };
        let res: stytch::magic_links::AuthenticateResponse = self.client.reqwest(req).await?;
        Ok(User {
            user_id: res.user_id,
        })
    }

    pub async fn send_email(&self, email: &str) -> Result<User> {
        if !ALLOW_LOGINS {
            return Err(Error::Disabled);
        }
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
}

#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    pub user_id: String,
}

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("authentication disabled")]
    Disabled,
    #[error(transparent)]
    Stytch(#[from] stytch::Error),
}

pub type Result<T> = std::result::Result<T, Error>;
