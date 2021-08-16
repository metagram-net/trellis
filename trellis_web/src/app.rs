use trellis_core::config;
use uuid::Uuid;
use yew::prelude::*;

mod clock;
mod config_form;
mod note;
mod settings;
mod weather;

pub struct App {
    link: ComponentLink<Self>,
    settings: config::Config,
    state: State,
    textarea_ref: NodeRef,
    settings_service: Box<dyn Bridge<settings::Settings>>,
}

pub enum Msg {
    FetchSettings,
    ReceiveSettings(config::Config),
    SaveSettings(config::Config),
    EditSettings,
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
            state: State::Loading,
            settings_service,
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
            Msg::SaveSettings(cfg) => {
                self.settings_service.send(settings::Request::Save(cfg));
                // Wait for the save to complete before re-rendering
                false
            }
            Msg::EditSettings => {
                self.state = State::Editing;
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
            State::Editing => {
                html! {
                    <config_form::ConfigForm
                        config=self.settings.clone()
                        onsave=self.link.callback(Msg::SaveSettings)
                    />
                }
            }
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
