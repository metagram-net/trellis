use crate::endpoint::Endpoint;
use crate::{Error, ErrorResponse, Result};
use reqwest;
use serde::de::DeserializeOwned;
use url::Url;

pub struct Client {
    client: reqwest::Client,
    base_url: Url,
    project_id: String,
    secret: String,
}

impl Client {
    pub fn new(project_id: String, secret: String, base_url: &str) -> Result<Self> {
        let client = reqwest::Client::builder()
            .user_agent(super::USER_AGENT)
            .https_only(true)
            .build()?;
        Ok(Self {
            client,
            base_url: Url::parse(base_url)?,
            project_id,
            secret,
        })
    }

    pub async fn reqwest<Res: DeserializeOwned>(&self, e: impl Endpoint) -> Result<Res> {
        let url = self.base_url.join(&e.path())?.to_string();
        let req = self
            .client
            .request(e.method(), url)
            .basic_auth(&self.project_id, Some(&self.secret))
            .query(&e.query())
            .json(&e.body()?);
        let res = req.send().await?;
        if res.status().is_success() {
            Ok(res.json::<Res>().await?)
        } else {
            Err(Error::Response(res.json::<ErrorResponse>().await?))
        }
    }
}
