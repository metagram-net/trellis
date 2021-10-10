use std::env;
use stytch;
use tokio;

fn must_env(var: &str) -> String {
    env::var(var).expect(var)
}

fn test_client() -> stytch::Result<stytch::Client> {
    stytch::Client::new(
        must_env("STYTCH_PROJECT_ID"),
        must_env("STYTCH_SECRET"),
        stytch::env::TEST,
    )
}

#[tokio::test]
async fn sandbox_magic_links_authenticate() -> stytch::Result<()> {
    let client = test_client()?;
    let req = stytch::magic_links::Authenticate {
        token: "DOYoip3rvIMMW5lgItikFK-Ak1CfMsgjuiCyI7uuU94=",
        session_duration_minutes: None,
    };
    let res: stytch::magic_links::AuthenticateResponse = client.reqwest(req).await?;
    assert_eq!(
        res.user_id,
        "user-test-e3795c81-f849-4167-bfda-e4a6e9c280fd"
    );
    Ok(())
}

#[tokio::test]
async fn sandbox_magic_links_email_send() -> stytch::Result<()> {
    let client = test_client()?;
    let req = stytch::magic_links::email::Send {
        email: "sandbox@stytch.com",
        login_magic_link_url: &must_env("STYTCH_LOGIN_URL"),
        signup_magic_link_url: &must_env("STYTCH_SIGNUP_URL"),
    };
    let res: stytch::magic_links::email::SendResponse = client.reqwest(req).await?;
    assert_eq!(
        res.user_id,
        "user-test-e3795c81-f849-4167-bfda-e4a6e9c280fd"
    );
    Ok(())
}

#[tokio::test]
async fn sandbox_sessions_authenticate() -> stytch::Result<()> {
    let client = test_client()?;
    let req = stytch::sessions::Authenticate {
        session_token: "WJtR5BCy38Szd5AfoDpf0iqFKEt4EE5JhjlWUY7l3FtY",
    };
    let res: stytch::sessions::AuthenticateResponse = client.reqwest(req).await?;
    assert_eq!(
        res.session_token,
        Some("WJtR5BCy38Szd5AfoDpf0iqFKEt4EE5JhjlWUY7l3FtY".to_owned())
    );
    assert_eq!(
        res.session,
        stytch::sessions::Session {
            user_id: "user-test-e3795c81-f849-4167-bfda-e4a6e9c280fd".to_owned(),
            authentication_factors: vec![],
        }
    );
    Ok(())
}
