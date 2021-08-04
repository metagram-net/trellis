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
    pub id: Uuid,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub row: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub col: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<u32>,

    pub data: Data,
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
            // TODO: Put instructions in a text tile
            tiles: vec![],
        }
    }
}
