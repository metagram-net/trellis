use super::*;
use std::env;
use tokio;

fn test_client() -> Result<Client> {
    Client::new(
        env::var("STYTCH_PROJECT_ID").expect("STYTCH_PROJECT_ID"),
        env::var("STYTCH_SECRET").expect("STYTCH_SECRET"),
        super::env::TEST,
    )
}

#[tokio::test]
async fn sandbox_authenticate() -> Result<()> {
    let client = test_client()?;
    let req = magic_links::Authenticate {
        token: "DOYoip3rvIMMW5lgItikFK-Ak1CfMsgjuiCyI7uuU94=",
    };
    let res: magic_links::AuthenticateResponse = client.reqwest(req).await?;
    assert_eq!(
        res.user_id,
        "user-test-e3795c81-f849-4167-bfda-e4a6e9c280fd"
    );
    Ok(())
}

#[tokio::test]
async fn sandbox_email_send() -> Result<()> {
    let client = test_client()?;
    let req = magic_links::email::Send {
        email: "sandbox@stytch.com",
        login_magic_link_url: "http://localhost:3000/auth/login",
        signup_magic_link_url: "http://localhost:3000/auth/signup",
    };
    let res: magic_links::email::SendResponse = client.reqwest(req).await?;
    assert_eq!(
        res.user_id,
        "user-test-e3795c81-f849-4167-bfda-e4a6e9c280fd"
    );
    Ok(())
}
