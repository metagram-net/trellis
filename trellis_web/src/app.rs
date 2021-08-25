use trellis_core::config;
use uuid::Uuid;
use yew::prelude::*;

mod add_tile_form;
mod board;
mod clock;
mod config_form;
mod footer;
mod grid;
mod note;
mod secrets_form;
mod settings;
mod weather;

pub struct App {
    link: ComponentLink<Self>,
    props: Props,
    settings: config::Config,
    state: State,
    settings_service: Box<dyn Bridge<settings::Settings>>,
}

#[derive(Properties, Clone, Debug)]
pub struct Props {
    pub version: &'static str,
    pub source_url: &'static str,
}

pub enum Msg {
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
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let mut settings_service = settings::Settings::bridge(link.callback(Msg::ReceiveSettings));
        settings_service.send(settings::Request::Load);

        Self {
            link,
            props,
            settings: config::Config::default(),
            state: State::Loading,
            settings_service,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
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

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props = props;
        true
    }

    fn view(&self) -> Html {
        let main = match self.state {
            State::Loading => html! { <p class="text-xl">{"Loading..."}</p> },
            State::Editing => {
                html! {
                    <config_form::ConfigForm
                        config=self.settings.clone()
                        onsubmit=self.link.callback(Msg::SaveSettings)
                    />
                }
            }
            State::Viewing => {
                let onedit = self.link.callback(|_| Msg::EditSettings);
                let onchange = self
                    .link
                    .callback(|(id, data)| Msg::SaveSingle { id, data });

                html! {
                    <board::Board config=self.settings.clone() onchange=onchange onedit=onedit />
                }
            }
        };
        html! {
            <div class="min-h-screen flex flex-col justify-between">
                <main class="m-1">{main}</main>
                <footer::Footer
                    version=self.props.version
                    source_url=self.props.source_url
                />
            </div>
        }
    }
}
