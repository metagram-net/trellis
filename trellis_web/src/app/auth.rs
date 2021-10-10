use anyhow;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use yew::format::{Json, Nothing};
use yew::services::fetch;
use yew::services::storage::{Area, StorageService};
use yew::worker::*;

#[derive(Serialize, Deserialize, Debug)]
pub enum Request {
    Whoami,
}

pub struct Auth {
    link: AgentLink<Self>,
    subscribers: HashSet<HandlerId>,
    local: StorageService,
    user: Option<User>,
    req: Option<fetch::FetchTask>,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Msg {
    Whoami(Option<User>),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct User {
    pub user_id: String,
}

impl Auth {
    const KEY: &'static str = "trellis.user";

    fn broadcast(&self, u: Option<User>) {
        for sub in self.subscribers.iter() {
            self.link.respond(*sub, u.clone());
        }
    }

    fn whoami(&mut self) -> fetch::FetchTask {
        let req = fetch::Request::get("/api/v1/whoami")
            .body(Nothing)
            .expect("could not build request");

        let cb = self
            .link
            .callback(|res: fetch::Response<Json<anyhow::Result<User>>>| {
                let Json(data) = res.into_body();
                Msg::Whoami(data.ok())
            });
        fetch::FetchService::fetch(req, cb).expect("could not start request")
    }
}

impl Agent for Auth {
    type Reach = Context<Self>;
    type Message = Msg;
    type Input = Request;
    type Output = Option<User>;

    fn create(link: AgentLink<Self>) -> Self {
        Self {
            link,
            user: None,
            subscribers: HashSet::new(),
            local: StorageService::new(Area::Local).expect("Could not connect to LocalStorage"),
            req: None,
        }
    }

    fn update(&mut self, msg: Self::Message) {
        match msg {
            Msg::Whoami(u) => {
                self.local.store(Self::KEY, Json(&u));
                self.user = u.clone();
                self.req = None;
                self.broadcast(u);
            }
        }
    }

    fn handle_input(&mut self, msg: Self::Input, _caller: HandlerId) {
        match msg {
            Request::Whoami => {
                if self.user.is_some() {
                    let u = self.user.clone();
                    self.broadcast(u);
                } else {
                    self.req = Some(self.whoami());
                }
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
