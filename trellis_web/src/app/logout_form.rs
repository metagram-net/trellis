use super::csrf;
use anyhow;
use yew::format::Nothing;
use yew::prelude::*;
use yew::services::console::ConsoleService;
use yew::services::fetch::{FetchService, FetchTask, Request, Response};

pub struct LogoutForm {
    link: ComponentLink<Self>,
    req: Option<FetchTask>,
}

pub enum Msg {
    Submit(FocusEvent),
    Success,
    Error(String),
}

impl Component for LogoutForm {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { link, req: None }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Submit(e) => {
                e.prevent_default();
                self.req = Some(self.logout());
                false
            }
            Msg::Success => {
                reload_page();
                false
            }
            Msg::Error(err) => {
                ConsoleService::error(&err);
                reload_page();
                false
            }
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        let onsubmit = self.link.callback(Msg::Submit);

        html! {
            <form class="inline-block" onsubmit=onsubmit>
                <button type="submit">{"Log out"}</button>
            </form>
        }
    }
}

impl LogoutForm {
    fn logout(&self) -> FetchTask {
        let req = Request::post("/api/v1/logout")
            .header("Content-Type", "application/json")
            .header(csrf::HEADER_NAME, csrf::token())
            .body(Nothing)
            .expect("could not build request");

        let cb = self.link.callback(|res: Response<anyhow::Result<String>>| {
            if res.status().is_success() {
                Msg::Success
            } else {
                match res.into_body() {
                    Ok(body) => Msg::Error(body),
                    Err(err) => Msg::Error(err.to_string()),
                }
            }
        });

        FetchService::fetch(req, cb).expect("could not start request")
    }
}

fn reload_page() {
    let window = web_sys::window().expect("global `window`");
    let document = window.document().expect("`document` on `window`");
    let location = document.location().expect("`location` on `document`");
    location.reload().expect("`reload` on `location`")
}
