use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    pub user_id: String,
}

#[derive(Error, Debug)]
pub enum AuthError {
    #[error("authentication disabled")]
    Disabled,
}

pub fn authenticate(_token: &str) -> Result<User, AuthError> {
    // TODO: Implement real users someday
    Err(AuthError::Disabled)
}
