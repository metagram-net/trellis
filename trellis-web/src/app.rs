use serde_json;
use web_sys::HtmlTextAreaElement;
use yew::format::{Json, Nothing};
use yew::prelude::*;
use yew::services::fetch::{FetchService, FetchTask, Request, Response};

mod clock;
mod settings;
mod weather;

pub struct App {
    link: ComponentLink<Self>,
    settings: settings::Settings,
    state: State,
    textarea_ref: NodeRef,
}

pub enum Msg {
    FetchSettings,
    ReceiveSettings(Result<settings::Settings, anyhow::Error>),
    EditSettings,
    SaveSettings,
    SavedSettings,
}

enum State {
    Saving(FetchTask),
    Loading(FetchTask),
    Editing,
    Viewing,
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        link.send_message(Msg::FetchSettings);

        Self {
            link,
            settings: settings::Settings::load(),
            textarea_ref: NodeRef::default(),
            state: State::Viewing,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::FetchSettings => {
                self.state = State::Loading(self.fetch_settings());
                true
            }
            Msg::ReceiveSettings(maybe_settings) => {
                if let Ok(settings) = maybe_settings {
                    // TODO: Merge settings
                    self.settings = settings;
                    self.state = State::Viewing;
                    true
                } else {
                    // This could be someone who's not logged in, an application data error, or a
                    // broken network connection.
                    //
                    // TODO: Handle the error
                    false
                }
            }
            Msg::EditSettings => {
                self.textarea_ref = NodeRef::default();
                self.state = State::Editing;
                true
            }
            Msg::SaveSettings => {
                let text = self
                    .textarea_ref
                    .cast::<HtmlTextAreaElement>()
                    .unwrap()
                    .value();
                if let Ok(settings) = serde_json::from_str::<settings::Settings>(&text) {
                    if let Ok(_) = settings.save() {
                        self.state = State::Saving(self.send_settings(settings.clone()));
                        self.settings = settings;
                        return true;
                    }
                }
                self.state = State::Editing;
                true
            }
            Msg::SavedSettings => {
                self.state = State::Viewing;
                true
            }
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        match self.state {
            State::Loading(_) => self.view_loading(),
            State::Editing => self.view_settings(),
            State::Saving(_) => self.view_settings(),
            State::Viewing => self.view_board(),
        }
    }
}

impl App {
    fn view_loading(&self) -> Html {
        html! {
            <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4">
                <p class="text-xl">{"Loading..."}</p>
                </div>
        }
    }

    fn view_settings(&self) -> Html {
        let onclick = self.link.callback(|_| Msg::SaveSettings);
        let text =
            serde_json::to_string_pretty(&self.settings).expect("Could not serialize settings");
        let rows = format!(
            "{}",
            // Add an extra row because the last line doesn't have a newline.
            text.as_bytes().iter().filter(|&&c| c == b'\n').count() + 1
        );

        html! {
            <form>
                <textarea class="w-full" rows=rows ref=self.textarea_ref.clone()>
                {text}
            </textarea>
                <button onclick=onclick class="btn btn-gray">{ "Save" }</button>
                </form>
        }
    }

    fn view_board(&self) -> Html {
        let onclick = self.link.callback(|_| Msg::EditSettings);
        html! {
            <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4">
                <clock::Clock />
                <weather::Weather with self.settings.weather.clone() />
                <button onclick=onclick class="btn btn-gray">{ "Settings" }</button>
                </div>
        }
    }

    fn fetch_settings(&self) -> FetchTask {
        let request = Request::get("/api/v1/load")
            .body(Nothing)
            .expect("could not build request");
        let callback = self.link.callback(
            |response: Response<Json<anyhow::Result<settings::Settings>>>| {
                let Json(data) = response.into_body();
                Msg::ReceiveSettings(data)
            },
        );
        FetchService::fetch(request, callback).expect("could not start request")
    }

    fn send_settings(&self, settings: settings::Settings) -> FetchTask {
        let request = Request::post("/api/v1/save")
            .header("Content-Type", "application/json")
            .body(Json(&settings))
            .expect("could not build request");
        let callback = self
            .link
            .callback(|response: Response<anyhow::Result<String>>| {
                if response.status().is_success() {
                    Msg::SavedSettings
                } else {
                    // TODO: Try again?
                    // TODO: exponential backoff
                    Msg::SavedSettings
                }
            });
        FetchService::fetch(request, callback).expect("could not start request")
    }
}
