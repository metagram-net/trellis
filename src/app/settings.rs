use super::weather;
use anyhow;
use serde::{Deserialize, Serialize};
use yew::format::Json;
use yew::services::storage::{Area, StorageService};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Settings {
    pub weather: weather::Props,
}

impl Settings {
    const KEY: &'static str = "trellis.settings";

    pub fn load() -> Self {
        StorageService::new(Area::Local)
            .ok()
            .and_then(|storage| {
                storage
                    .restore::<Json<anyhow::Result<Settings>>>(Self::KEY)
                    .0
                    .ok()
            })
            .unwrap_or_default()
    }

    pub fn save(&self) -> Result<(), Box<dyn std::error::Error>> {
        let mut storage = StorageService::new(Area::Local)?;
        storage.store(Self::KEY, Json(self));
        Ok(())
    }
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            weather: weather::Props {
                location_id: String::new(),
                owm_api_key: String::new(),
            },
        }
    }
}
