use anyhow;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use yew::format::Json;
use yew::services::fetch;
use yew::services::storage::{Area, StorageService};
use yew::worker::*;

#[derive(Serialize, Deserialize, Debug)]
pub enum Request {
    Save(trellis_core::Settings),
    Load,
}

pub struct Settings {
    link: AgentLink<Self>,
    subscribers: HashSet<HandlerId>,
    local: StorageService,
    save_req: Option<fetch::FetchTask>,
    // TODO: loadReq: fetch::FetchTask,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Msg {
    Saved,
    // Loaded(trellis_core::Settings),
}

impl Settings {
    const KEY: &'static str = "trellis.settings";

    fn broadcast(&self, s: trellis_core::Settings) {
        for sub in self.subscribers.iter() {
            self.link.respond(*sub, s.clone());
        }
    }

    fn save(&mut self, s: &trellis_core::Settings) {
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

    fn load(&self) -> trellis_core::Settings {
        self.local
            .restore::<Json<anyhow::Result<trellis_core::Settings>>>(Self::KEY)
            .0
            .unwrap_or_default()
        // TODO: Start a task to load from server
    }
}

impl Agent for Settings {
    type Reach = Context<Self>;
    type Message = Msg;
    type Input = Request;
    type Output = trellis_core::Settings;

    fn create(link: AgentLink<Self>) -> Self {
        Self {
            link,
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

    fn handle_input(&mut self, msg: Self::Input, id: HandlerId) {
        match msg {
            Request::Save(s) => {
                self.save(&s);
                self.broadcast(s);
            }
            Request::Load => {
                self.link.respond(id, self.load());
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
