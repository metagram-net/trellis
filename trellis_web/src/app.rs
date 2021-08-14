use serde_json;
use trellis_core::config;
use uuid::Uuid;
use web_sys::HtmlTextAreaElement;
use yew::prelude::*;

mod clock;
mod note;
mod settings;
mod weather;

pub struct App {
    link: ComponentLink<Self>,
    settings: config::Config,
    state: State,
    textarea_ref: NodeRef,
    settings_service: Box<dyn Bridge<settings::Settings>>,
    error: Option<String>,
}

pub enum Msg {
    FetchSettings,
    ReceiveSettings(config::Config),
    EditSettings,
    SaveSettings,
    SaveSingle { id: Uuid, data: config::Data },
}

enum State {
    Loading,
    Editing,
    Viewing,
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let mut settings_service = settings::Settings::bridge(link.callback(Msg::ReceiveSettings));
        settings_service.send(settings::Request::Load);

        Self {
            link: link.clone(),
            settings: config::Config::default(),
            textarea_ref: NodeRef::default(),
            state: State::Loading,
            settings_service,
            error: None,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::FetchSettings => {
                self.settings_service.send(settings::Request::Load);
                self.state = State::Loading;
                true
            }
            Msg::ReceiveSettings(settings) => {
                self.settings = settings;
                self.state = State::Viewing;
                true
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
                match serde_json::from_str::<config::Config>(&text) {
                    Ok(settings) => {
                        self.settings_service
                            .send(settings::Request::Save(settings.clone()));
                        self.settings = settings;
                        self.state = State::Editing;
                        self.error = None;
                    }
                    Err(err) => {
                        self.error = Some(err.to_string());
                    }
                }
                true
            }
            Msg::SaveSingle { id, data } => {
                self.settings_service
                    .send(settings::Request::SaveSingle { id, data });
                true
            }
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        let main = match self.state {
            State::Loading => self.view_loading(),
            State::Editing => self.view_settings(),
            State::Viewing => self.view_board(),
        };
        html! {
            <div class="min-h-screen flex flex-col justify-between">
                <main class="m-1">{main}</main>
                <footer class="flex justify-center space-x-8 text-gray-400 text-sm">
                    <p>{crate::VERSION_INFO}</p>
                    <p>{"Source code on "}<a href="https://github.com/metagram-net/trellis">{"GitHub"}</a></p>
                </footer>
            </div>
        }
    }
}

impl App {
    fn view_loading(&self) -> Html {
        html! { <p class="text-xl">{"Loading..."}</p> }
    }

    fn view_settings(&self) -> Html {
        let onsubmit = self.link.callback(|e: FocusEvent| {
            e.prevent_default();
            Msg::SaveSettings
        });
        let text =
            serde_json::to_string_pretty(&self.settings).expect("Could not serialize settings");
        let rows = format!(
            "{}",
            // Add an extra row because the last line doesn't have a newline.
            text.as_bytes().iter().filter(|&&c| c == b'\n').count() + 1
        );

        let errors = match &self.error {
            None => html! {},
            Some(msg) => html! {
                <div class="alert">
                    <p>{msg}</p>
                </div>
            },
        };

        html! {
            <form onsubmit=onsubmit>
                <textarea class="w-full" rows=rows ref=self.textarea_ref.clone()>
                    {text}
                </textarea>
                <button type="submit" class="btn btn-gray">{ "Save" }</button>
                {errors}
            </form>
        }
    }

    fn view_board(&self) -> Html {
        let edit_settings = self.link.callback(|_| Msg::EditSettings);
        let reload_settings = self.link.callback(|_| Msg::FetchSettings);

        let tiles = self.settings.tiles.clone();
        let secrets = self.settings.secrets.clone();

        html! {
            <>
                <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4 space-x-1 space-y-1">
                    { tiles.iter().map(|t| self.render_tile(t.clone(), secrets.clone())).collect::<Html>() }
                    <div class="flex flex-col items-center justify-around w-full h-full">
                        <button type="button" onclick=edit_settings class="btn btn-gray">{ "Edit Settings" }</button>
                        <button type="button" onclick=reload_settings class="btn btn-gray">{ "Reload Settings" }</button>

                    </div>
                </div>
            </>
        }
    }

    fn render_tile(&self, tile: config::Tile, secrets: config::Secrets) -> Html {
        let id = tile.id.clone();
        match &tile.data {
            config::Data::Clock => html! { <clock::Clock id=id /> },
            config::Data::Weather { location_id } => {
                html! {
                    <weather::Weather
                        id=id
                        location_id=location_id.clone()
                        owm_api_key=secrets.owm_api_key.clone().unwrap_or("".to_owned())
                    />
                }
            }
            config::Data::Note { text } => html! {
                <note::Note
                    id=id
                    text=text.clone()
                    onchange=self.link.callback(move |text| {
                        Msg::SaveSingle{id, data: config::Data::Note{ text }}
                    })
                />
            },
        }
    }
}
