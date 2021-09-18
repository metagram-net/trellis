use super::login_confirmation::LoginConfirmation;
use super::login_form::LoginForm;
use anyhow;
use serde::{Deserialize, Serialize};
use yew::format::Json;
use yew::prelude::*;
use yew::services::fetch::{FetchService, FetchTask, Request, Response};

pub struct Login {
    link: ComponentLink<Self>,
    state: State,
    login_task: Option<FetchTask>,
}

pub enum Msg {
    Submit(String),
    Success(LoginResponse),
    Error(anyhow::Result<LoginResponse>),
    Reload,
}

enum State {
    Form(Option<String>),
    Success(String),
}

impl Component for Login {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            state: State::Form(None),
            login_task: None,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Submit(email) => {
                self.login_task = Some(self.login(email));
                false
            }
            Msg::Success(res) => {
                self.state = State::Success(res.email);
                true
            }
            Msg::Error(res) => {
                let msg = match res {
                    Err(err) => err.to_string(),
                    Ok(r) => match r.message {
                        None => "Whoops! Something went wrong. Please try again.".to_owned(),
                        Some(msg) => msg,
                    },
                };
                self.state = State::Form(Some(msg));
                true
            }
            Msg::Reload => {
                self.state = State::Form(None);
                self.login_task = None;
                true
            }
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        match &self.state {
            State::Form(errors) => self.view_form(errors),
            State::Success(email) => self.view_confirmation(email),
        }
    }
}

// TODO: Merge these structs with the trellis_server versions

#[derive(Serialize, Clone)]
pub struct LoginRequest {
    email: String,
}

#[derive(Deserialize, Clone)]
pub struct LoginResponse {
    email: String,
    message: Option<String>,
}

impl Login {
    fn login(&self, email: String) -> FetchTask {
        let body = LoginRequest { email };
        let req = Request::post("/api/v1/login")
            .header("Content-Type", "application/json")
            .body(Json(&body))
            .expect("could not build request");

        let cb = self
            .link
            .callback(|res: Response<Json<anyhow::Result<LoginResponse>>>| {
                if res.status().is_success() {
                    let Json(data) = res.into_body();
                    match data {
                        Ok(r) => Msg::Success(r),
                        Err(err) => Msg::Error(Err(err)),
                    }
                } else {
                    let Json(data) = res.into_body();
                    Msg::Error(data)
                }
            });

        FetchService::fetch(req, cb).expect("could not start request")
    }

    fn view_form(&self, error_message: &Option<String>) -> Html {
        let errors = match &error_message {
            None => html! {},
            Some(err) => html! {
                <div class="alert">
                    <p>{err.to_string()}</p>
                </div>
            },
        };
        let onsubmit = self.link.callback(Msg::Submit);

        html! {
            <div>
                {errors}
                <LoginForm onsubmit=onsubmit />
            </div>
        }
    }

    fn view_confirmation(&self, email: &String) -> Html {
        let onsubmit = self.link.callback(|_| Msg::Reload);

        html! {
            <div>
                <LoginConfirmation email=email.clone() onsubmit=onsubmit />
            </div>
        }
    }
}
