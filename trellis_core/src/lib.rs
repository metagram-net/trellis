use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[cfg(test)]
mod tests;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct Settings {
    pub secrets: Secrets,
    pub tiles: Vec<Tile>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct Secrets {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owm_api_key: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(tag = "type")]
pub enum Data {
    Clock,
    Weather { location_id: String },
    Note { text: String },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct Tile {
    id: Uuid,

    #[serde(skip_serializing_if = "Option::is_none")]
    row: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    col: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    width: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    height: Option<u32>,

    data: Data,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct WeatherData {
    pub location_id: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct NoteData {
    pub text: String,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            secrets: Secrets { owm_api_key: None },
            tiles: vec![],
        }
    }
}
