use anyhow;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use trellis_core::config;
use uuid;
use yew::format::Json;
use yew::services::fetch;
use yew::services::storage::{Area, StorageService};
use yew::worker::*;

#[derive(Serialize, Deserialize, Debug)]
pub enum Request {
    Save(config::Config),
    Load,
    SaveNote { id: uuid::Uuid, text: String },
}

pub struct Settings {
    link: AgentLink<Self>,
    subscribers: HashSet<HandlerId>,
    local: StorageService,
    settings: config::Config,
    save_req: Option<fetch::FetchTask>,
    // TODO: loadReq: fetch::FetchTask,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Msg {
    Saved,
    // Loaded(config::Config),
}

impl Settings {
    const KEY: &'static str = "trellis.settings";

    fn broadcast(&self, s: config::Config) {
        for sub in self.subscribers.iter() {
            self.link.respond(*sub, s.clone());
        }
    }

    fn save(&mut self, s: &config::Config) {
        self.settings = s.clone();
        self.local.store(Self::KEY, Json(s));

        let req = fetch::Request::post("/api/v1/save")
            .header("Content-Type", "application/json")
            .body(Json(s))
            .expect("could not build request");

        let cb = self
            .link
            .callback(|res: fetch::Response<anyhow::Result<String>>| {
                if res.status().is_success() {
                    Msg::Saved
                } else {
                    // TODO: Try again with exponential backoff?
                    Msg::Saved
                }
            });

        let task = fetch::FetchService::fetch(req, cb).expect("could not start request");
        self.save_req = Some(task);
    }

    fn load(&mut self) -> config::Config {
        self.settings = self
            .local
            .restore::<Json<anyhow::Result<config::Config>>>(Self::KEY)
            .0
            .unwrap_or_default();
        // TODO: Start a task to load from server
        self.settings.clone()
    }
}

impl Agent for Settings {
    type Reach = Context<Self>;
    type Message = Msg;
    type Input = Request;
    type Output = config::Config;

    fn create(link: AgentLink<Self>) -> Self {
        Self {
            link,
            settings: config::Config::default(),
            subscribers: HashSet::new(),
            local: StorageService::new(Area::Local).expect("Could not connect to LocalStorage"),
            save_req: None,
        }
    }

    fn update(&mut self, msg: Self::Message) {
        match msg {
            Msg::Saved => self.save_req = None,
        }
    }

    fn handle_input(&mut self, msg: Self::Input, _caller: HandlerId) {
        match msg {
            Request::Save(s) => {
                self.save(&s);
                self.broadcast(s);
            }
            Request::Load => {
                let cfg = self.load();
                self.broadcast(cfg);
            }
            Request::SaveNote { id, text } => {
                let mut new_tiles: Vec<config::Tile> = Vec::new();
                for tile in self.settings.tiles.iter() {
                    if tile.id != id {
                        new_tiles.push(tile.clone());
                    } else {
                        new_tiles.push(config::Tile {
                            id: tile.id,
                            row: tile.row,
                            col: tile.col,
                            width: tile.width,
                            height: tile.height,
                            data: config::Data::Note { text: text.clone() },
                        })
                    }
                }
                self.settings.tiles = new_tiles;
                self.link.send_input(Request::Save(self.settings.clone()));
            }
        }
    }

    fn connected(&mut self, id: HandlerId) {
        self.subscribers.insert(id);
    }

    fn disconnected(&mut self, id: HandlerId) {
        self.subscribers.remove(&id);
    }
}
