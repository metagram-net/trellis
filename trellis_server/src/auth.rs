use serde::{Deserialize, Serialize};
use stytch;
use thiserror;

const ALLOW_LOGINS: bool = false; // TODO: enable!

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

pub async fn authenticate_token(client: &stytch::Client, token: &str) -> Result<User> {
    if !ALLOW_LOGINS {
        return Err(Error::Disabled);
    }
    let req = stytch::magic_links::Authenticate { token };
    let res: stytch::magic_links::AuthenticateResponse = client.reqwest(req).await?;
    Ok(User {
        user_id: res.user_id,
    })
}

pub async fn send_email(client: &stytch::Client, email: &str) -> Result<User> {
    if !ALLOW_LOGINS {
        return Err(Error::Disabled);
    }
    let req = stytch::magic_links::email::Send {
        email,
        // TODO: Get these values from environment variables
        login_magic_link_url: "http://localhost:3000/auth/login",
        signup_magic_link_url: "http://localhost:3000/auth/signup",
    };
    let res: stytch::magic_links::email::SendResponse = client.reqwest(req).await?;
    Ok(User {
        user_id: res.user_id,
    })
}
