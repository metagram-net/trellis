use anyhow;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use trellis_core::config;
use uuid;
use yew::format::{Json, Nothing};
use yew::services::console::ConsoleService;
use yew::services::fetch;
use yew::services::storage::{Area, StorageService};
use yew::worker::*;

#[derive(Serialize, Deserialize, Debug)]
pub enum Request {
    Save(config::Config),
    Load,
    SaveSingle { id: uuid::Uuid, data: config::Data },
    SaveSecrets { secrets: config::Secrets },
}

pub struct Settings {
    link: AgentLink<Self>,
    subscribers: HashSet<HandlerId>,
    local: StorageService,
    settings: config::Config,
    save_req: Option<fetch::FetchTask>,
    load_req: Option<fetch::FetchTask>,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Msg {
    Saved,
    Loaded(config::Config),
    Noop,
}

// TODO: Skip all the remote save/load stuff unless logged in

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
                    // TODO: Try again later?
                    ConsoleService::error("could not save settings");
                    Msg::Saved
                }
            });

        let task = fetch::FetchService::fetch(req, cb).expect("could not start request");
        self.save_req = Some(task);
    }

    fn load(&mut self) -> config::Config {
        // Load local settings early to make things interactive faster.
        self.settings = self
            .local
            .restore::<Json<anyhow::Result<config::Config>>>(Self::KEY)
            .0
            .unwrap_or_default();

        // Load remote settings in the background and accept the flash of stale layout.  Ideally,
        // syncs happen frequently enough that LocalStorage doesn't get _too_ desynced.

        let req = fetch::Request::get("/api/v1/load")
            .body(Nothing)
            .expect("could not build request");

        let cb = self.link.callback(
            |res: fetch::Response<Json<anyhow::Result<config::Config>>>| {
                let Json(data) = res.into_body();
                match data {
                    Ok(settings) => Msg::Loaded(settings),
                    Err(err) => {
                        // TODO: Try again later?
                        ConsoleService::error(&format!("could not load settings: {}", err));
                        Msg::Noop
                    }
                }
            },
        );
        let task = fetch::FetchService::fetch(req, cb).expect("could not start request");
        self.load_req = Some(task);

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
            load_req: None,
        }
    }

    fn update(&mut self, msg: Self::Message) {
        match msg {
            Msg::Noop => (),
            Msg::Saved => self.save_req = None,
            Msg::Loaded(settings) => {
                self.settings = settings;
                self.load_req = None;
            }
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
            Request::SaveSingle { id, data } => {
                // TODO: Make this a method on Config
                let mut new_tiles: Vec<config::Tile> = Vec::new();
                for tile in self.settings.tiles.iter() {
                    if tile.id != id {
                        new_tiles.push(tile.clone());
                    } else {
                        new_tiles.push(config::Tile {
                            id: tile.id,
                            width: tile.width,
                            height: tile.height,
                            data: data.clone(),
                        })
                    }
                }
                self.settings.tiles = new_tiles;
                self.link.send_input(Request::Save(self.settings.clone()));
            }
            Request::SaveSecrets { secrets } => {
                let mut cfg = self.settings.clone();
                cfg.secrets = secrets;
                self.link.send_input(Request::Save(cfg));
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
