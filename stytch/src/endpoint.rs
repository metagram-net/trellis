use http::Method;
use serde::Serialize;
use serde_json;

#[derive(Serialize)]
pub struct Query(Vec<(String, String)>);

impl Default for Query {
    fn default() -> Self {
        Self(Vec::new())
    }
}

#[derive(Serialize)]
pub struct Body(Option<serde_json::Value>);

impl Default for Body {
    fn default() -> Self {
        Self(None)
    }
}

impl Body {
    pub fn from_json(data: serde_json::Value) -> Self {
        Self(Some(data))
    }

    pub fn from_data<T: Serialize>(data: T) -> serde_json::Result<Self> {
        Ok(Self(Some(serde_json::to_value(data)?)))
    }
}

pub trait Endpoint {
    fn method(&self) -> Method;
    fn path(&self) -> String; // http::uri::PathAndQuery?
    fn query(&self) -> Query {
        Query::default()
    }
    fn body(&self) -> serde_json::Result<Body> {
        Ok(Body::default())
    }
}
