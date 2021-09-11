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
}

pub trait Endpoint {
    fn method(&self) -> Method;
    fn path(&self) -> String; // http::uri::PathAndQuery?
    fn query(&self) -> Query {
        Query::default()
    }
    fn body(&self) -> Body {
        Body::default()
    }
}
