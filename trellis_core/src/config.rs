use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct Config {
    pub secrets: Secrets,
    pub tiles: Vec<Tile>,
}

const STARTER_TEXT: &'static str = r#"Welcome to Trellis!
This is _very_ much a work-in-progress, and there are _definitely_ major bugs.
If you think this is cool, have an idea to share, or want to watch development, it's all on GitHub! There should be a link on this page somewhere."#;

impl Default for Config {
    fn default() -> Self {
        Self {
            secrets: Secrets { owm_api_key: None },
            tiles: vec![
                Tile {
                    id: Uuid::new_v4(),
                    width: None,
                    height: None,
                    data: Data::Clock,
                },
                Tile {
                    id: Uuid::new_v4(),
                    width: None,
                    height: None,
                    data: Data::Weather {
                        location_id: "".to_owned(),
                    },
                },
                Tile {
                    id: Uuid::new_v4(),
                    width: None,
                    height: Some(2),
                    data: Data::Note {
                        text: STARTER_TEXT.to_owned(),
                    },
                },
            ],
        }
    }
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
